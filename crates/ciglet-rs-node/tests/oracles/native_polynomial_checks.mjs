function check(condition, message) { if (!condition) throw new Error(message); }
function throws(action, message) { let failed = false; try { action(); } catch { failed = true; } check(failed, message); }
function bytes(values) { return new Uint8Array(values.buffer, values.byteOffset, values.byteLength); }
function sameBytes(a, b, label) {
    const x = bytes(a), y = bytes(b); check(x.length === y.length && x.every((v, i) => v === y[i]), label);
}
function multiply(a, b) { return [a[0]*b[0]-a[1]*b[1], a[0]*b[1]+a[1]*b[0]]; }
export async function verifyPolynomialBindings(api, fixture) {
    let cases = 0, comparisons = 0, trigCases = 0;
    function equal(actual, expected, label) {
        check(actual.length === expected.length, label + ' length');
        actual.forEach((value, i) => { check(value === expected[i], label + ' component ' + i + ': ' + value + ' vs ' + expected[i]); comparisons++; });
    }
    for (const width of [4, 8]) {
        const suffix = width === 4 ? 'f32' : 'f64', T = width === 4 ? Float32Array : Float64Array;
        const Values = api[width === 4 ? 'ComplexValuesF32' : 'ComplexValuesF64'];
        const Report = api[width === 4 ? 'PolynomialRootsF32' : 'PolynomialRootsF64'];
        const evaluate = api['polynomial_evaluate_' + suffix], evaluateReal = api['polynomial_evaluate_real_' + suffix];
        const roots = api['polynomial_roots_' + suffix], rootsReal = api['polynomial_roots_real_' + suffix];
        const raw = new Uint8Array(await fixture('polynomial-' + (width === 4 ? 'float' : 'double') + '.bin'));
        const data = new DataView(raw.buffer); let cursor = 4;
        check(String.fromCharCode(...raw.slice(0,4)) === 'POL1', 'fixture magic');
        const integer = () => { const n = data.getUint32(cursor, true); cursor += 4; return n; };
        const scalar = () => { const n = width === 4 ? data.getFloat32(cursor, true) : data.getFloat64(cursor, true); cursor += width; return n; };
        const complex = () => [scalar(), scalar()];
        check(integer() === width, 'fixture width'); const count = integer();
        check(count === (width === 4 ? 32 : 33), 'complete fixture cases');
        for (let c = 0; c < count; c++) {
            cases++; const n = integer(), real = new T(n), imaginary = new T(n);
            for (let i=0;i<n;i++) [real[i], imaginary[i]] = complex();
            const sourceReal = real.slice(), sourceImaginary = imaginary.slice();
            const input = new Values(real, imaginary), realOnly = imaginary.every(v => v === 0);
            for (let i=0;i<3;i++) {
                const [x,y] = complex(), expected = complex();
                const output = evaluate(input, x, y), snapshot = output.interleaved;
                check(output.length === 1, 'complete complex evaluation'); output.free();
                equal(snapshot, expected, 'C polynomial evaluation');
                if (realOnly) { const other = evaluateReal(real,x,y); equal(other.interleaved,expected,'C real evaluation'); other.free(); }
            }
            const expected = Array.from({length:2*(n-1)}, scalar);
            const result = roots(input), values = result.values, output = values.interleaved;
            equal(output, expected, 'C roots in source order');
            check(result.iterations >= 1 && result.iterations <= 50, 'bounded iterations');
            check(Number.isFinite(result.maximum_residual), 'finite residual');
            check(result.converged === (result.maximum_residual < 1e-6), 'returned-root convergence');
            if (realOnly) {
                const other = rootsReal(real), otherValues = other.values;
                equal(otherValues.interleaved, output, 'real root agreement');
                check(other.iterations === result.iterations && other.converged === result.converged && other.maximum_residual === result.maximum_residual, 'complete report agreement');
                otherValues.free(); other.free();
            }
            const cloned = result.cloned(), clonedValues = cloned.values;
            equal(clonedValues.interleaved, output, 'clone roots');
            check(cloned.iterations === result.iterations && cloned.converged === result.converged && cloned.maximum_residual === result.maximum_residual, 'clone report');
            input.free(); result.free(); values.free(); clonedValues.free(); cloned.free();
            equal(output, expected, 'result storage outlives owners');
            sameBytes(real, sourceReal, 'real input immutable'); sameBytes(imaginary, sourceImaginary, 'imaginary input immutable');
        }
        const trig = integer(); check(trig === 61, 'complete trig tail');
        for(let i=0;i<trig;i++) {
            const x = Math.fround(scalar()), sine = scalar(), cosine = scalar();
            check(api.approximation_sine_fast(x) === sine && api.approximation_cosine_fast(x) === cosine, 'source root-initialization trig'); trigCases++;
        }
        check(cursor === raw.length, 'fixture fully consumed');

        const constantValues = new Values(new T([1]),new T([0]));
        const constant = roots(constantValues), constantReal = rootsReal(new T([1]));
        for(const report of [constant,constantReal]) {
            const v=report.values; check(v.length===0 && report.iterations===0 && report.converged && report.maximum_residual===0,'constant report');v.free();report.free();
        }
        const evalConstant = evaluate(constantValues,0,0);equal(evalConstant.interleaved,[1,0],'constant evaluation');evalConstant.free();constantValues.free();
        const linear = rootsReal(new T([1,-1])), linearValues = linear.values;
        equal(linearValues.interleaved,[1,0],'exact linear root');check(linear.iterations===1 && linear.maximum_residual===0 && linear.converged,'linear report');linearValues.free();linear.free();
        const quadratic = rootsReal(new T([1,0,1])), quadraticValues=quadratic.values;
        check(quadratic.converged && quadratic.maximum_residual<1e-6,'quadratic convergence');
        const q=quadraticValues.interleaved; for(let i=0;i<q.length;i+=2) check(Math.abs(q[i])<1e-6 && Math.abs(Math.abs(q[i+1])-1)<1e-6,'independent imaginary roots');
        quadraticValues.free();quadratic.free();

        for(const [r,i] of [[[],[]],[[0,1],[0,0]],[[NaN,1],[0,0]],[[1,1],[0,Infinity]]]) {
            const invalid=new Values(new T(r),new T(i));throws(()=>roots(invalid),'invalid roots');check(invalid.length===r.length,'input owner recoverable');invalid.free();
        }
        for(const r of [[],[0,1],[NaN,1],[Infinity,1]]) throws(()=>rootsReal(new T(r)),'invalid real roots');
        for(const [r,i] of [[[],[]],[[1,NaN],[0,0]],[[1,1],[0,Infinity]]]) {
            const invalid=new Values(new T(r),new T(i));throws(()=>evaluate(invalid,1,0),'invalid evaluation');check(invalid.length===r.length,'evaluation owner recovered');invalid.free();
        }
        const valid=new Values(new T([1,-1]),new T([0,0]));
        for(const [x,y] of [[NaN,0],[0,Infinity],[Infinity,0]]) {
            throws(()=>evaluate(valid,x,y),'invalid complex argument');throws(()=>evaluateReal(new T([1,-1]),x,y),'invalid real argument');check(valid.length===2,'argument failure owner recovery');
        }
        for(const r of [[],[NaN],[Infinity]]) throws(()=>evaluateReal(new T(r),1,0),'invalid real evaluation');
        const max=width===4 ? 3.4028234663852886e38 : Number.MAX_VALUE;
        throws(()=>evaluateReal(new T([max,max]),2,0),'evaluation overflow');
        const huge=new Values(new T([max,max]),new T([0,0]));throws(()=>evaluate(huge,2,0),'complex overflow');huge.free();
        const overflowRoots=new Values(new T([max,1]),new T([0,0]));throws(()=>roots(overflowRoots),'normalization overflow');overflowRoots.free();
        const recovered=evaluate(valid,2,0);equal(recovered.interleaved,[1,0],'successful computation after errors');recovered.free();valid.free();

        const rawComponents = width===4 ? new Uint32Array([0x7f812345,0x80000000,0x00000001,0xffc12345]) : new BigUint64Array([0x7ff0000000001234n,0x8000000000000000n,1n,0xfff8000000004321n]);
        const arbitrary=new T(rawComponents.buffer), r=new T([arbitrary[0],arbitrary[2]]), im=new T([arbitrary[1],arbitrary[3]]);
        // Copy raw component planes without crossing the JavaScript Number ABI.
        const words = width===4 ? Uint32Array : BigUint64Array;
        new words(r.buffer).set([rawComponents[0],rawComponents[2]]);new words(im.buffer).set([rawComponents[1],rawComponents[3]]);
        const arbitraryValues = new Values(r,im), report = new Report(arbitraryValues,0xffffffff,false,-0), saved=report.values;
        sameBytes(saved.real,r,'arbitrary real report storage');sameBytes(saved.imaginary,im,'arbitrary imaginary report storage');saved.free();
        check(report.iterations===0xffffffff && !report.converged && Object.is(report.maximum_residual,-0),'arbitrary report metadata');
        const residualWords=new BigUint64Array([0x7ff0000000005678n]), residual=new Float64Array(residualWords.buffer);
        report.set_residual_storage(residual);sameBytes(report.residual_storage(),residual,'residual payload');
        const copy=report.cloned(), snapshot=copy.values;report.iterations=0;report.converged=true;report.maximum_residual=Infinity;
        check(copy.iterations===0xffffffff && !copy.converged,'clone metadata remains independent');sameBytes(copy.residual_storage(),residual,'clone residual remains independent');
        check(report.iterations===0 && report.converged && report.maximum_residual===Infinity,'full metadata mutation');
        for(const length of [0,2]) {throws(()=>copy.set_residual_storage(new Float64Array(length)),'atomic residual shape');sameBytes(copy.residual_storage(),residual,'residual error leaves owner reusable');}
        const replacement=new Values(new T([3]),new T([-4])); report.values=replacement;replacement.free();
        const replaced=report.values;equal(replaced.interleaved,[3,-4],'complete values replacement');replaced.free();
        report.free();arbitraryValues.free();copy.free();sameBytes(snapshot.real,r,'clone snapshot real lifetime');sameBytes(snapshot.imaginary,im,'clone snapshot imaginary lifetime');snapshot.free();
    }
    const coefficients=new Float64Array([1,-36,546,-4536,22449,-67284,118124,-109584,40320]);
    const limited=api.polynomial_roots_real_f64(coefficients), values=limited.values, roots=values.interleaved;
    check(limited.iterations===50 && !limited.converged && limited.maximum_residual>1e-6,'unconverged report retained');
    let maximum=0;
    for(let i=0;i<roots.length;i+=2) {
        let product=[1,0];for(let k=1;k<=8;k++) product=multiply(product,[roots[i]-k,roots[i+1]]);
        maximum=Math.max(maximum,Math.hypot(...product));
    }
    check(Math.abs(maximum-limited.maximum_residual)/maximum<1e-8,'independent returned-root residual');values.free();limited.free();
    return {cases, comparisons, trigCases, functions:8, owners:2, exactCResults:true};
}
