function check(v,label) {if(!v) throw new Error(label);}
function throws(action,label) {let failed=false;try{action();}catch{failed=true;}check(failed,label);}
function same(a,b,label) {check(a.length===b.length && a.every((v,i)=>v===b[i]),label);}
function bits(a,b,label) {const x=new Uint8Array(a.buffer,a.byteOffset,a.byteLength),y=new Uint8Array(b.buffer,b.byteOffset,b.byteLength);check(x.length===y.length && x.every((v,i)=>v===y[i]),label);}
function reader(raw,width,magic,count) {
    const bytes=new Uint8Array(raw),data=new DataView(bytes.buffer);let cursor=4;
    const integer=()=>{const n=data.getUint32(cursor,true);cursor+=4;return n;};
    const scalar=()=>{const n=width===4?data.getFloat32(cursor,true):data.getFloat64(cursor,true);cursor+=width;return n;};
    check(String.fromCharCode(...bytes.slice(0,4))===magic && integer()===width && integer()===count,'complete fixture header');
    return {integer,scalar,end:()=>check(cursor===bytes.length,'fully consumed fixture')};
}
export async function verifyGlottalBindings(api,fixture) {
    let cases=0,comparisons=0,preparationCases=0;const maximumCError=[[0,0,0],[0,0,0]],fields=['period','closure','peak','return_time','excitation'];
    for(const width of [4,8]) {
        const index=width===4?0:1,T=width===4?Float32Array:Float64Array,Model=api['GlottalModelF'+width*8],Spectrum=api['GlottalSpectrumF'+width*8];
        const r=reader(await fixture('glottal-'+(width===4?'float':'double')+'.bin'),width,'LFM1',53),vector=n=>new T(Array.from({length:n},r.scalar));
        for(let c=0;c<53;c++) {
            cases++;const input=vector(3),expectedFields=vector(5),model=input[0]===0?Model.from_components(expectedFields):Model.from_shape(...input);
            bits(model.components(),expectedFields,'exact source model fields');
            fields.forEach((field,i)=>check(model[field]===expectedFields[i],'all scalar fields'));
            const rate=r.integer()|0,count=r.integer(),nf=r.integer(),frequencies=vector(nf),expected=[vector(nf),vector(nf),vector(count)],before=frequencies.slice();
            const prepared=model.prepare(),clone=prepared.cloned(),copy=prepared.model();bits(copy.components(),expectedFields,'prepared original model');copy.free();
            const response=prepared.spectrum(frequencies),actual=[response.magnitude,response.phase,prepared.sampled_period(rate,count)],tolerances=width===4?[0,0,0]:[1e-17,3e-15,5e-16];
            actual.forEach((values,field)=>{check(values.length===expected[field].length,'full output length');values.forEach((v,i)=>{const error=Math.abs(v-expected[field][i])/(1+Math.abs(expected[field][i]));maximumCError[index][field]=Math.max(maximumCError[index][field],error);check(error<=tolerances[field],'unchanged C bound field '+field+' case '+c+' index '+i+' error '+error);comparisons++;});if(width===4)bits(values,expected[field],'exact binary32 C bits');});
            const explicit=prepared.spectrum_with(frequencies,1),one=model.spectrum(frequencies),cloneResponse=clone.spectrum(frequencies);
            for(const other of [explicit,one,cloneResponse]) {same(other.magnitude,actual[0],'default/one-shot/clone magnitude');same(other.phase,actual[1],'default/one-shot/clone phase');other.free();}
            same(model.sampled_period(rate,count),actual[2],'one-shot waveform');same(clone.sampled_period(rate,count),actual[2],'clone waveform');
            const spectrumCopy=response.cloned();same(spectrumCopy.magnitude,actual[0],'spectrum clone');bits(frequencies,before,'frequency inputs immutable');
            response.free();spectrumCopy.free();model.free();prepared.free();same(clone.sampled_period(rate,count),actual[2],'prepared clone lifetime');clone.free();
        }
        r.end();
        const conditioning=reader(await fixture('glottal-preparation-endpoints-'+(width===4?'float':'double')+'.bin'),width,'LFP1',208);
        for(let c=0;c<208;c++) {
            const fieldsArray=new T(Array.from({length:20},conditioning.scalar)),model=Model.from_components(fieldsArray.slice(0,5)),prepared=model.prepare(),retained=prepared.model();
            bits(retained.components(),fieldsArray.slice(0,5),'conditioning corpus retains complete model');check(prepared.sampled_period(16000,0).length===0,'conditioning corpus preparation supports empty period');retained.free();prepared.free();model.free();preparationCases++;
        }
        conditioning.end();
        const model=Model.from_shape(1,.005,.5),prepared=model.prepare(),queries=new T([0,200,-200,800]),response=prepared.spectrum(queries);
        check(Object.is(response.magnitude[0],0) && Object.is(response.phase[0],0),'DC positive zero');
        if(width===8) {check(Math.abs(response.magnitude[1]-response.magnitude[2])<2e-10 && Math.abs(response.phase[1]+response.phase[2])<1e-6,'source approximate symmetry bounds');}
        const exact=prepared.spectrum_with(new T([200,-200]),2);check(exact.magnitude[0]===exact.magnitude[1] && exact.phase[0]===-exact.phase[1],'exact conjugate symmetry');exact.free();
        for(const precision of [0,1,2]) {const s=prepared.spectrum_with(new T([200,800]),precision);check([...s.magnitude,...s.phase].every(Number.isFinite),'all precisions');s.free();}
        const wave=prepared.sampled_period(16000,128),start=width===4?Math.round(model.period*16000):80;
        check(wave.slice(start).every(v=>Object.is(v,0)),'positive zero after period');same(prepared.sampled_period(16000,20),wave.slice(0,20),'truncated period');check(prepared.sampled_period(16000,0).length===0,'empty period');
        const doubled=model.cloned();doubled.excitation=1;const doubledWave=doubled.sampled_period(16000,128);
        wave.forEach((v,i)=>check(width===4?doubledWave[i]===2*v:Math.abs(doubledWave[i]-2*v)<1e-14,'amplitude scaling'));
        const inverted=Model.from_shape(1,.005,-.5),invertedWave=inverted.sampled_period(16000,128);wave.forEach((v,i)=>check(invertedWave[i]===-v,'signed excitation waveform'));
        const invertedSpectrum=inverted.spectrum(new T([200]));check(invertedSpectrum.magnitude[0]===response.magnitude[1],'signed excitation magnitude');invertedSpectrum.free();inverted.free();doubled.free();
        for(const code of [3,9,0xffffffff]) {throws(()=>prepared.spectrum_with(new T([200]),code),'invalid precision');const original=prepared.model();check(original.period===model.period,'owner releases after precision error');original.free();}
        for(const queries of [new T([NaN]),new T([200,Infinity])]) {throws(()=>prepared.spectrum(queries),'invalid queries');throws(()=>model.spectrum(queries),'one-shot query failure');}
        const maximum=width===4?3.4028234663852886e38:Number.MAX_VALUE;
        throws(()=>prepared.spectrum(new T([200,maximum])),'late frequency overflow');
        for(const rate of [0,-1,-2147483648]) {throws(()=>prepared.sampled_period(rate,0),'invalid sample rate even empty');throws(()=>model.sampled_period(rate,16),'invalid one-shot rate');}
        same(prepared.sampled_period(16000,128),wave,'owner reusable after failures');
        const original=model.components();for(const [field,bad] of [['period',0],['closure',1],['peak',.7],['return_time',.5],['excitation',Infinity],['period',NaN]]) {model[field]=bad;throws(()=>model.prepare(),'invalid mutable physical fields');model.set_components(original);}
        for(const shape of [0,-1,NaN,Infinity]) throws(()=>Model.from_shape(shape,.005,1),'invalid shape');
        const rawWords=width===4?new Uint32Array([0x7f812345,0x80000000,1,0xffc12345,0x7f800000]):new BigUint64Array([0x7ff0000000001234n,0x8000000000000000n,1n,0xfff8000000004321n,0x7ff0000000000000n]);
        const arbitrary=new T(rawWords.buffer),storage=Model.from_components(arbitrary),storageCopy=storage.cloned();bits(storage.components(),arbitrary,'all model payload bits');bits(storageCopy.components(),arbitrary,'model clone payloads');
        for(const length of [0,4,6]) {throws(()=>storage.set_components(new T(length)),'atomic model shape');bits(storage.components(),arbitrary,'model storage unchanged after error');throws(()=>Model.from_components(new T(length)),'constructor storage shape');}
        throws(()=>storage.prepare(),'invalid raw physical model');storage.set_components(original);check(storage.period===model.period,'model repair');
        const spectrumStorage=new Spectrum(arbitrary,new T()),spectrumClone=spectrumStorage.cloned();spectrumStorage.magnitude=new T();spectrumStorage.phase=arbitrary;
        bits(spectrumClone.magnitude,arbitrary,'spectrum complete clone');check(spectrumClone.phase.length===0,'spectrum arbitrary mismatched fields');bits(spectrumStorage.phase,arbitrary,'spectrum mutable phase');spectrumStorage.magnitude=arbitrary;bits(spectrumStorage.magnitude,arbitrary,'spectrum mutable magnitude');
        const snapshot=spectrumStorage.phase;spectrumStorage.free();spectrumClone.free();bits(snapshot,arbitrary,'spectrum snapshot lifetime');storage.free();storageCopy.free();
        const long=new Model(2,.6,.4,.05,1);same(long.sampled_period(2147483647,1),long.sampled_period(16000,1),'large sample index safe');long.free();
        response.free();prepared.free();model.free();
    }
    return {cases,comparisons,preparationCases,maximumCError,owners:6,precisions:3};
}
