function check(v,label) {if(!v) throw new Error(label);}
function throws(action,label) {let failed=false;try{action();}catch{failed=true;}check(failed,label);}
function same(a,b,label) {check(a.length===b.length && a.every((v,i)=>v===b[i]),label);}
function bits(a,b,label) {const x=new Uint8Array(a.buffer,a.byteOffset,a.byteLength),y=new Uint8Array(b.buffer,b.byteOffset,b.byteLength);check(x.length===y.length && x.every((v,i)=>v===y[i]),label);}
export async function verifyEnvelopeBindings(api,fixture) {
    let cases=0,comparisons=0;const maximumCError=[[0,0],[0,0]];
    for(const width of [4,8]) {
        const index=width===4?0:1,T=width===4?Float32Array:Float64Array,suffix=width===4?'f32':'f64',Plan=api['SpectralEnvelopePlanF'+width*8],Result=api['SpectralEnvelopeF'+width*8];
        const raw=new Uint8Array(await fixture('spectral-envelope-'+(width===4?'float':'double')+'.bin')),data=new DataView(raw.buffer);let cursor=4;
        const integer=()=>{const v=data.getUint32(cursor,true);cursor+=4;return v;};
        const scalar=()=>{const v=width===4?data.getFloat32(cursor,true):data.getFloat64(cursor,true);cursor+=width;return v;};
        const vector=n=>new T(Array.from({length:n},scalar));
        check(String.fromCharCode(...raw.slice(0,4))==='ENV1' && integer()===width && integer()===144,'complete header');
        for(let c=0;c<144;c++) {
            cases++;const length=integer(),count=integer(),fundamental=scalar(),spectrum=vector(length/2+1),expected=[vector(length),vector(length/2+1)],before=spectrum.slice(),harmonics=count===0?undefined:count;
            const plan=new Plan(length);check(plan.length===length && plan.bins===length/2+1 && !plan.is_empty(),'full plan metadata');
            const result=plan.estimate_with_smoothing(spectrum,fundamental,harmonics,1,1),outputs=[result.log_spectrum,result.cepstrum];
            outputs.forEach((values,field)=>{check(values.length===expected[field].length,'complete native output');values.forEach((v,i)=>{const error=Math.abs(v-expected[field][i])/(1+Math.abs(expected[field][i]));maximumCError[index][field]=Math.max(maximumCError[index][field],error);check(error<=(width===4?3e-7:5e-16),'unchanged envelope C bound');comparisons++;});});
            const reused=plan.estimate_with_smoothing(spectrum,fundamental,harmonics,1,1);same(reused.log_spectrum,outputs[0],'plan reuse spectrum');same(reused.cepstrum,outputs[1],'plan reuse coefficients');reused.free();
            const defaults=plan.estimate(spectrum,fundamental,harmonics),one=api['spectral_envelope_estimate_'+suffix](spectrum,length,fundamental,harmonics),withPolicy=plan.estimate_with(spectrum,fundamental,harmonics,0),explicit=plan.estimate_with_smoothing(spectrum,fundamental,harmonics,0,0);
            for(const other of [one,withPolicy,explicit]) {same(other.log_spectrum,defaults.log_spectrum,'default entrypoint spectrum');same(other.cepstrum,defaults.cepstrum,'default entrypoint cepstrum');other.free();}
            const clone=result.cloned();same(clone.log_spectrum,outputs[0],'clone spectrum');same(clone.cepstrum,outputs[1],'clone cepstrum');
            bits(spectrum,before,'source spectrum immutable');result.free();defaults.free();clone.free();plan.free();check(outputs[0].length===length && outputs[1].length===length/2+1,'snapshot lifetime');
        }
        check(cursor===raw.length,'fixture fully consumed');
        const plan=new Plan(8),source=new T([.125,.25,.5,1,8]);
        for(const nyquist of [0,1]) for(const smoothing of [0,1]) {
            const result=plan.estimate_with_smoothing(source,.25,undefined,nyquist,smoothing);check(result.log_spectrum.length===8 && result.cepstrum.length===5,'all independent modes');if(nyquist===1)check(result.cepstrum[4]===result.cepstrum[3],'Legacy Nyquist coefficients');result.free();
        }
        const preserve=plan.estimate(source,.25,undefined),legacy=plan.estimate_with(source,.25,undefined,1);check(preserve.log_spectrum.some((v,i)=>v!==legacy.log_spectrum[i]),'Nyquist policy distinct');preserve.free();legacy.free();
        const baseline=plan.estimate(source,.125,5),before=baseline.log_spectrum;
        for(const code of [2,9,0xffffffff]) {
            throws(()=>plan.estimate_with(source,.125,5,code),'invalid Nyquist');throws(()=>plan.estimate_with_smoothing(source,.125,5,0,code),'invalid smoothing');check(plan.length===8,'owner released');
        }
        for(const f0 of [0,-.1,.5001,NaN,Infinity]) throws(()=>plan.estimate(source,f0,undefined),'fundamental bounds');
        for(const count of [0,1,0x80000000,0xffffffff]) throws(()=>plan.estimate(source,.125,count),'harmonic count bounds');
        for(const samples of [new T(4).fill(.5),new T(5).fill(-1),new T(5).fill(Infinity),new T([.125,.25,.5,1,NaN])]) throws(()=>plan.estimate(samples,.125,undefined),'spectrum bounds including Legacy unused Nyquist validation');
        const repaired=plan.estimate(source,.125,5);same(repaired.log_spectrum,before,'post-error workspace reuse');repaired.free();baseline.free();
        for(const length of [0,1,3,7]) throws(()=>new Plan(length),'plan length bounds');
        const tinyPlan=new Plan(2),small=tinyPlan.estimate(new T([.5,1]),.5,undefined);check(small.log_spectrum[0]===small.log_spectrum[1],'two-point envelope');small.free();
        const tiny=tinyPlan.estimate(new T([.5,1]),1e-10,undefined),explicit=tinyPlan.estimate(new T([.5,1]),1e-10,2);same(tiny.log_spectrum,explicit.log_spectrum,'unbounded implicit count two-point cutoff');same(tiny.cepstrum,explicit.cepstrum,'unbounded implicit count coefficients');tiny.free();explicit.free();tinyPlan.free();
        const words=width===4?new Uint32Array([0x7f812345,0x80000000,1,0xffc12345]):new BigUint64Array([0x7ff0000000001234n,0x8000000000000000n,1n,0xfff8000000004321n]);
        const arbitrary=new T(words.buffer),storage=new Result(arbitrary,new T()),copy=storage.cloned();bits(storage.log_spectrum,arbitrary,'full raw log spectrum');check(storage.cepstrum.length===0,'arbitrary mismatched coefficient storage');
        storage.cepstrum=arbitrary;storage.log_spectrum=new T();bits(storage.cepstrum,arbitrary,'mutable cepstrum bits');bits(copy.log_spectrum,arbitrary,'clone independent bits');storage.log_spectrum=arbitrary;bits(storage.log_spectrum,arbitrary,'mutable log spectrum bits');
        const snapshot=storage.cepstrum;storage.free();copy.free();bits(snapshot,arbitrary,'owned output lifetime');plan.free();
    }
    const plan=new api.SpectralEnvelopePlanF64(32);
    for(const value of [0,.5,2]) {
        const result=plan.estimate(new Float64Array(17).fill(value),.09375,undefined),expected=api.approximation_logarithm(Math.fround(value+1e-15));
        result.log_spectrum.forEach(v=>check(Math.abs(v-expected)<2e-13,'independent constant spectrum'));
        check(Math.abs(result.cepstrum[0]-expected)<2e-13 && result.cepstrum.slice(1).every(v=>Math.abs(v)<2e-13),'independent constant coefficients');result.free();
    }
    const source=Float64Array.from({length:17},(_,i)=>.25+i*.125),result=plan.estimate(source,.125,5);
    for(let i=17;i<32;i++) check(Math.abs(result.log_spectrum[i]-result.log_spectrum[32-i])<1e-14,'full symmetric spectrum');result.free();plan.free();
    const cutoffPlan=new api.SpectralEnvelopePlanF64(8),cutoff=cutoffPlan.estimate(new Float64Array([.5,1,2,4,8]),.125,2);
    check(cutoff.log_spectrum.every(v=>Math.abs(v-cutoff.log_spectrum[0])<1e-14),'corrected zero cutoff');cutoff.free();
    const wide=cutoffPlan.estimate(new Float64Array(5),1e-12,undefined);check(wide.log_spectrum.every(v=>Math.abs(v-wide.log_spectrum[0])<1e-13),'large implicit harmonic count');wide.free();cutoffPlan.free();
    return {cases,comparisons,maximumCError,functions:2,owners:4,nyquistPolicies:2,smoothingPolicies:2};
}
