function check(value,label){if(!value)throw new Error(label);}
function throws(action,label){let failed=false;try{action();}catch{failed=true;}check(failed,label);}
export async function verifyScalarBindings(api,fixture){
    let foldCases=0,foldCalls=0,expressionChecks=0;
    const constants=api.scalar_math_constants(),constantBits=new BigUint64Array(constants.buffer);check(constantBits.every((bits,i)=>bits===[0x3cd203af9ee75616n,0x430c6bf526340000n,0x400921fb54442d18n,0x3ff921fb54442d18n][i]),'four original constant bits');constants[2]=0;check(api.scalar_math_constants()[2]===Math.PI,'independent constant snapshots');
    for(const width of [4,8]){
        const T=width===4?Float32Array:Float64Array,suffix=width===4?'f32':'f64',cast=width===4?Math.fround:x=>x,fold=api['scalar_fold_'+suffix],add=api['scalar_add_'+suffix],square=api['scalar_accumulate_square_'+suffix];
        const bytes=new Uint8Array(await fixture('custom_window-'+(width===4?'float':'double')+'.bin')),view=new DataView(bytes.buffer,bytes.byteOffset,bytes.byteLength);let cursor=4;
        const integer=()=>{const value=view.getUint32(cursor,true);cursor+=4;return value;},scalar=()=>{const value=width===4?view.getFloat32(cursor,true):view.getFloat64(cursor,true);cursor+=width;return value;};
        check(String.fromCharCode(...bytes.slice(0,4))==='CUS1'&&integer()===width&&integer()===60,'custom corpus header');
        // Window records are verified by the separate complete window binding suite.
        for(let i=0;i<60;i++){integer();integer();const count=integer();integer();for(let j=0;j<count;j++)scalar();}
        check(String.fromCharCode(...bytes.slice(cursor,cursor+4))==='RED1','fold tail');cursor+=4;check(integer()===12,'all original fold records');
        for(let i=0;i<12;i++){
            foldCases++;const count=integer(),expectedCalls=integer(),initial=scalar(),input=new T(Array.from({length:count},scalar)),expected=scalar(),context={calls:0},actual=fold(input,initial,(accumulator,value,received)=>{check(received===context,'callback context identity');context.calls++;return cast(cast(accumulator*0.5)-cast(value*0.125));},context);
            check(Object.is(actual,expected),'exact original custom fold');check(context.calls===expectedCalls,'original callback count');foldCalls+=context.calls;
        }
        check(cursor===bytes.length,'complete fixture consumption');
        const maximum=width===4?3.4028234663852886e38:Number.MAX_VALUE,tiny=width===4?2**-149:Number.MIN_VALUE,epsilon=width===4?2**-23:2**-52;
        for(const[first,second]of [[0,-0],[-0,-0],[1,2],[-7,0.125],[maximum,maximum],[tiny,tiny],[maximum,-maximum],[Infinity,-Infinity],[NaN,3],[3,NaN]]){
            const expectedAdd=cast(cast(first)+cast(second)),expectedSquare=cast(cast(first)+cast(cast(second)*cast(second)));check(Object.is(add(first,second),expectedAdd),'selected-scalar addition');check(Object.is(square(first,second),expectedSquare),'selected multiply before add');expressionChecks+=2;
        }
        check(square(-(1+2*epsilon),1+epsilon)===0,'square is not fused');expressionChecks++;
        let calls=0;check(Object.is(fold(new T(),-0,undefined,null),-0),'empty signed-zero seed');check(fold(new T(),7,()=>{calls++;throw new Error('unused');},{})===7&&calls===0,'empty fold never invokes callback');
        throws(()=>fold(new T([1]),0,undefined,null),'missing callback on nonempty input');const context={calls:0},input=new T([1,2,3]),copy=input.slice();check(fold(input,4,(accumulator,value,state)=>{state.calls++;return accumulator*10+value;},context)===4123&&context.calls===3,'left-to-right stateful order');check(input.every((v,i)=>v===copy[i]),'input immutable');
        const sentinel={message:'fold callback failure'};let received;calls=0;try{fold(input,0,()=>{calls++;throw sentinel;},null);}catch(error){received=error;}check(received===sentinel&&calls===1,'exception identity and early stop');check(fold(input,0,(a,b)=>a+b,null)===6,'fold recovered after exception');
        for(const bad of ['1',null,undefined,1n,{}]){calls=0;throws(()=>fold(input,0,()=>{calls++;return bad;},null),'nonnumeric callback result');check(calls===1,'invalid result stops immediately');}
        calls=0;check(Number.isNaN(fold(input,0,()=>{calls++;return NaN;},null))&&calls===3,'IEEE NaN callback result retained');check(fold(input,0,()=>Infinity,null)===Infinity,'IEEE infinity result retained');
        check(fold(new T([1,2]),3,(a,b)=>a+fold(new T([b]),0,(c,d)=>c+d,null),null)===6,'nested folds');
    }
    return{foldCases,foldCalls,expressionChecks,constantComparisons:4};
}
