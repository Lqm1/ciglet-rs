function check(value,label){if(!value)throw new Error(label);}
function same(a,b,label){check(a.length===b.length&&a.every((v,i)=>v===b[i]),label);}
function throws(action,label){let failed=false;try{action();}catch{failed=true;}check(failed,label);}
export async function verifyPlottingBindings(api,fixture){
    let transcriptBytes=0,lineCommands=0,imageCommands=0;
    const text=new TextEncoder();
    for(const width of [4,8]){
        const T=width===4?Float32Array:Float64Array,suffix=width===4?'f32':'f64',y=new T([0,-0,1.23456789,-0.0000005,1e20,1e-20,width===4?3.4028234663852886e38:Number.MAX_VALUE,width===4?2**-149:Number.MIN_VALUE,NaN,NaN,Infinity,-Infinity]);
        if(width===4){const bits=new Uint32Array(y.buffer);bits[8]=0x7fc00000;bits[9]=0xffc00000;}else{const bits=new BigUint64Array(y.buffer);bits[8]=0x7ff8000000000000n;bits[9]=0xfff8000000000000n;}
        const x=new T(Array.from({length:12},(_,i)=>i*0.25-1)),copyY=y.slice(),copyX=x.slice(),commands=new api.PlotCommands(new Uint8Array());
        for(const[index,color]of ['b','r','g','k','c','y','?'].entries()){commands['line_'+suffix](index%2===0?undefined:x,y,api.plot_color_from_character(color));lineCommands++;}
        commands['line_'+suffix](null,new T(),0);lineCommands++;
        for(const[rows,columns,values]of [[2,6,y],[0,3,new T()],[2,0,new T()]]){commands['image_'+suffix](rows,columns,values);imageCommands++;}
        const expected=new Uint8Array(await fixture('plotting-'+(width===4?'float':'double')+'.txt')),actual=commands.bytes;same(actual,expected,'complete exact original POSIX transcript');transcriptBytes+=actual.length;
        const bitView=a=>width===4?new Uint32Array(a.buffer,a.byteOffset,a.length):new BigUint64Array(a.buffer,a.byteOffset,a.length);same(bitView(y),bitView(copyY),'line/image input payload unchanged');same(x,copyX,'x unchanged');
        const clone=commands.cloned();commands.bytes=new Uint8Array([0,255,128]);commands.append(new Uint8Array([1,0]));same(commands.bytes,new Uint8Array([0,255,128,1,0]),'complete arbitrary writer storage');same(clone.into_inner(),expected,'consuming command stream');throws(()=>clone.bytes,'consumed command owner');commands.free();same(actual,expected,'byte snapshot lifetime');
        const guarded=new api.PlotCommands(text.encode("set title 'custom'\n")),snapshot=guarded.bytes;
        for(const code of [6,0xffffffff])throws(()=>guarded['line_'+suffix](undefined,new T([1]),code),'invalid color');
        throws(()=>guarded['line_'+suffix](new T([0]),new T([1,2]),1),'short x');
        for(const[rows,columns,values]of [[1,2,new T([1])],[0,0x80000000,new T()],[0x80000000,0,new T()],[0xffffffff,0xffffffff,new T()]])throws(()=>guarded['image_'+suffix](rows,columns,values),'invalid shape/dimension');
        same(guarded.bytes,snapshot,'invalid commands leave output unchanged');guarded['line_'+suffix](new T([4,9]),new T([2]),1);same(guarded.bytes,text.encode("set title 'custom'\nplot '-' with lines lc rgb \"red\"\n4.000000 2.000000\ne\n"),'extra x ignored/recovery');
        guarded.bytes=new Uint8Array();guarded['line_'+suffix](undefined,new T([0,1,-1]),0);guarded['image_'+suffix](1,2,new T([1e-7,1e100]));check(guarded.bytes.length>0,'command reuse');guarded.free();
    }
    for(const[color,code]of [['b',0],['r',1],['g',2],['k',3],['c',4],['y',5],['?',0],['R',0],['😀',0]])check(api.plot_color_from_character(color)===code,'native char color mapping');
    for(const value of ['', 'rr','😀b'])throws(()=>api.plot_color_from_character(value),'one Unicode scalar required');
    return{transcriptBytes,lineCommands,imageCommands};
}
