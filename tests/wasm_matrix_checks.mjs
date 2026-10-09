function check(value,label) {if(!value) throw new Error(label);}
function throws(action,label) {let failed=false;try{action();}catch{failed=true;}check(failed,label);}
function bytes(a) {return new Uint8Array(a.buffer,a.byteOffset,a.byteLength);}
function sameBytes(a,b,label) {const x=bytes(a),y=bytes(b);check(x.length===y.length && x.every((v,i)=>v===y[i]),label);}
export async function verifyMatrixBindings(api,fixture) {
    let products=0,permutations=0,solves=0,comparisons=0;const maximumResidual=[0,0];
    function equal(a,b,label) {check(a.length===b.length,label+' length');a.forEach((v,i)=>{check(v===b[i],label+' index '+i);comparisons++;});}
    for(const width of [4,8]) {
        const suffix=width===4?'f32':'f64',T=width===4?Float32Array:Float64Array,Matrix=api['MatrixF'+width*8],Lu=api['LuF'+width*8],apply=api['matrix_apply_swaps_'+suffix];
        for(const corrected of [0,1]) {
            const raw=new Uint8Array(await fixture('matrix-'+(corrected?'corrected':'legacy')+'-'+(width===4?'float':'double')+'.bin')),data=new DataView(raw.buffer);let cursor=4;
            const integer=()=>{const v=data.getUint32(cursor,true);cursor+=4;return v;};
            const scalar=()=>{const v=width===4?data.getFloat32(cursor,true):data.getFloat64(cursor,true);cursor+=width;return v;};
            const vector=n=>new T(Array.from({length:n},scalar)),indices=n=>new Uint32Array(Array.from({length:n},integer));
            check(String.fromCharCode(...raw.slice(0,4))==='MAT1' && integer()===width && integer()===corrected,'fixture header');
            const productCount=integer();check(productCount===54,'complete product cases');
            for(let c=0;c<productCount;c++) {
                products++;const m=integer(),n=integer(),l=integer(),aValues=vector(m*n),bValues=vector(n*l),expected=vector(m*l),expectedVector=vector(m);
                const a=new Matrix(m,n,aValues),b=new Matrix(n,l,bValues),product=a.multiply(b),copy=a.cloned();
                check(a.rows===m && a.columns===n && product.rows===m && product.columns===l,'complete dimensions');
                equal(product.as_slice(),expected,'exact C matrix product');equal(a.multiply_vector(bValues.slice(0,n)),expectedVector,'exact C vector product');
                equal(copy.as_slice(),aValues,'matrix clone');equal(a.as_slice(),aValues,'left input retained');equal(b.as_slice(),bValues,'right input retained');
                for(let column=0;column<n;column++) for(let row=0;row<m;row++) check(a.get(row,column)===aValues[row+column*m],'column-major lookup');
                check(a.get(m,0)===undefined && a.get(0,n)===undefined,'missing cell');
                const snapshot=product.as_slice();product.free();equal(snapshot,expected,'snapshot lifetime');
                const consumed=copy.into_column_major();equal(consumed,aValues,'consuming native buffer');throws(()=>copy.as_slice(),'consumed matrix unavailable');a.free();b.free();
            }
            const permutationCount=integer();
            for(let c=0;c<permutationCount;c++) {
                permutations++;const m=integer(),n=integer(),input=vector(m*n),values=vector(m),partners=indices(m),expected=vector(m*n),expectedVector=vector(m);
                const a=new Matrix(m,n,input);a.apply_row_swaps(partners);apply(values,partners);equal(a.as_slice(),expected,'C row swaps');equal(values,expectedVector,'C vector swaps');a.free();
            }
            const solveCount=integer();
            for(let c=0;c<solveCount;c++) {
                solves++;const n=integer(),input=vector(n*n),right=vector(n),partners=indices(n),expectedPivot=vector(n*n),expectedRight=vector(n),expectedLu=vector(n*n),expectedSolution=vector(n);
                const a=new Matrix(n,n,input),pivot=a.cloned();equal(pivot.pivot_rows(),partners,'C pivot partners');equal(pivot.as_slice(),expectedPivot,'C pivot matrix');
                const reordered=right.slice();apply(reordered,partners);equal(reordered,expectedRight,'C pivot right');
                const lu=Lu.factor_unpivoted(pivot),packed=lu.packed(),solution=lu.solve(reordered);equal(packed.as_slice(),expectedLu,'C packed LU');equal(solution,expectedSolution,'C solution');
                equal(lu.row_swaps(),Uint32Array.from({length:n},(_,i)=>i),'unpivoted swaps');
                const clone=lu.cloned();equal(clone.solve(reordered),solution,'LU clone solve');const clonePacked=clone.packed();equal(clonePacked.as_slice(),expectedLu,'LU clone factors');clonePacked.free();clone.free();
                const robust=Lu.factor(a.cloned()),observed=a.multiply_vector(robust.solve(right));
                observed.forEach((v,i)=>{const error=Math.abs(v-right[i])/(1+Math.abs(right[i]));maximumResidual[width===4?0:1]=Math.max(maximumResidual[width===4?0:1],error);check(error<=(width===4?2e-6:1e-13),'native robust residual');comparisons++;});
                equal(a.as_slice(),input,'original matrix retained');const rightBefore=right.slice();robust.solve(right);sameBytes(right,rightBefore,'right input retained');
                packed.free();lu.free();robust.free();a.free();
            }
            check(cursor===raw.length,'all four complete fixtures consumed');
        }
        const rectangular=new Matrix(2,3,new T([1,2,3,4,5,6])),identity=Matrix.identity(3),product=rectangular.multiply(identity);
        equal(product.as_slice(),rectangular.as_slice(),'rectangular identity');equal(rectangular.multiply_vector(new T([1,2,3])),new T([22,28]),'independent rectangular product');product.free();identity.free();
        const before=rectangular.as_slice();for(const partners of [new Uint32Array([1]),new Uint32Array([1,2])]) {throws(()=>rectangular.apply_row_swaps(partners),'invalid swaps');sameBytes(rectangular.as_slice(),before,'invalid swaps atomic');}
        throws(()=>rectangular.pivot_rows(),'square pivot required');sameBytes(rectangular.as_slice(),before,'pivot shape error atomic');
        throws(()=>rectangular.multiply_vector(new T([1])),'vector shape');throws(()=>rectangular.multiply_vector(new T([1,NaN,3])),'nonfinite vector');
        throws(()=>rectangular.multiply(rectangular),'matrix inner dimensions');throws(()=>Lu.factor(rectangular.cloned()),'factor rectangular');throws(()=>Lu.factor_unpivoted(rectangular.cloned()),'unpivoted rectangular');check(rectangular.rows===2,'source owner retained through errors');rectangular.free();
        for(const [m,n,v] of [[2,2,[1]],[1,1,[NaN]],[1,1,[Infinity]],[0xffffffff,0,[]],[0,0xffffffff,[]]]) throws(()=>new Matrix(m,n,new T(v)),'constructor bounds');
        throws(()=>Matrix.zeros(0xffffffff,2),'zero matrix overflow');throws(()=>Matrix.identity(0xffffffff),'identity overflow');
        const left=Matrix.zeros(2,0),right=Matrix.zeros(0,3),zeros=left.multiply(right);equal(zeros.as_slice(),new T(6),'zero inner product');equal(left.multiply_vector(new T()),new T(2),'zero inner vector');left.free();right.free();zeros.free();
        const empty=Lu.factor(Matrix.identity(0));check(empty.solve(new T()).length===0 && empty.row_swaps().length===0,'empty factor');const emptyPacked=empty.packed();check(emptyPacked.rows===0 && emptyPacked.columns===0,'empty packed');emptyPacked.free();empty.free();
        const singular=new Matrix(2,2,new T([1,2,2,4]));throws(()=>Lu.factor(singular.cloned()),'singular factor');throws(()=>Lu.factor_unpivoted(singular.cloned()),'singular unpivoted');singular.free();
        const max=width===4?3.4028234663852886e38:Number.MAX_VALUE,huge=new Matrix(1,1,new T([max]));throws(()=>huge.multiply_vector(new T([2])),'vector overflow');const two=new Matrix(1,1,new T([2]));throws(()=>huge.multiply(two),'matrix overflow');check(huge.get(0,0)===max,'overflow source retained');huge.free();two.free();
        const source=new Matrix(3,3,new T([4,2,1,4,2,0,1,2,3])),preliminary=source.cloned();equal(preliminary.pivot_rows(),new Uint32Array([0,1,2]),'initial pivot');throws(()=>Lu.factor_unpivoted(preliminary),'late zero pivot');
        const late=Lu.factor(source.cloned()),solution=late.solve(new T([15,12,10]));solution.forEach((v,i)=>check(Math.abs(v-(i+1))<(width===4?2e-6:1e-12),'independent late-pivot solution'));
        const p=source.cloned();p.apply_row_swaps(late.row_swaps());const factors=late.packed();
        for(let row=0;row<3;row++) for(let column=0;column<3;column++) {
            let value=0;for(let k=0;k<3;k++) value+=(row===k?1:row>k?factors.get(row,k):0)*(k<=column?factors.get(k,column):0);
            check(Math.abs(value-p.get(row,column))<(width===4?2e-6:1e-12),'independent P*A=L*U');
        }
        throws(()=>late.solve(new T([1])),'solve shape');throws(()=>late.solve(new T([NaN,1,2])),'solve finite');same(late.solve(new T([15,12,10])),solution,'solve owner reusable');
        const retained=late.packed();late.free();factors.free();p.free();source.free();check(retained.rows===3,'packed snapshot outlives factor');retained.free();
        const words=width===4?new Uint32Array([0x7f812345,0x80000000,1,0xffc12345]):new BigUint64Array([0x7ff0000000001234n,0x8000000000000000n,1n,0xfff8000000004321n]);
        const payload=new T(words.buffer),swapped=payload.slice(),partners=new Uint32Array([3,2,2,3]),expected=payload.slice();
        const expectedBytes=bytes(expected),originalBytes=bytes(payload);for(const [to,from] of [[0,3],[1,2],[2,1],[3,0]]) expectedBytes.set(originalBytes.slice(from*width,(from+1)*width),to*width);
        apply(swapped,partners);sameBytes(swapped,expected,'permutation retains all raw bits');
        for(const bad of [new Uint32Array([0]),new Uint32Array([0,1,2,4])]) {const v=payload.slice();throws(()=>apply(v,bad),'invalid raw swaps');sameBytes(v,payload,'invalid raw swaps no mutation');}
        const signed=new Matrix(1,2,new T([-0,0]));sameBytes(signed.as_slice(),new T([-0,0]),'finite matrix signedzero');signed.free();
    }
    for(const [suffix,T] of [['u8',Uint8Array],['u16',Uint16Array],['u32',Uint32Array],['u64',BigUint64Array],['i8',Int8Array],['i16',Int16Array],['i32',Int32Array],['i64',BigInt64Array]]) {
        const big=suffix.endsWith('64'),values=big?new T([0n,1n,-1n,0x8000000000000000n]):new T([0,1,-1,127]),before=values.slice(),expected=values.slice().reverse(),fn=api['matrix_apply_swaps_'+suffix];
        fn(values,new Uint32Array([3,2,2,3]));sameBytes(values,expected,'integer permutation full width');
        throws(()=>fn(values,new Uint32Array([0,1,2,4])),'integer invalid swaps');sameBytes(values,expected,'integer atomic failure');fn(values,new Uint32Array([3,2,2,3]));sameBytes(values,before,'integer double permutation');
    }
    return {products,permutations,solves,comparisons,maximumResidual,functions:10,owners:4,exactCResults:true};
}
function same(a,b,label) {check(a.length===b.length && a.every((v,i)=>v===b[i]),label);}
