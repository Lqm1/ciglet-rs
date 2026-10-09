function check(value, label) { if (!value) throw new Error(label); }
function throws(action, label) { let failed=false;try { action(); } catch { failed=true; }check(failed,label); }
function sameBytes(a,b,label) {
    const x=new Uint8Array(a.buffer,a.byteOffset,a.byteLength), y=new Uint8Array(b.buffer,b.byteOffset,b.byteLength);
    check(x.length===y.length && x.every((v,i)=>v===y[i]),label);
}
export async function verifyInstantaneousFrequencyBindings(api, fixture) {
    const fields=['real','imaginary','derivative_real','derivative_imaginary'];
    let cases=0, estimates=0, comparisons=0;const maximumKernelError=[0,0], maximumEstimateError=[0,0];
    for(const width of [4,8]) {
        const precision=width===4?0:1,T=width===4?Float32Array:Float64Array;
        const Kernels=api['InstantaneousFrequencyKernelsF'+width*8], Detector=api['InstantaneousFrequencyDetectorF'+width*8];
        const raw=new Uint8Array(await fixture('instantaneous-frequency-'+(width===4?'float':'double')+'.bin'));
        check(String.fromCharCode(...raw.slice(0,4))==='IFD1','fixture magic');const data=new DataView(raw.buffer);let cursor=4;
        const integer=()=>{const n=data.getUint32(cursor,true);cursor+=4;return n;};
        const scalar=()=>{const v=width===4?data.getFloat32(cursor,true):data.getFloat64(cursor,true);cursor+=width;return v;};
        const vector=n=>new T(Array.from({length:n},scalar));
        check(integer()===width && integer()===24,'complete cases and width');
        for(let c=0;c<24;c++) {
            cases++;const center=scalar(),resolution=scalar(),length=integer();
            const detector=new Detector(center,resolution),kernels=detector.kernels, expectedFields=[];
            check(detector.center_frequency===center,'center retained');
            for(const field of fields) {
                const expected=vector(length), actual=kernels[field];expectedFields.push(expected);check(actual.length===length,'kernel length');
                actual.forEach((value,i)=>{
                    const error=Math.abs(value-expected[i])/(1+Math.abs(expected[i]));maximumKernelError[precision]=Math.max(maximumKernelError[precision],error);
                    check(error <= (width===4?2e-7:3e-15),'unchanged kernel C bound');if(width===4) check(value===expected[i],'exact binary32 kernels');comparisons++;
                });
            }
            const copied=detector.cloned(), original=new Kernels(...expectedFields), configured=Detector.from_kernels(center,original);
            for(let s=0;s<4;s++) {
                estimates++;const samples=vector(integer()),expected=scalar(),before=samples.slice();
                const actual=detector.estimate_with(samples,1),error=Math.abs(actual-expected)/(1+Math.abs(expected));
                maximumEstimateError[precision]=Math.max(maximumEstimateError[precision],error);
                check(error <= (width===4?0:1e-12),'unchanged estimate C bound');comparisons++;
                check(copied.estimate_with(samples,1)===actual,'clone estimation');
                const configuredValue=configured.estimate_with(samples,1);
                check(Math.abs(configuredValue-expected)/(1+Math.abs(expected)) <= (width===4?0:1e-12),'custom C kernels retain reference estimate');comparisons++;
                // Compare against a separately configured copy of the exact C kernels.
                const configuredCopy=configured.cloned();check(configuredCopy.estimate_with(samples,1)===configuredValue,'custom kernel estimate agrees');configuredCopy.free();
                check(detector.estimate(samples)===detector.estimate_with(samples,0),'default stable policy');sameBytes(samples,before,'samples immutable');
            }
            const saved=kernels.real;kernels.free();detector.free();copied.free();original.free();configured.free();check(saved.length===length,'kernel snapshot lifetime');
        }
        check(cursor===raw.length,'fixture fully consumed');
        const custom=new Kernels(new T([1,0]),new T([0,1]),new T([0,-2]),new T([2,0]));
        const detector=Detector.from_kernels(.125,custom),copy=detector.cloned();custom.free();
        const max=width===4?3.4028234663852886e38:Number.MAX_VALUE,tiny=width===4?2**-149:Number.MIN_VALUE;
        for(const samples of [new T([1,2]),new T([max,max/2]),new T([tiny,2*tiny])]) {
            check(Math.abs(detector.estimate(samples)-1/Math.PI) <= (width===4?2e-8:2e-16),'analytic Stable full scales');
        }
        check(Object.is(detector.estimate(new T([NaN])),0),'short buffer positive zero');
        check(detector.estimate(new T([NaN,1,2,NaN]))===detector.estimate(new T([1,2])),'unused samples ignored');
        for(const mode of [0,1]) {
            throws(()=>detector.estimate_with(new T([0,0]),mode),'zero response rejected');
            throws(()=>detector.estimate_with(new T([NaN,1]),mode),'used nonfinite samples');
        }
        for(const code of [2,9,0xffffffff]) {throws(()=>detector.estimate_with(new T([1,2]),code),'invalid policy');check(detector.center_frequency===.125,'owner released after error');}
        for(const center of [NaN,Infinity,-Infinity]) {throws(()=>detector.set_center_frequency(center),'invalid center edit');check(detector.center_frequency===.125,'center edit atomic');}
        detector.set_center_frequency(-.25);check(detector.center_frequency===-.25 && copy.center_frequency===.125,'center metadata independent');
        detector.set_center_frequency(-0);check(Object.is(detector.center_frequency,-0),'signedzero center');
        const saved=detector.kernels, edited=saved.cloned();
        for(const center of [NaN,Infinity]) {throws(()=>Detector.from_kernels(center,saved),'invalid custom center');check(saved.real.length===2,'custom center failure releases kernel owner');}
        for(const field of fields) {
            const original=edited[field];edited[field]=new T();detector.kernels=edited;
            throws(()=>detector.estimate(new T([1,2])),'empty mismatched mutable kernel');check(detector.center_frequency===0,'mutable error releases owner');
            edited[field]=new T([NaN,1]);detector.kernels=edited;throws(()=>detector.estimate(new T([1,2])),'invalid mutable kernel');
            edited[field]=original;detector.kernels=edited;check(Number.isFinite(detector.estimate(new T([1,2]))),'repaired kernels');
        }
        const rawWords=width===4?new Uint32Array([0x7f812345,0x80000000,1,0xffc12345]):new BigUint64Array([0x7ff0000000001234n,0x8000000000000000n,1n,0xfff8000000004321n]);
        const arbitrary=new T(rawWords.buffer), storage=new Kernels(arbitrary,new T(),new T([1]),arbitrary), storageCopy=storage.cloned();
        sameBytes(storage.real,arbitrary,'full raw kernels');sameBytes(storageCopy.derivative_imaginary,arbitrary,'clone raw kernels');
        for(const field of fields) {storage[field]=arbitrary;sameBytes(storage[field],arbitrary,'every mutable raw field');}
        detector.kernels=storage;const invalidSnapshot=detector.kernels;sameBytes(invalidSnapshot.real,arbitrary,'detector stores full invalid edits');
        throws(()=>detector.estimate(new T()),'validate kernels even for short samples');
        throws(()=>Detector.from_kernels(.125,storage),'validate custom construction');check(storage.real.length===4,'invalid custom owner recoverable');
        detector.kernels=saved;check(Number.isFinite(detector.estimate(new T([1,2]))),'repair after arbitrary edits');
        storage.free();storageCopy.free();invalidSnapshot.free();saved.free();edited.free();copy.free();detector.free();
        for(const resolution of [0,-1,NaN,Infinity,5,tiny]) throws(()=>new Detector(.125,resolution),'invalid resolution');
        for(const center of [NaN,Infinity]) throws(()=>new Detector(center,.125),'invalid center');
        const truncated=new Detector(.125,.3), truncatedKernels=truncated.kernels;check(truncatedKernels.real.length===13,'source truncation');truncatedKernels.free();truncated.free();
    }
    const sinusoid=new api.InstantaneousFrequencyDetectorF64(.125,.015625), signal=new Float64Array(512);
    signal.forEach((_,i)=>signal[i]=Math.cos(2*Math.PI*.125*i+.3));
    check(Math.abs(sinusoid.estimate(signal)-.125)<1e-12,'independent sinusoid');sinusoid.free();
    return {cases, estimates, comparisons, maximumKernelError, maximumEstimateError, owners:4, policies:2};
}
