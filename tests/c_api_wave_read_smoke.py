"""Call all WAVE decoding functions with actual stream callbacks and files."""
import ctypes as C
import io
import os
import pathlib
import tempfile
import sys
lib=C.CDLL(str(pathlib.Path(sys.argv[1]).resolve()))
read_type=C.CFUNCTYPE(C.c_uint32,C.c_void_p,C.POINTER(C.c_uint8),C.c_size_t,C.POINTER(C.c_size_t))
seek_type=C.CFUNCTYPE(C.c_uint32,C.c_void_p,C.c_uint32,C.c_uint64,C.c_int64,C.POINTER(C.c_uint64))
class Stream(C.Structure):
    _fields_=[("context",C.c_void_p),("read",read_type),("seek",seek_type)]
class Path(C.Structure):
    _fields_=[("bytes",C.POINTER(C.c_uint8)),("byte_count",C.c_size_t),("encoding",C.c_uint32)]
class Info(C.Structure):
    _fields_=[("sample_rate",C.c_uint32),("bits_per_sample",C.c_uint16),("channels",C.c_uint16),("encoding",C.c_uint32),("sample_count",C.c_size_t)]
wave_bytes=bytes([82, 73, 70, 70, 44, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 128, 62, 0, 0, 128, 62, 0, 0, 1, 0, 8, 0, 100, 97, 116, 97, 7, 0, 0, 0, 0, 64, 127, 128, 129, 160, 255, 0])
prefixed=bytes([13,14,15])+wave_bytes
raw=(C.c_uint8*len(prefixed)).from_buffer_copy(prefixed)
with tempfile.NamedTemporaryFile(prefix="wave-c-python-",suffix=".wav",delete=False) as target:
    target.write(wave_bytes);filename=target.name
try:
    scalar=C.c_float;pointer=C.POINTER(scalar);slot=C.POINTER(C.c_void_p)
    def bind(name,args):
        fn=getattr(lib,"ciglet_rs_wave_"+name+"_f32");fn.argtypes=args;fn.restype=C.c_uint32;return fn
    decode=bind("decode",[C.POINTER(C.c_uint8),C.c_size_t,C.c_uint64,C.c_size_t,slot])
    read_stream=bind("read_stream",[Stream,C.c_size_t,slot]);read_file=bind("read_file",[Path,C.c_size_t,slot])
    info=bind("info",[C.c_void_p,C.POINTER(Info)]);copy=bind("copy_samples",[C.c_void_p,pointer,C.c_size_t])
    clone=bind("clone",[C.c_void_p,slot]);release=bind("release",[slot])
    owner=C.c_void_p();cloned=C.c_void_p();metadata=Info();samples=(scalar*7)()
    assert decode(raw,len(prefixed),3,7,C.byref(owner))==0
    assert info(owner,C.byref(metadata))==0
    assert (metadata.sample_rate,metadata.bits_per_sample,metadata.channels,metadata.encoding,metadata.sample_count)==(16000,8,1,0,7)
    assert clone(owner,C.byref(cloned))==0 and release(C.byref(owner))==0
    assert copy(cloned,samples,7)==0 and list(samples)==[-1,-0.5,-1/128,0,1/128,0.25,127/128]
    assert copy(cloned,samples,6)==2 and release(C.byref(cloned))==0
    reader=io.BytesIO(prefixed);reader.seek(3);counts=[0,0];failure=[0]
    @read_type
    def read_callback(context,output,capacity,count):
        counts[0]+=1
        if failure[0]==1:return 9
        if failure[0]==2:count[0]=capacity+1;return 0
        if failure[0]==3:count[0]=0;return 0
        value=reader.read(min(capacity,1));C.memmove(output,value,len(value));count[0]=len(value);return 0
    @seek_type
    def seek_callback(context,origin,absolute,relative,position):
        counts[1]+=1
        if failure[0]==4:return 9
        position[0]=reader.seek(absolute if origin==0 else relative,origin);return 0
    stream=Stream(None,read_callback,seek_callback)
    assert read_stream(stream,7,C.byref(owner))==0 and counts[0]>10 and counts[1]>0
    assert copy(owner,samples,7)==0 and list(samples)==[-1,-0.5,-1/128,0,1/128,0.25,127/128]
    assert release(C.byref(owner))==0
    for value in range(1,5):
        reader.seek(3);failure[0]=value
        assert read_stream(stream,7,C.byref(owner))==3 and not owner.value
    failure[0]=0;previous=counts.copy()
    assert read_stream(stream,7,None)==1 and counts==previous
    for encoding,value in [(0,filename.encode("utf-8")),(1,filename.encode("utf-16le") if os.name=="nt" else os.fsencode(filename))]:
        path_bytes=(C.c_uint8*len(value)).from_buffer_copy(value);path=Path(path_bytes,len(value),encoding)
        assert read_file(path,7,C.byref(owner))==0 and release(C.byref(owner))==0
    assert decode(raw,len(prefixed),3,6,C.byref(owner))==3 and not owner.value
    scalar=C.c_double;pointer=C.POINTER(scalar);slot=C.POINTER(C.c_void_p)
    def bind(name,args):
        fn=getattr(lib,"ciglet_rs_wave_"+name+"_f64");fn.argtypes=args;fn.restype=C.c_uint32;return fn
    decode=bind("decode",[C.POINTER(C.c_uint8),C.c_size_t,C.c_uint64,C.c_size_t,slot])
    read_stream=bind("read_stream",[Stream,C.c_size_t,slot]);read_file=bind("read_file",[Path,C.c_size_t,slot])
    info=bind("info",[C.c_void_p,C.POINTER(Info)]);copy=bind("copy_samples",[C.c_void_p,pointer,C.c_size_t])
    clone=bind("clone",[C.c_void_p,slot]);release=bind("release",[slot])
    owner=C.c_void_p();cloned=C.c_void_p();metadata=Info();samples=(scalar*7)()
    assert decode(raw,len(prefixed),3,7,C.byref(owner))==0
    assert info(owner,C.byref(metadata))==0
    assert (metadata.sample_rate,metadata.bits_per_sample,metadata.channels,metadata.encoding,metadata.sample_count)==(16000,8,1,0,7)
    assert clone(owner,C.byref(cloned))==0 and release(C.byref(owner))==0
    assert copy(cloned,samples,7)==0 and list(samples)==[-1,-0.5,-1/128,0,1/128,0.25,127/128]
    assert copy(cloned,samples,6)==2 and release(C.byref(cloned))==0
    reader=io.BytesIO(prefixed);reader.seek(3);counts=[0,0];failure=[0]
    @read_type
    def read_callback(context,output,capacity,count):
        counts[0]+=1
        if failure[0]==1:return 9
        if failure[0]==2:count[0]=capacity+1;return 0
        if failure[0]==3:count[0]=0;return 0
        value=reader.read(min(capacity,1));C.memmove(output,value,len(value));count[0]=len(value);return 0
    @seek_type
    def seek_callback(context,origin,absolute,relative,position):
        counts[1]+=1
        if failure[0]==4:return 9
        position[0]=reader.seek(absolute if origin==0 else relative,origin);return 0
    stream=Stream(None,read_callback,seek_callback)
    assert read_stream(stream,7,C.byref(owner))==0 and counts[0]>10 and counts[1]>0
    assert copy(owner,samples,7)==0 and list(samples)==[-1,-0.5,-1/128,0,1/128,0.25,127/128]
    assert release(C.byref(owner))==0
    for value in range(1,5):
        reader.seek(3);failure[0]=value
        assert read_stream(stream,7,C.byref(owner))==3 and not owner.value
    failure[0]=0;previous=counts.copy()
    assert read_stream(stream,7,None)==1 and counts==previous
    for encoding,value in [(0,filename.encode("utf-8")),(1,filename.encode("utf-16le") if os.name=="nt" else os.fsencode(filename))]:
        path_bytes=(C.c_uint8*len(value)).from_buffer_copy(value);path=Path(path_bytes,len(value),encoding)
        assert read_file(path,7,C.byref(owner))==0 and release(C.byref(owner))==0
    assert decode(raw,len(prefixed),3,6,C.byref(owner))==3 and not owner.value
finally:
    os.unlink(filename)
print("WAVE reader C ABI Python checks passed")
