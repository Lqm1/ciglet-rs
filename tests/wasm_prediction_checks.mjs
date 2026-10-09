function check(value,label) { if(!value) throw new Error(label); }
function throws(action,label) {let failed=false;try{action();}catch{failed=true;}check(failed,label);}
function same(a,b,label) {check(a.length===b.length && a.every((v,i)=>v===b[i]),label);}
function reader(raw,width,magic) {
    const bytes=new Uint8Array(raw),data=new DataView(bytes.buffer);let cursor=4;
    const integer=()=>{const v=data.getUint32(cursor,true);cursor+=4;return v;};
    const signed=()=>{const v=data.getInt32(cursor,true);cursor+=4;return v;};
    const double=()=>{const v=data.getFloat64(cursor,true);cursor+=8;return v;};
    const scalar=()=>{if(width===8)return double();const v=data.getFloat32(cursor,true);cursor+=4;return v;};
    check(String.fromCharCode(...bytes.slice(0,4))===magic && integer()===width,'fixture header');
    return {integer,signed,double,scalar,end:()=>check(cursor===bytes.length,'fixture fully consumed')};
}
export async function verifyPredictionBindings(api,fixture) {
    let waveformCases=0,spectrumCases=0,resonanceCases=0,stableCases=0,scaledCases=0,comparisons=0;
    const maximumSpectrumError=[new Array(5).fill(0),new Array(5).fill(0)],maximumStableError=[0,0],maximumScaledError=[[0,0],[0,0]];
    function measure(actual,expected,bound,label) {const error=Math.abs(actual-expected)/(1+Math.abs(expected));check(Number.isFinite(error)&&error<=bound,label+': '+actual+' vs '+expected+' error='+error);comparisons++;return error;}
    for(const width of [4,8]) {
        const index=width===4?0:1,T=width===4?Float32Array:Float64Array,suffix=width===4?'f32':'f64',Plan=api['PredictionSpectrumPlanF'+width*8];
        const functionOf=name=>api['prediction_'+name+'_'+suffix];
        const vector=(r,n)=>new T(Array.from({length:n},r.scalar));
        const waveform=reader(await fixture('prediction-'+(width===4?'float':'double')+'.bin'),width,'LPC1'),waveformCount=waveform.integer();
        for(let c=0;c<waveformCount;c++) {
            waveformCases++;const length=waveform.integer(),order=waveform.integer(),samples=vector(waveform,length),correlation=vector(waveform,order+1),coefficients=vector(waveform,order+1),gain=waveform.scalar();
            const legacy=functionOf('from_samples_with')(samples,order,1);same(legacy.autocorrelation,correlation,'exact C waveform correlation');same(legacy.coefficients,coefficients,'exact C waveform coefficients');comparisons+=2*(order+1);
            same(functionOf('levinson_with')(correlation,1),coefficients,'source recursion');check(functionOf('gain_with')(coefficients,correlation,1)===gain,'exact C waveform gain');comparisons++;
            const stable=functionOf('from_samples')(samples,order),stableCorrelation=stable.autocorrelation,stableCoefficients=stable.coefficients,bound=width===4?2e-7:5e-15;
            stableCorrelation.forEach((v,i)=>measure(v,correlation[i],bound,'Stable correlation'));
            if(stableCorrelation[0]>0) for(let row=1;row<=order;row++) {
                let residual=stableCorrelation[row]/stableCorrelation[0],scale=1+Math.abs(residual);
                for(let column=1;column<=order;column++) {const covariance=row===column?1.000001:stableCorrelation[Math.abs(row-column)]/stableCorrelation[0],term=covariance*stableCoefficients[column];residual+=term;scale+=Math.abs(term);}
                check(Math.abs(residual)/scale<=bound,'independent regularized Toeplitz equation');
            }
            const selected=functionOf('from_samples_with')(samples,order,0);same(selected.coefficients,stableCoefficients,'default arithmetic');same(selected.autocorrelation,stableCorrelation,'default correlation');selected.free();
            check(Number.isFinite(functionOf('gain')(stableCoefficients,stableCorrelation)),'stable gain');legacy.free();stable.free();
        }
        waveform.end();
        const spectrum=reader(await fixture('prediction-spectrum-'+(width===4?'float':'double')+'.bin'),width,'LPS1'),spectrumCount=spectrum.integer(),bounds=width===4?[2e-8,3e-8,0,7e-8,8e-8]:[5e-17,2e-15,4e-16,2e-15,3e-15];
        for(let c=0;c<spectrumCount;c++) {
            spectrumCases++;const bins=spectrum.integer(),length=spectrum.integer(),order=spectrum.integer(),input=vector(spectrum,bins),correlation=vector(spectrum,order+1),coefficients=vector(spectrum,order+1),gain=spectrum.scalar(),expectedSpectrum=vector(spectrum,length),plan=new Plan(length);
            check(plan.length===length && !plan.is_empty(),'plan metadata');
            const actual=plan.estimate_with_arithmetic(input,order,1,1),actualCorrelation=actual.autocorrelation,actualCoefficients=actual.coefficients,actualGain=functionOf('gain_with')(actualCoefficients,actualCorrelation,1);
            const outputs=[actualCorrelation,actualCoefficients,[actualGain],plan.spectrum(coefficients,gain),plan.spectrum(actualCoefficients,actualGain)],expected=[correlation,coefficients,[gain],expectedSpectrum,expectedSpectrum];
            outputs.forEach((values,field)=>values.forEach((v,i)=>maximumSpectrumError[index][field]=Math.max(maximumSpectrumError[index][field],measure(v,expected[field][i],bounds[field],'LPS1 field '+field))));
            const one=functionOf('from_magnitude_with_arithmetic')(input,order,1,1),reused=plan.estimate_with_arithmetic(input,order,1,1);same(one.coefficients,actualCoefficients,'one-shot magnitude');same(one.autocorrelation,actualCorrelation,'one-shot lag');same(reused.coefficients,actualCoefficients,'plan reuse');one.free();reused.free();
            same(functionOf('levinson_with')(correlation,1),coefficients,'source spectral recursion');
            const stable=plan.estimate(input,order,1);same(stable.autocorrelation,actualCorrelation,'stable spectral lag');same(stable.coefficients,functionOf('levinson')(stable.autocorrelation),'stable spectral recursion');
            check(plan.spectrum(stable.coefficients,functionOf('gain')(stable.coefficients,stable.autocorrelation)).every(Number.isFinite),'stable finite spectrum');stable.free();actual.free();plan.free();
        }
        const resonanceCount=spectrum.integer();
        for(let c=0;c<resonanceCount;c++) {
            resonanceCases++;const length=spectrum.integer(),coefficients=vector(spectrum,length),count=spectrum.integer(),frequencies=vector(spectrum,count),expectedPoles=vector(spectrum,2*count),report=functionOf('resonances')(coefficients),values=report.resonances;
            check(values.length===3*count,'complete resonance result');for(let i=0;i<count;i++) {
                check(values[3*i]===frequencies[i],'exact C resonance frequency');check(values[3*i+1]===expectedPoles[2*i] && values[3*i+2]===expectedPoles[2*i+1],'exact C resonance pole');comparisons+=3;
                const owner=report.resonance(i);same(owner.components(),values.slice(3*i,3*i+3),'standalone resonance');owner.free();
            }
            check(report.resonance(count)===undefined,'missing resonance');check(report.iterations<=50 && Number.isFinite(report.maximum_residual) && report.converged===(report.maximum_residual<1e-6),'complete convergence report');report.free();
        }
        spectrum.end();
        const stable=reader(await fixture('stable-prediction-'+(width===4?'float':'double')+'.bin'),width,'SLP1');check(stable.integer()===100,'complete stable corpus');
        for(let c=0;c<100;c++) {stableCases++;const length=stable.integer(),correlation=vector(stable,length),expected=vector(stable,length),actual=functionOf('levinson')(correlation);actual.forEach((v,i)=>maximumStableError[index]=Math.max(maximumStableError[index],measure(v,expected[i],width===4?2e-15:6e-11,'stable high precision reference')));}
        stable.end();
        const scaled=reader(await fixture('scaled-waveform-'+(width===4?'float':'double')+'.bin'),width,'SWV1');check(scaled.integer()===64,'complete scaled corpus');
        for(let c=0;c<64;c++) {
            scaledCases++;const length=scaled.integer(),order=scaled.integer(),samples=vector(scaled,length),model=functionOf('from_samples_scaled')(samples,order),parts=model.autocorrelation_parts();check(parts.length===order+1,'complete scaled lags');
            const fractions=parts.fractions,exponents=parts.exponents;
            for(let i=0;i<=order;i++) {
                const expected=scaled.double(),exponent=scaled.signed(),error=expected===0?Math.abs(fractions[i]):Math.abs(fractions[i]*2**(exponents[i]-exponent)/expected-1);
                check(error<=2e-15 && (expected!==0 || fractions[i]===0),'scaled correlation bound');maximumScaledError[index][0]=Math.max(maximumScaledError[index][0],error);comparisons++;
            }
            const expected=vector(scaled,order+1);model.coefficients.forEach((v,i)=>maximumScaledError[index][1]=Math.max(maximumScaledError[index][1],measure(v,expected[i],width===4?2e-15:5e-15,'scaled coefficient')));
            const clone=model.cloned(),cloneParts=clone.autocorrelation_parts();same(clone.coefficients,model.coefficients,'scaled clone');same(cloneParts.fractions,fractions,'scaled clone significands');same(cloneParts.exponents,exponents,'scaled clone exponents');cloneParts.free();clone.free();parts.free();model.free();
        }
        scaled.end();
        const maximum=width===4?3.4028234663852886e38:Number.MAX_VALUE,tiny=width===4?2**-149:Number.MIN_VALUE;
        for(const amplitude of [maximum,tiny]) {
            const model=functionOf('from_samples_scaled')(new T([amplitude,0]),1),parts=model.autocorrelation_parts();same(model.coefficients,new T([1,-0]),'extreme coefficients');check(model.gain()===amplitude,'extreme representable gain');check(parts.fractions[1]===0 && parts.exponents[1]===0,'zero lag representation');
            const gain=model.gain_parts();check(gain.fraction>=1 && gain.fraction<2,'scaled gain normalized');gain.free();parts.free();throws(()=>model.cloned().into_prediction(),'extreme correlation conversion rejected');model.free();
        }
        const plan=new Plan(16),constant=plan.spectrum(new T([maximum]),maximum);check(constant.every(v=>v===1),'scaled spectrum large ratio');check(plan.spectrum(new T([tiny]),tiny).every(v=>v===1),'scaled spectrum tiny ratio');
        for(const code of [2,0xffffffff]) {throws(()=>plan.estimate(new T(9).fill(1),1,code),'invalid Nyquist');throws(()=>plan.estimate_with_arithmetic(new T(9).fill(1),1,0,code),'invalid arithmetic');check(plan.length===16,'owner released after code error');}
        const scaledMagnitude=plan.estimate_scaled(new T(9).fill(1),1,0),oneScaled=functionOf('from_magnitude_scaled')(new T(9).fill(1),1,0);same(scaledMagnitude.coefficients,oneScaled.coefficients,'scaled magnitude entrypoints');oneScaled.free();const ordinary=scaledMagnitude.into_prediction();same(ordinary.coefficients,new T([1,-0]),'consuming scaled conversion');ordinary.free();plan.free();
    }
    verifyPredictionEdges(api);
    return {waveformCases,spectrumCases,resonanceCases,stableCases,scaledCases,comparisons,maximumSpectrumError,maximumStableError,maximumScaledError};
}

function verifyPredictionEdges(api) {
    for(const width of [4,8]) {
        const T=width===4?Float32Array:Float64Array,suffix=width===4?'f32':'f64',bits=width===4?Uint32Array:BigUint64Array;
        const fn=name=>api['prediction_'+name+'_'+suffix],Plan=api['PredictionSpectrumPlanF'+width*8];
        const maximum=width===4?3.4028234663852886e38:Number.MAX_VALUE,tiny=width===4?2**-149:Number.MIN_VALUE;
        const raw=new bits(width===4?[0x80000000,0x7fc00123,1,0x7f800000]:[0x8000000000000000n,0x7ff8000000000123n,1n,0x7ff0000000000000n]);
        const payload=new T(raw.buffer),equalBits=(a,b,label)=>same(new bits(a.buffer,a.byteOffset,a.length),new bits(b.buffer,b.byteOffset,b.length),label);
        const Ordinary=api['PredictionF'+width*8],Resonance=api['PredictionResonanceF'+width*8],Estimate=api['PredictionResonanceEstimateF'+width*8];
        const ordinary=new Ordinary(payload,new T([7])),clone=ordinary.cloned(),snapshot=ordinary.coefficients;
        equalBits(snapshot,payload,'ordinary complete IEEE payload');ordinary.coefficients=new T();ordinary.autocorrelation=payload;
        check(ordinary.coefficients.length===0,'arbitrary empty public coefficients');equalBits(ordinary.autocorrelation,payload,'arbitrary public lag storage');
        equalBits(clone.coefficients,payload,'ordinary clone independence');same(clone.autocorrelation,new T([7]),'mismatched clone storage');ordinary.free();clone.free();equalBits(snapshot,payload,'ordinary snapshot lifetime');
        const resonance=Resonance.from_components(payload.slice(0,3)),resonanceClone=resonance.cloned(),components=resonance.components();
        equalBits(components,payload.slice(0,3),'resonance complete IEEE fields');throws(()=>resonance.set_components(payload),'atomic resonance shape');equalBits(resonance.components(),components,'unchanged resonance on error');
        throws(()=>resonance.set_pole(new T([1])),'atomic pole shape');equalBits(resonance.components(),components,'unchanged pole on error');
        resonance.set_pole(payload.slice(1,3));equalBits(resonance.pole(),payload.slice(1,3),'pole payload copy');resonance.frequency_nyquist=0.25;check(resonance.frequency_nyquist===0.25,'frequency mutation');
        resonance.set_components(new T([0.5,2,3]));same(resonance.components(),new T([0.5,2,3]),'full resonance replacement');resonance.free();resonanceClone.free();equalBits(components,payload.slice(0,3),'resonance snapshot lifetime');
        const triples=new T([...payload.slice(0,3),...payload.slice(0,3)]),report=new Estimate(triples,0xffffffff,false,-0),reportClone=report.cloned(),reportSnapshot=report.resonances;
        equalBits(reportSnapshot,triples,'all raw report triples');check(report.iterations===0xffffffff && !report.converged && Object.is(report.maximum_residual,-0),'all report metadata');
        throws(()=>{report.resonances=payload;},'atomic triples shape');equalBits(report.resonances,triples,'report retained on error');throws(()=>report.set_residual_storage(new Float64Array()),'atomic residual shape');check(Object.is(report.maximum_residual,-0),'residual retained on error');
        const residualBits=new BigUint64Array([0x7ff8000000000432n]),residual=new Float64Array(residualBits.buffer);report.set_residual_storage(residual);
        same(new BigUint64Array(report.residual_storage().buffer),residualBits,'residual binary64 payload');report.iterations=7;report.converged=true;report.maximum_residual=0.5;
        check(report.iterations===7 && report.converged && report.maximum_residual===0.5,'all mutable report metadata');report.resonances=new T();check(report.resonance(0)===undefined,'empty report index');
        equalBits(reportClone.resonances,triples,'report clone independence');check(reportClone.iterations===0xffffffff && !reportClone.converged && Object.is(reportClone.maximum_residual,-0),'cloned metadata independence');report.free();reportClone.free();equalBits(reportSnapshot,triples,'report snapshot lifetime');
        throws(()=>new Estimate(new T([1]),0,true,0),'constructor triples shape');throws(()=>Resonance.from_components(new T()),'constructor components shape');
        const plan=new Plan(16),ones=new T(9).fill(1),silent=plan.estimate(new T(9),7,0);same(silent.coefficients,new T([1,0,0,0,0,0,0,0]),'silent prediction');silent.free();
        const defaultModel=fn('from_magnitude')(new T([0,0,2]),0),legacyModel=fn('from_magnitude_with')(new T([0,0,2]),0,1);
        same(defaultModel.autocorrelation,new T([1]),'default preserves Nyquist');same(legacyModel.autocorrelation,new T([0]),'legacy discards Nyquist');defaultModel.free();legacyModel.free();
        for(const arithmetic of [0,1]) for(const policy of [0,1]) {
            const input=new T([1,2,3,4,5,6,7,8,9]),copy=input.slice(),estimate=plan.estimate_with_arithmetic(input,3,policy,arithmetic),one=fn('from_magnitude_with_arithmetic')(input,3,policy,arithmetic);
            same(estimate.coefficients,one.coefficients,'all policy combinations');same(estimate.autocorrelation,one.autocorrelation,'all policy lags');same(input,copy,'input remains immutable');estimate.free();one.free();
        }
        const ignored=fn('from_magnitude_with')(new T([1,1,NaN]),1,1);same(ignored.autocorrelation,new T([1,0]),'ignored nonfinite Nyquist');ignored.free();
        throws(()=>fn('from_magnitude')(new T([1,1,NaN]),1),'preserved nonfinite Nyquist');throws(()=>plan.estimate(new T([1,1,NaN]),1,1),'short input last value not Nyquist');
        const positive=plan.spectrum(new T([1,-0.5]),2),negative=plan.spectrum(new T([1,-0.5]),-2);same(negative,new T(Array.from(positive,v=>-v)),'signed spectrum');
        if(width===8) positive.forEach((v,i)=>check(Math.abs(v-2/Math.sqrt(1.25-Math.cos(2*Math.PI*i/16)))<3e-15,'independent analytic spectrum'));
        check(plan.spectrum(new T([1,-1]),0).every(v=>v===0),'zero gain at pole');check(plan.spectrum(new T([1,-1]),-0).every(v=>Object.is(v,-0)),'negative zero gain');
        for(const coefficients of [new T(),new T([0]),new T(17).fill(1),new T([NaN]),new T([1,-1])]) throws(()=>plan.spectrum(coefficients,1),'invalid spectrum input');
        throws(()=>plan.spectrum(new T([1]),NaN),'nonfinite gain');
        for(const sign of [1,-1]) plan.spectrum(new T([maximum,maximum*0.5]),sign*maximum).forEach((v,i)=>check(Math.abs(v/(sign/Math.sqrt(1.25+Math.cos(2*Math.PI*i/16)))-1)<=(width===4?2e-7:4e-15),'overflowing FFT ratio'));
        const cancelled=plan.spectrum(new T([maximum,1,-maximum]),1);check(cancelled[0]===1 && cancelled[8]===1,'small term survives spectral cancellation');
        const singleton=new Plan(1);same(singleton.spectrum(new T([1]),2),new T([2]),'singleton spectrum');singleton.free();
        for(const length of [0,3]) throws(()=>new Plan(length),'invalid plan size');
        for(const action of [()=>fn('levinson')(new T()),()=>fn('levinson')(new T([0,1])),()=>fn('levinson')(new T([-1])),()=>fn('gain')(new T([1]),new T()),()=>fn('gain')(new T([1]),new T([-1])),()=>fn('from_samples')(new T(),0),()=>fn('from_samples_scaled')(new T([NaN]),0),()=>fn('from_magnitude')(new T([1]),0),()=>plan.estimate(ones,16,0),()=>plan.estimate(new T(9).fill(maximum),1,0),()=>fn('resonances')(new T())]) throws(action,'invalid prediction operation');
        for(const code of [2,0xffffffff]) for(const action of [()=>fn('levinson_with')(new T([1]),code),()=>fn('gain_with')(new T([1]),new T([1]),code),()=>fn('from_samples_with')(new T([1]),0,code),()=>fn('from_magnitude_with')(ones,1,code),()=>fn('from_magnitude_with_arithmetic')(ones,1,0,code),()=>fn('from_magnitude_scaled')(ones,1,code),()=>plan.estimate_scaled(ones,1,code)]) throws(action,'invalid numeric policy');
        const recovered=plan.estimate(ones,1,0);same(recovered.coefficients,new T([1,-0]),'plan recovery after all errors');recovered.free();plan.free();
        for(const amplitude of [maximum,tiny]) {
            const smallPlan=new Plan(4),scaled=smallPlan.estimate_scaled(new T(3).fill(amplitude),3,0),parts=scaled.autocorrelation_parts(),fractionSnapshot=parts.fractions,exponentSnapshot=parts.exponents;
            same(scaled.coefficients,new T([1,0,0,0]),'extreme scaled magnitude');check(fractionSnapshot.slice(1).every(v=>v===0) && exponentSnapshot.slice(1).every(v=>v===0),'all exact zero scaled lags');check(scaled.gain()===amplitude,'extreme scaled spectral gain');
            const consumed=scaled.cloned();throws(()=>consumed.into_prediction(),'unrepresentable scaled conversion');throws(()=>consumed.gain(),'failed conversion consumes owner');check(scaled.gain()===amplitude,'consuming clone leaves source usable');
            const discarded=smallPlan.estimate_scaled(new T([amplitude,amplitude,NaN]),3,1);check(discarded.gain()===amplitude,'scaled ignored Nyquist');discarded.free();
            throws(()=>smallPlan.estimate_scaled(new T(3).fill(amplitude),4,0),'scaled order bounds');throws(()=>smallPlan.estimate_scaled(new T(3).fill(NaN),1,0),'scaled nonfinite validation');
            parts.free();scaled.free();smallPlan.free();check(fractionSnapshot[0]>=1 && exponentSnapshot.length===4,'copied iterator storage lifetime');
        }
        const convertible=fn('from_samples_scaled')(new T([1,0]),1),converted=convertible.into_prediction();throws(()=>convertible.gain(),'successful conversion consumes owner');same(converted.autocorrelation,new T([1,0]),'ordinary converted lags');converted.free();
        const silence=fn('from_samples_scaled')(new T(4),3),zeroGain=silence.gain_parts();check(zeroGain.fraction===0 && zeroGain.exponent===0,'silent gain parts');zeroGain.fraction=1.5;zeroGain.exponent=-7;check(zeroGain.fraction_storage()[0]===1.5 && zeroGain.exponent===-7,'mutable gain parts');zeroGain.free();silence.free();
        const analytic=fn('resonances')(new T([1,0,0,0,0.25]));check(analytic.converged && analytic.resonances.length===6,'analytic resonance convergence');check(Math.abs(analytic.resonances[0]-0.25)<1e-6 && Math.abs(analytic.resonances[3]-0.75)<1e-6,'analytic resonance frequencies');analytic.free();const constant=fn('resonances')(new T([1]));check(constant.resonances.length===0,'constant has no resonance');constant.free();
        if(width===8) {
            const cancelledWave=fn('from_samples_scaled')(new T([maximum,1,-maximum]),2),parts=cancelledWave.autocorrelation_parts(),gain=cancelledWave.gain_parts();check(parts.fractions[1]===0 && parts.exponents[1]===0,'cancelled waveform lag');check(gain.fraction>=1 && gain.fraction<2 && gain.exponent===1024,'unrepresentable scaled gain retained');throws(()=>cancelledWave.gain(),'unrepresentable ordinary gain rejected');gain.free();parts.free();cancelledWave.free();
            for(const amplitude of [maximum,tiny]) {
                const model=fn('from_magnitude_scaled')(new T([0,amplitude,0,0,0]),3,0),parts=model.autocorrelation_parts(),fractions=parts.fractions,exponents=parts.exponents;
                [1,Math.SQRT1_2,0,-Math.SQRT1_2].forEach((expected,i)=>{const ratio=fractions[i]===0?0:fractions[i]/fractions[0]*2**(exponents[i]-exponents[0]);check(Math.abs(ratio-expected)<4e-15,'independent extreme frequency-grid correlations');});
                const gain=model.gain_parts();check(gain.fraction>=1 && gain.fraction<2,'frequency-grid gain normalization');if(amplitude===tiny){check(gain.exponent<-1074,'subnormal gain exponent');throws(()=>model.gain(),'subnormal gain rejected');}else check(model.gain()>0,'large gain representable');gain.free();parts.free();model.free();
            }
            const amplitude=Math.sqrt(maximum)*0.5,largePlan=new Plan(16),large=largePlan.estimate(new T(9).fill(amplitude),3,0);check(large.coefficients.slice(1).every(v=>Math.abs(v)<2e-15),'inverse FFT overflow coefficients');check(Math.abs(fn('gain')(large.coefficients,large.autocorrelation)/amplitude-1)<2e-15,'inverse FFT overflow gain');large.free();largePlan.free();
        } else {
            const magnitudes=new T(33);magnitudes[8]=Math.sqrt(2**-126);const subnormalPlan=new Plan(64),legacy=subnormalPlan.estimate_with_arithmetic(magnitudes,1,0,1),stable=subnormalPlan.estimate_scaled(magnitudes,1,0),expected=-Math.SQRT1_2/1.000001;
            check(Math.abs(legacy.coefficients[1]-expected)>2e-7 && Math.abs(stable.coefficients[1]-expected)<3e-8,'scaled estimation fixes subnormal FFT energy');legacy.free();stable.free();subnormalPlan.free();
        }
    }
}
