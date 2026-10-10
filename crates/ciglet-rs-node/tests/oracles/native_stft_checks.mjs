function check(v, label) { if(!v) throw new Error(label); }
function throws(action,label) { let failed=false;try {action();}catch{failed=true;}check(failed,label); }
function same(a,b,label) {check(a.length===b.length && a.every((v,i)=>Object.is(v,b[i]) || v===b[i]),label);}
function bits(a,b,label) {same(new Uint8Array(a.buffer,a.byteOffset,a.byteLength),new Uint8Array(b.buffer,b.byteOffset,b.byteLength),label);}
function reader(raw,width,magic,count) {
    const bytes=new Uint8Array(raw), data=new DataView(bytes.buffer);let cursor=4;
    check(String.fromCharCode(...bytes.slice(0,4))===magic,'magic');
    const integer=()=>{const n=data.getUint32(cursor,true);cursor+=4;return n;};
    const scalar=()=>{const n=width===4?data.getFloat32(cursor,true):data.getFloat64(cursor,true);cursor+=width;return n;};
    check(integer()===width && integer()===count,'complete fixture header');
    return {integer,scalar,end:()=>check(cursor===bytes.length,'fully consumed fixture')};
}
export async function verifyStftBindings(api, fixture) {
    let analysisCases=0,synthesisCases=0,comparisons=0,skippedPhase=0;
    const maximumMagnitude=[0,0],maximumWeightedPhase=[0,0],maximumSynthesis=[0,0];
    for(const width of [4,8]) {
        const index=width===4?0:1,T=width===4?Float32Array:Float64Array,suffix=width===4?'f32':'f64';
        const Plan=api['StftPlanF'+width*8],Result=api['StftAnalysisF'+width*8],Synthesis=api['StftSynthesisOptionsF'+width*8];
        const options=new api.StftAnalysisOptions();
        check(!options.subtract_mean && options.magnitude && options.phase && options.normalization_hop===null,'native defaults');
        const centers=new Int32Array([-3,5,13,21]),r=reader(await fixture('c-stft-'+suffix+'.bin'),width,'STF1',480);
        for(let record=0;record<480;record++) {
            analysisCases++;const window=r.integer(),precision=r.integer(),subtract=r.integer()!==0,variable=r.integer()!==0,scenario=r.integer(),mask=r.integer();
            const norm=r.scalar(),weight=r.scalar(),signal=new T(27);
            signal.forEach((_,i)=>signal[i]=scenario===0?(i===11?.75:0):scenario===1?(i%9-4)*.125:2);
            const before=signal.slice(),lengths=new Uint32Array(variable?[8,12,16,20]:[16,16,16,16]);
            options.subtract_mean=subtract;options.magnitude=(mask&1)!==0;options.phase=(mask&2)!==0;options.normalization_hop=variable?undefined:8;
            const plan=new Plan(32,window,precision),result=plan.analyze(signal,centers,lengths,options);
            const completeOptions=options.cloned();completeOptions.magnitude=true;completeOptions.phase=true;
            const full=plan.analyze(signal,centers,lengths,completeOptions),fullMagnitude=full.magnitude,fullPhase=full.phase;
            const magnitudes=result.magnitude,phases=result.phase;
            check(result.frames===4 && result.bins===17,'analysis metadata');
            check((magnitudes!==null)===((mask&1)!==0) && (phases!==null)===((mask&2)!==0),'optional output arrays');
            if(!variable) {check(Math.abs(result.window_weight-weight)<1e-14 && Math.abs(result.normalization-norm)<1e-14,'native normalization metadata');}
            else check(result.normalization===null,'no inferred variable hop');
            for(let i=0;i<68;i++) {
                if(mask&1) {
                    const expected=r.scalar(),error=Math.abs(magnitudes[i]-expected)/Math.max(1,Math.abs(expected));
                    maximumMagnitude[index]=Math.max(maximumMagnitude[index],error);check(error<(width===4?2e-6:2e-14),'native C magnitude bound');check(magnitudes[i]===fullMagnitude[i],'output mask magnitude');comparisons++;
                }
                if(mask&2) {
                    const expected=r.scalar();check(phases[i]===fullPhase[i],'output mask phase');
                    if(fullMagnitude[i]<1e-6) {skippedPhase++;continue;}
                    const begin=Math.floor(i/17)*17,scale=Math.max(1,...fullMagnitude.slice(begin,begin+17));
                    const error=2*Math.abs(Math.sin((phases[i]-expected)*.5))*fullMagnitude[i]/scale;
                    maximumWeightedPhase[index]=Math.max(maximumWeightedPhase[index],error);check(error<(width===4?2e-6:2e-14),'native C weighted phase bound');comparisons++;
                }
            }
            const copy=result.cloned();check(copy.frames===4 && copy.bins===17 && copy.window_weight===result.window_weight && copy.normalization===result.normalization,'full clone metadata');
            if(magnitudes) same(copy.magnitude,magnitudes,'clone magnitudes');if(phases) same(copy.phase,phases,'clone phases');
            bits(signal,before,'input immutable');result.free();full.free();copy.free();completeOptions.free();plan.free();
        }
        r.end();
        const inverse=reader(await fixture('c-istft-'+suffix+'.bin'),width,'IST1',288);
        for(let record=0;record<288;record++) {
            synthesisCases++;const length=inverse.integer(),frames=inverse.integer(),fade=inverse.integer(),offset=inverse.integer()|0;inverse.integer();const normalization=inverse.scalar();
            const count=frames*(length/2+1),magnitude=new T(count),phase=new T(count);
            for(let i=0;i<count;i++) {magnitude[i]=inverse.scalar();phase[i]=inverse.scalar();}
            const beforeMagnitude=magnitude.slice(),beforePhase=phase.slice(),plan=new Plan(length,3,0),synthesis=new Synthesis(4,offset,fade,normalization,1);
            const output=plan.synthesize(magnitude,phase,synthesis),copy=synthesis.cloned();same(plan.synthesize(magnitude,phase,copy),output,'reuse inverse');
            check(output.length===inverse.integer(),'complete output length');
            for(let i=0;i<output.length;i++) {const expected=inverse.scalar(),error=Math.abs(output[i]-expected)/Math.max(1,Math.abs(expected));maximumSynthesis[index]=Math.max(maximumSynthesis[index],error);check(error<(width===4?2e-7:2e-15),'native inverse C bound');comparisons++;}
            bits(magnitude,beforeMagnitude,'inverse magnitudes immutable');bits(phase,beforePhase,'inverse phases immutable');plan.free();synthesis.free();copy.free();
        }
        inverse.end();options.subtract_mean=false;options.magnitude=true;options.phase=true;options.normalization_hop=undefined;
        const plan=new Plan(8,0,0),odd=plan.analyze(new T([1,2,3]),new Int32Array([1]),new Uint32Array([3]),options);
        check(odd.magnitude[0]===6 && odd.window_weight===3,'odd window retains final sample');odd.free();
        const padded=plan.analyze(new T([1,2,3]),new Int32Array([-1]),new Uint32Array([3]),options);check(padded.magnitude[0]===1,'zero padding');padded.free();
        const variablePlan=new Plan(8,1,0);options.normalization_hop=2;
        const variableResult=variablePlan.analyze(new T(12).fill(1),new Int32Array([4,8]),new Uint32Array([4,8]),options);
        const weights=api['window_'+suffix](1,8,0),sum=weights.reduce((a,b)=>width===4?Math.fround(a+b):a+b,0);
        check(Math.abs(variableResult.magnitude[5]-sum)<1e-14,'each variable frame has own weights');variableResult.free();variablePlan.free();options.normalization_hop=undefined;
        for(const [signal,c,l] of [[[],[0],[]],[[],[0],[0]],[[],[0],[9]],[[NaN],[0],[4]]]) throws(()=>plan.analyze(new T(signal),new Int32Array(c),new Uint32Array(l),options),'analysis shape/input failure');
        options.normalization_hop=0;throws(()=>plan.analyze(new T(),new Int32Array([0]),new Uint32Array([4]),options),'zero hop');options.normalization_hop=undefined;
        const empty=plan.analyze(new T(),new Int32Array(),new Uint32Array(),options);check(empty.frames===0 && empty.bins===5 && empty.magnitude.length===0 && empty.phase.length===0 && empty.window_weight===null && empty.normalization===null,'empty full analysis');empty.free();
        const synthesis=new Synthesis(8,4,0,1,0),magnitude=new T([0,0,0,0,8]),phase=new T(5),output=plan.synthesize(magnitude,phase,synthesis),amplitude=api.approximation_cosine(0);
        for(let i=0;i<8;i++) check(output[i]===(i%2===0?amplitude:-amplitude),'independent preserved Nyquist');same(output.slice(8),new T(4),'output trailing zeros');
        synthesis.nyquist=1;check(plan.synthesize(magnitude,phase,synthesis).every(v=>v===0),'Legacy Nyquist overwrite');synthesis.nyquist=0;
        for(const [field,bad,good] of [['hop',0,8],['hop',0xffffffff,8],['fade_samples',9,0],['offset',-9,4],['normalization',0,1],['normalization',NaN,1],['nyquist',2,0]]) {
            synthesis[field]=bad;throws(()=>plan.synthesize(magnitude,phase,synthesis),'inverse options rejected');synthesis[field]=good;same(plan.synthesize(magnitude,phase,synthesis),output,'inverse post-error recovery');
        }
        throws(()=>plan.synthesize(magnitude,phase.slice(1),synthesis),'spectral shape');throws(()=>plan.synthesize(magnitude,new T([NaN,0,0,0,0]),synthesis),'invalid spectra');
        synthesis.offset=0;check(plan.synthesize(new T(),new T(),synthesis).length===0,'empty inverse');
        for(const field of ['window_weight','normalization']) {
            const words=width===4?new Uint32Array([0x7f812345]):new BigUint64Array([0x7ff0000000001234n]);const raw=new T(words.buffer);
            const arbitrary=new Result(0xffffffff,0xffffffff,raw,undefined,undefined,undefined);arbitrary['set_'+field+'_storage'](raw);bits(arbitrary[field+'_storage'](),raw,'optional scalar payload');
            const clone=arbitrary.cloned();arbitrary.frames=0;arbitrary.bins=1;arbitrary.magnitude=undefined;arbitrary.phase=raw;
            check(clone.frames===0xffffffff && clone.bins===0xffffffff && clone.phase===null,'full result metadata clone');bits(clone.magnitude,raw,'clone arbitrary arrays');
            for(const n of [0,2]) {throws(()=>clone['set_'+field+'_storage'](new T(n)),'optional storage shape');bits(clone[field+'_storage'](),raw,'atomic scalar error');}
            clone['set_'+field+'_storage'](undefined);check(clone[field]===null,'optional scalar cleared');clone[field]=-0;check(Object.is(clone[field],-0),'scalar setter signed zero');
            const snapshot=arbitrary.phase;arbitrary.free();clone.free();bits(snapshot,raw,'result snapshot lifetime');
            synthesis.set_normalization_storage(raw);bits(synthesis.normalization_storage(),raw,'synthesis typed scalar payload');
            throws(()=>synthesis.set_normalization_storage(new T()),'synthesis normalization shape');bits(synthesis.normalization_storage(),raw,'synthesis scalar error atomic');synthesis.normalization=1;
        }
        synthesis.free();plan.free();
        for(const window of [0,1,2,3,4,5,6]) for(const precision of [0,1]) {const p=new Plan(8,window,precision),a=p.analyze(new T([1]),new Int32Array([0]),new Uint32Array([4]),options);check(a.frames===1,'all windows and precisions');a.free();p.free();}
        for(const [length,w,p] of [[0,0,0],[3,0,0],[8,7,0],[8,0,2]]) throws(()=>new Plan(length,w,p),'invalid plan');
        const singleton=new Plan(1,0,0),singleOptions=new Synthesis(1,0,0,1,0);check(singleton.synthesize(new T([1]),new T([0]),singleOptions).length===1,'singleton inverse');singleOptions.nyquist=1;throws(()=>singleton.synthesize(new T([1]),new T([0]),singleOptions),'singleton legacy rejected');singleOptions.free();singleton.free();
        options.free();
    }
    const grid=new api.StftFrameGrid(8,24,8,2),options=new api.StftAnalysisOptions(),signal=new Float64Array(192);
    signal.forEach((_,i)=>signal[i]=Math.sin(2*Math.PI*i/32));
    const regular=api.stft_analyze_regular_f64(signal,grid,0,options),approximate=api.stft_analyze_regular_f64(signal,grid,1,options),explicit=new api.StftPlanF64(128,3,0);
    options.normalization_hop=8;const centers=new Int32Array(Array.from({length:24},(_,i)=>i*8)),lengths=new Uint32Array(24).fill(64),analysis=explicit.analyze(signal,centers,lengths,options);
    same(regular.magnitude,analysis.magnitude,'regular/explicit magnitude');same(regular.phase,analysis.phase,'regular/explicit phase');same(regular.magnitude,approximate.magnitude,'regular precision levels');check(regular.normalization===analysis.normalization,'regular normalization');
    const reconstructed=api.stft_synthesize_regular_f64(regular.magnitude,regular.phase,grid,regular.normalization,0);check(reconstructed.length===192,'regular output length');
    for(const suffix of ['f32','f64']) {
        const source=suffix==='f32'?new Float32Array(signal):signal;
        for(const level of [-2147483648,-1,0,1,2,3,4,2147483647]) {
            const legacy=api['stft_analyze_regular_legacy_'+suffix](source,grid,level,options),selected=api['stft_analyze_regular_'+suffix](source,grid,level>=3?0:1,options);
            same(legacy.magnitude,selected.magnitude,'signed source level magnitudes');same(legacy.phase,selected.phase,'signed source level phases');legacy.free();selected.free();
        }
    }
    check(Math.max(...reconstructed.slice(64,128).map((v,i)=>Math.abs(v-signal[i+64])))<1e-4,'native audio reconstruction bound');
    const copied=grid.cloned();grid.hop=0;throws(()=>api.stft_analyze_regular_f64(signal,grid,0,options),'invalid grid');check(copied.hop===8,'grid clone independent');grid.hop=8;
    for(const code of [2,0xffffffff]) {throws(()=>api.stft_analyze_regular_f64(signal,grid,code,options),'precision Result validation');throws(()=>api.stft_synthesize_regular_f64(regular.magnitude,regular.phase,grid,regular.normalization,code),'Nyquist Result validation');check(grid.frames===24 && options.magnitude,'owners released');}
    for(const [field,bad,good] of [['frames',0xffffffff,24],['window_factor',0,8],['zero_padding_factor',0,2]]) {grid[field]=bad;throws(()=>api.stft_analyze_regular_f64(signal,grid,0,options),'grid bounds');grid[field]=good;}
    throws(()=>api.stft_synthesize_regular_f64(new Float64Array([0]),new Float64Array([0]),grid,1,0),'regular inverse shape');
    const signal32=new Float32Array(signal),regular32=api.stft_analyze_regular_f32(signal32,grid,0,options),reconstructed32=api.stft_synthesize_regular_f32(regular32.magnitude,regular32.phase,grid,regular32.normalization,0);check(reconstructed32.length===192,'binary32 regular computation');regular32.free();
    grid.frames=1;const one=api.stft_analyze_regular_f64(signal,grid,0,options);check(one.normalization>0,'single frame normalization');one.free();
    regular.free();approximate.free();analysis.free();explicit.free();options.free();grid.free();copied.free();
    return {analysisCases,synthesisCases,comparisons,skippedPhase,maximumMagnitude,maximumWeightedPhase,maximumSynthesis,functions:6,owners:8};
}
