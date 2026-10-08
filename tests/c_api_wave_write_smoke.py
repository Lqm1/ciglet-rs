"""Exercise original byte oracles, borrowed writer callbacks and real files."""
import ctypes as C
import os
import pathlib
import struct
import tempfile
import sys
lib=C.CDLL(str(pathlib.Path(sys.argv[1]).resolve()))
write_type=C.CFUNCTYPE(C.c_uint32,C.c_void_p,C.POINTER(C.c_uint8),C.c_size_t,C.POINTER(C.c_size_t))
flush_type=C.CFUNCTYPE(C.c_uint32,C.c_void_p)
class Stream(C.Structure):
    _fields_=[("context",C.c_void_p),("write",write_type),("flush",flush_type)]
class Path(C.Structure):
    _fields_=[("bytes",C.POINTER(C.c_uint8)),("byte_count",C.c_size_t),("encoding",C.c_uint32)]
class Options(C.Structure):
    _fields_=[("bits_per_sample",C.c_uint16),("pcm8_quantization",C.c_uint32),("pad_odd_data",C.c_uint32)]
def bind(name,args):
    fn=getattr(lib,"ciglet_rs_"+name);fn.argtypes=args;fn.restype=C.c_uint32;return fn
slot=C.POINTER(C.c_void_p);byte_pointer=C.POINTER(C.c_uint8)
default=bind("wave_write_options_default",[C.POINTER(Options)])
legacy=bind("wave_write_options_legacy",[C.c_uint16,C.POINTER(Options)])
length=bind("bytes_length",[C.c_void_p,C.POINTER(C.c_size_t)])
copy=bind("bytes_copy",[C.c_void_p,C.c_size_t,byte_pointer,C.c_size_t])
clone=bind("bytes_clone",[C.c_void_p,slot]);release=bind("bytes_release",[slot])
options=Options();assert C.sizeof(options)==12
assert default(C.byref(options))==0 and (options.bits_per_sample,options.pcm8_quantization,options.pad_odd_data)==(16,0,1)
cases={'f32': [{'bits': 8, 'rate': 22050, 'sample_bits': (), 'expected': [82, 73, 70, 70, 36, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 34, 86, 0, 0, 34, 86, 0, 0, 1, 0, 8, 0, 100, 97, 116, 97, 0, 0, 0, 0]}, {'bits': 8, 'rate': 22050, 'sample_bits': (3210739712, 3204448256, 3187671040, 3072837036, 0, 1040187392, 1056964608, 1063256064), 'expected': [82, 73, 70, 70, 44, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 34, 86, 0, 0, 34, 86, 0, 0, 1, 0, 8, 0, 100, 97, 116, 97, 8, 0, 0, 0, 15, 63, 111, 126, 127, 142, 190, 238]}, {'bits': 8, 'rate': 22050, 'sample_bits': (3210739712, 3204448256, 3187671040, 3072837036, 0, 1040187392, 1056964608, 1063256064, 3210739712), 'expected': [82, 73, 70, 70, 45, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 34, 86, 0, 0, 34, 86, 0, 0, 1, 0, 8, 0, 100, 97, 116, 97, 9, 0, 0, 0, 15, 63, 111, 126, 127, 142, 190, 238, 15]}, {'bits': 16, 'rate': 22050, 'sample_bits': (), 'expected': [82, 73, 70, 70, 36, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 34, 86, 0, 0, 68, 172, 0, 0, 2, 0, 16, 0, 100, 97, 116, 97, 0, 0, 0, 0]}, {'bits': 16, 'rate': 22050, 'sample_bits': (3210739712, 3204448256, 3187671040, 3072837036, 0, 1040187392, 1056964608, 1063256064), 'expected': [82, 73, 70, 70, 52, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 34, 86, 0, 0, 68, 172, 0, 0, 2, 0, 16, 0, 100, 97, 116, 97, 16, 0, 0, 0, 1, 144, 1, 192, 1, 240, 0, 0, 0, 0, 255, 15, 255, 63, 255, 111]}, {'bits': 16, 'rate': 22050, 'sample_bits': (3210739712, 3204448256, 3187671040, 3072837036, 0, 1040187392, 1056964608, 1063256064, 3210739712), 'expected': [82, 73, 70, 70, 54, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 34, 86, 0, 0, 68, 172, 0, 0, 2, 0, 16, 0, 100, 97, 116, 97, 18, 0, 0, 0, 1, 144, 1, 192, 1, 240, 0, 0, 0, 0, 255, 15, 255, 63, 255, 111, 1, 144]}, {'bits': 24, 'rate': 22050, 'sample_bits': (), 'expected': [82, 73, 70, 70, 36, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 34, 86, 0, 0, 102, 2, 1, 0, 3, 0, 24, 0, 100, 97, 116, 97, 0, 0, 0, 0]}, {'bits': 24, 'rate': 22050, 'sample_bits': (3210739712, 3204448256, 3187671040, 3072837036, 0, 1040187392, 1056964608, 1063256064), 'expected': [82, 73, 70, 70, 60, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 34, 86, 0, 0, 102, 2, 1, 0, 3, 0, 24, 0, 100, 97, 116, 97, 24, 0, 0, 0, 1, 0, 144, 1, 0, 192, 1, 0, 240, 173, 255, 255, 0, 0, 0, 255, 255, 15, 255, 255, 63, 255, 255, 111]}, {'bits': 24, 'rate': 22050, 'sample_bits': (3210739712, 3204448256, 3187671040, 3072837036, 0, 1040187392, 1056964608, 1063256064, 3210739712), 'expected': [82, 73, 70, 70, 63, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 34, 86, 0, 0, 102, 2, 1, 0, 3, 0, 24, 0, 100, 97, 116, 97, 27, 0, 0, 0, 1, 0, 144, 1, 0, 192, 1, 0, 240, 173, 255, 255, 0, 0, 0, 255, 255, 15, 255, 255, 63, 255, 255, 111, 1, 0, 144]}, {'bits': 32, 'rate': 22050, 'sample_bits': (), 'expected': [82, 73, 70, 70, 36, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 34, 86, 0, 0, 136, 88, 1, 0, 4, 0, 32, 0, 100, 97, 116, 97, 0, 0, 0, 0]}, {'bits': 32, 'rate': 22050, 'sample_bits': (3210739712, 3204448256, 3187671040, 3072837036, 0, 1040187392, 1056964608, 1063256064), 'expected': [82, 73, 70, 70, 68, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 34, 86, 0, 0, 136, 88, 1, 0, 4, 0, 32, 0, 100, 97, 116, 97, 32, 0, 0, 0, 0, 0, 0, 144, 0, 0, 0, 192, 0, 0, 0, 240, 30, 172, 255, 255, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 64, 0, 0, 0, 112]}, {'bits': 32, 'rate': 22050, 'sample_bits': (3210739712, 3204448256, 3187671040, 3072837036, 0, 1040187392, 1056964608, 1063256064, 3210739712), 'expected': [82, 73, 70, 70, 72, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 34, 86, 0, 0, 136, 88, 1, 0, 4, 0, 32, 0, 100, 97, 116, 97, 36, 0, 0, 0, 0, 0, 0, 144, 0, 0, 0, 192, 0, 0, 0, 240, 30, 172, 255, 255, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 64, 0, 0, 0, 112, 0, 0, 0, 144]}], 'f64': [{'bits': 8, 'rate': 22050, 'sample_bits': (), 'expected': [82, 73, 70, 70, 36, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 34, 86, 0, 0, 34, 86, 0, 0, 1, 0, 8, 0, 100, 97, 116, 97, 0, 0, 0, 0]}, {'bits': 8, 'rate': 22050, 'sample_bits': (13829428555747950592, 13826050856027422720, 13817043656772681728, 13755392620465711345, 0, 4593671619917905920, 4602678819172646912, 4606056518893174784), 'expected': [82, 73, 70, 70, 44, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 34, 86, 0, 0, 34, 86, 0, 0, 1, 0, 8, 0, 100, 97, 116, 97, 8, 0, 0, 0, 15, 63, 111, 126, 127, 142, 190, 238]}, {'bits': 8, 'rate': 22050, 'sample_bits': (13829428555747950592, 13826050856027422720, 13817043656772681728, 13755392620465711345, 0, 4593671619917905920, 4602678819172646912, 4606056518893174784, 13829428555747950592), 'expected': [82, 73, 70, 70, 45, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 34, 86, 0, 0, 34, 86, 0, 0, 1, 0, 8, 0, 100, 97, 116, 97, 9, 0, 0, 0, 15, 63, 111, 126, 127, 142, 190, 238, 15]}, {'bits': 16, 'rate': 22050, 'sample_bits': (), 'expected': [82, 73, 70, 70, 36, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 34, 86, 0, 0, 68, 172, 0, 0, 2, 0, 16, 0, 100, 97, 116, 97, 0, 0, 0, 0]}, {'bits': 16, 'rate': 22050, 'sample_bits': (13829428555747950592, 13826050856027422720, 13817043656772681728, 13755392620465711345, 0, 4593671619917905920, 4602678819172646912, 4606056518893174784), 'expected': [82, 73, 70, 70, 52, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 34, 86, 0, 0, 68, 172, 0, 0, 2, 0, 16, 0, 100, 97, 116, 97, 16, 0, 0, 0, 1, 144, 1, 192, 1, 240, 0, 0, 0, 0, 255, 15, 255, 63, 255, 111]}, {'bits': 16, 'rate': 22050, 'sample_bits': (13829428555747950592, 13826050856027422720, 13817043656772681728, 13755392620465711345, 0, 4593671619917905920, 4602678819172646912, 4606056518893174784, 13829428555747950592), 'expected': [82, 73, 70, 70, 54, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 34, 86, 0, 0, 68, 172, 0, 0, 2, 0, 16, 0, 100, 97, 116, 97, 18, 0, 0, 0, 1, 144, 1, 192, 1, 240, 0, 0, 0, 0, 255, 15, 255, 63, 255, 111, 1, 144]}, {'bits': 24, 'rate': 22050, 'sample_bits': (), 'expected': [82, 73, 70, 70, 36, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 34, 86, 0, 0, 102, 2, 1, 0, 3, 0, 24, 0, 100, 97, 116, 97, 0, 0, 0, 0]}, {'bits': 24, 'rate': 22050, 'sample_bits': (13829428555747950592, 13826050856027422720, 13817043656772681728, 13755392620465711345, 0, 4593671619917905920, 4602678819172646912, 4606056518893174784), 'expected': [82, 73, 70, 70, 60, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 34, 86, 0, 0, 102, 2, 1, 0, 3, 0, 24, 0, 100, 97, 116, 97, 24, 0, 0, 0, 1, 0, 144, 1, 0, 192, 1, 0, 240, 173, 255, 255, 0, 0, 0, 255, 255, 15, 255, 255, 63, 255, 255, 111]}, {'bits': 24, 'rate': 22050, 'sample_bits': (13829428555747950592, 13826050856027422720, 13817043656772681728, 13755392620465711345, 0, 4593671619917905920, 4602678819172646912, 4606056518893174784, 13829428555747950592), 'expected': [82, 73, 70, 70, 63, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 34, 86, 0, 0, 102, 2, 1, 0, 3, 0, 24, 0, 100, 97, 116, 97, 27, 0, 0, 0, 1, 0, 144, 1, 0, 192, 1, 0, 240, 173, 255, 255, 0, 0, 0, 255, 255, 15, 255, 255, 63, 255, 255, 111, 1, 0, 144]}, {'bits': 32, 'rate': 22050, 'sample_bits': (), 'expected': [82, 73, 70, 70, 36, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 34, 86, 0, 0, 136, 88, 1, 0, 4, 0, 32, 0, 100, 97, 116, 97, 0, 0, 0, 0]}, {'bits': 32, 'rate': 22050, 'sample_bits': (13829428555747950592, 13826050856027422720, 13817043656772681728, 13755392620465711345, 0, 4593671619917905920, 4602678819172646912, 4606056518893174784), 'expected': [82, 73, 70, 70, 68, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 34, 86, 0, 0, 136, 88, 1, 0, 4, 0, 32, 0, 100, 97, 116, 97, 32, 0, 0, 0, 1, 0, 0, 144, 1, 0, 0, 192, 1, 0, 0, 240, 30, 172, 255, 255, 0, 0, 0, 0, 255, 255, 255, 15, 255, 255, 255, 63, 255, 255, 255, 111]}, {'bits': 32, 'rate': 22050, 'sample_bits': (13829428555747950592, 13826050856027422720, 13817043656772681728, 13755392620465711345, 0, 4593671619917905920, 4602678819172646912, 4606056518893174784, 13829428555747950592), 'expected': [82, 73, 70, 70, 72, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 34, 86, 0, 0, 136, 88, 1, 0, 4, 0, 32, 0, 100, 97, 116, 97, 36, 0, 0, 0, 1, 0, 0, 144, 1, 0, 0, 192, 1, 0, 0, 240, 30, 172, 255, 255, 0, 0, 0, 0, 255, 255, 255, 15, 255, 255, 255, 63, 255, 255, 255, 111, 1, 0, 0, 144]}]}

scalar=C.c_float;pointer=C.POINTER(scalar)
encode=bind("wave_encode_f32",[pointer,C.c_size_t,C.c_uint32,Options,slot])
write_stream=bind("wave_write_stream_f32",[Stream,pointer,C.c_size_t,C.c_uint32,Options])
write_file=bind("wave_write_file_f32",[Path,pointer,C.c_size_t,C.c_uint32,Options])
with tempfile.TemporaryDirectory(prefix="wave-c-writer-") as directory:
    for index,case in enumerate(cases["f32"]):
        floats=[struct.unpack("<f",struct.pack("<I",v))[0] for v in case["sample_bits"]]
        samples=(scalar*len(floats))(*floats);expected=bytes(case["expected"])
        assert legacy(case["bits"],C.byref(options))==0
        owner=C.c_void_p();cloned=C.c_void_p();size=C.c_size_t()
        assert encode(samples,len(samples),case["rate"],options,C.byref(owner))==0
        assert length(owner,C.byref(size))==0 and size.value==len(expected)
        assert clone(owner,C.byref(cloned))==0 and release(C.byref(owner))==0
        result=(C.c_uint8*size.value)()
        assert copy(cloned,0,result,size.value)==0 and bytes(result)==expected
        assert copy(cloned,size.value,None,0)==0
        C.memset(result,71,size.value)
        assert copy(cloned,1,result,size.value)==2 and bytes(result)==bytes([71])*size.value
        assert copy(cloned,1,result,size.value-1)==0 and bytes(result)[:-1]==expected[1:]
        assert release(C.byref(cloned))==0
        sink=bytearray();counts=[0,0];failure=[0]
        @write_type
        def write_callback(context,source,count,written):
            counts[0]+=1
            if failure[0]==1:return 9
            if failure[0]==2:written[0]=count+1;return 0
            if failure[0]==3:written[0]=0;return 0
            if failure[0]==4 and len(sink)>=8:return 9
            n=min(count,1);sink.extend(C.string_at(source,n));written[0]=n;return 0
        @flush_type
        def flush_callback(context):
            counts[1]+=1;return 0
        stream=Stream(None,write_callback,flush_callback)
        assert write_stream(stream,samples,len(samples),case["rate"],options)==0 and bytes(sink)==expected and counts[1]==0
        filename=os.path.join(directory,"output-"+str(index)+".wav")
        for encoding,path_bytes in [(0,filename.encode("utf-8")),(1,filename.encode("utf-16le") if os.name=="nt" else os.fsencode(filename))]:
            path_raw=(C.c_uint8*len(path_bytes)).from_buffer_copy(path_bytes);path=Path(path_raw,len(path_bytes),encoding)
            assert write_file(path,samples,len(samples),case["rate"],options)==0 and pathlib.Path(filename).read_bytes()==expected
        for value in range(1,5):
            failure[0]=value;sink.clear();counts[:]=[0,0]
            assert write_stream(stream,samples,len(samples),case["rate"],options)==3 and len(sink)==(8 if value==4 else 0) and counts[1]==0
        failure[0]=0;sink.clear();counts[:]=[0,0];options.pad_odd_data=2
        assert write_stream(stream,samples,len(samples),case["rate"],options)==3 and counts==[0,0]

scalar=C.c_double;pointer=C.POINTER(scalar)
encode=bind("wave_encode_f64",[pointer,C.c_size_t,C.c_uint32,Options,slot])
write_stream=bind("wave_write_stream_f64",[Stream,pointer,C.c_size_t,C.c_uint32,Options])
write_file=bind("wave_write_file_f64",[Path,pointer,C.c_size_t,C.c_uint32,Options])
with tempfile.TemporaryDirectory(prefix="wave-c-writer-") as directory:
    for index,case in enumerate(cases["f64"]):
        floats=[struct.unpack("<d",struct.pack("<Q",v))[0] for v in case["sample_bits"]]
        samples=(scalar*len(floats))(*floats);expected=bytes(case["expected"])
        assert legacy(case["bits"],C.byref(options))==0
        owner=C.c_void_p();cloned=C.c_void_p();size=C.c_size_t()
        assert encode(samples,len(samples),case["rate"],options,C.byref(owner))==0
        assert length(owner,C.byref(size))==0 and size.value==len(expected)
        assert clone(owner,C.byref(cloned))==0 and release(C.byref(owner))==0
        result=(C.c_uint8*size.value)()
        assert copy(cloned,0,result,size.value)==0 and bytes(result)==expected
        assert copy(cloned,size.value,None,0)==0
        C.memset(result,71,size.value)
        assert copy(cloned,1,result,size.value)==2 and bytes(result)==bytes([71])*size.value
        assert copy(cloned,1,result,size.value-1)==0 and bytes(result)[:-1]==expected[1:]
        assert release(C.byref(cloned))==0
        sink=bytearray();counts=[0,0];failure=[0]
        @write_type
        def write_callback(context,source,count,written):
            counts[0]+=1
            if failure[0]==1:return 9
            if failure[0]==2:written[0]=count+1;return 0
            if failure[0]==3:written[0]=0;return 0
            if failure[0]==4 and len(sink)>=8:return 9
            n=min(count,1);sink.extend(C.string_at(source,n));written[0]=n;return 0
        @flush_type
        def flush_callback(context):
            counts[1]+=1;return 0
        stream=Stream(None,write_callback,flush_callback)
        assert write_stream(stream,samples,len(samples),case["rate"],options)==0 and bytes(sink)==expected and counts[1]==0
        filename=os.path.join(directory,"output-"+str(index)+".wav")
        for encoding,path_bytes in [(0,filename.encode("utf-8")),(1,filename.encode("utf-16le") if os.name=="nt" else os.fsencode(filename))]:
            path_raw=(C.c_uint8*len(path_bytes)).from_buffer_copy(path_bytes);path=Path(path_raw,len(path_bytes),encoding)
            assert write_file(path,samples,len(samples),case["rate"],options)==0 and pathlib.Path(filename).read_bytes()==expected
        for value in range(1,5):
            failure[0]=value;sink.clear();counts[:]=[0,0]
            assert write_stream(stream,samples,len(samples),case["rate"],options)==3 and len(sink)==(8 if value==4 else 0) and counts[1]==0
        failure[0]=0;sink.clear();counts[:]=[0,0];options.pad_odd_data=2
        assert write_stream(stream,samples,len(samples),case["rate"],options)==3 and counts==[0,0]
print("WAVE writer C ABI Python checks passed")
