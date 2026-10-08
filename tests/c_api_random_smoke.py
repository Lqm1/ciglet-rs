"""Exercise all random C functions and original fixture bits via ctypes."""
import ctypes as C
import pathlib
import struct
import sys
lib=C.CDLL(str(pathlib.Path(sys.argv[1]).resolve()))
def bind(name,args):
    fn=getattr(lib,"ciglet_rs_random_"+name);fn.argtypes=args;fn.restype=C.c_uint32;return fn
owner=C.c_void_p();clone=C.c_void_p();slot=C.POINTER(C.c_void_p)
seed=(C.c_uint8*32)(*([37]*32))
create=bind("create",[C.POINTER(C.c_uint8),C.c_size_t,slot])
custom=bind("create_with_maximum",[C.POINTER(C.c_uint8),C.c_size_t,C.c_uint32,slot])
copy=bind("clone",[C.c_void_p,slot]);release=bind("release",[slot])
maximum=bind("maximum",[C.c_void_p,C.POINTER(C.c_uint32)])
draw=bind("draw",[C.c_void_p,C.POINTER(C.c_uint32)])
value=C.c_uint32()
assert create(seed,32,C.byref(owner))==0
assert maximum(owner,C.byref(value))==0 and value.value==2147483647
assert release(C.byref(owner))==0 and not owner.value
assert custom(seed,32,31,C.byref(owner))==0
assert maximum(owner,C.byref(value))==0 and value.value==31
assert copy(owner,C.byref(clone))==0
other=C.c_uint32()
assert draw(owner,C.byref(value))==0 and draw(clone,C.byref(other))==0 and value.value==other.value
assert release(C.byref(clone))==0

scalar=C.c_float;pointer=C.POINTER(scalar)
uniform_draw=bind("uniform_from_draw_f32",[C.c_uint32,C.c_uint32,pointer])
normal_draw=bind("normal_from_draws_f32",[scalar,scalar,C.c_uint32,C.c_uint32,C.c_uint32,C.c_uint32,pointer])
white_draw=bind("white_from_draw_f32",[scalar,C.c_uint32,C.c_uint32,pointer])
white_draws=bind("white_from_draws_f32",[scalar,C.POINTER(C.c_uint32),C.c_size_t,C.c_uint32,pointer])
uniform=bind("uniform_f32",[C.c_void_p,pointer])
normal=bind("normal_f32",[C.c_void_p,scalar,scalar,pointer])
normal_with=bind("normal_with_f32",[C.c_void_p,scalar,scalar,C.c_uint32,pointer])
white=bind("white_noise_f32",[C.c_void_p,scalar,pointer,C.c_size_t])
fmt="f";integer="I"
def scalar_bits(v):return struct.unpack("="+integer,struct.pack("="+fmt,v))[0]
def scalar_value(bits):return struct.unpack("="+fmt,struct.pack("="+integer,bits))[0]
bits=[3212836864, 0, 0, 3212835840, 3212836864, 2147483648, 2147483648]
output=scalar(71);expected=scalar()
assert uniform_draw(1,32767,C.byref(output))==0 and scalar_bits(output.value)==bits[3]
assert normal_draw(scalar_value(bits[0]),scalar_value(bits[1]),1,0,32767,1,C.byref(output))==0 and scalar_bits(output.value)==bits[4]
assert white_draw(scalar_value(bits[2]),1,32767,C.byref(output))==0 and scalar_bits(output.value)==bits[5]
original=(C.c_uint32*2)(1,0);samples=(scalar*7)()
assert white_draws(scalar_value(bits[2]),original,2,32767,samples)==0
assert [scalar_bits(samples[i]) for i in range(2)]==bits[5:7]
assert copy(owner,C.byref(clone))==0
assert uniform(owner,C.byref(output))==0 and draw(clone,C.byref(value))==0
assert uniform_draw(value.value,31,C.byref(expected))==0 and scalar_bits(output.value)==scalar_bits(expected.value)
assert normal(owner,3,0,C.byref(output))==0 and output.value==3
assert draw(clone,C.byref(value))==0 and draw(clone,C.byref(value))==0
assert normal_with(owner,1,2,1,C.byref(output))==0
first=C.c_uint32();second=C.c_uint32()
assert draw(clone,C.byref(first))==0 and draw(clone,C.byref(second))==0
assert normal_draw(1,2,first.value,second.value,31,1,C.byref(expected))==0 and scalar_bits(output.value)==scalar_bits(expected.value)
draws=(C.c_uint32*7)()
for i in range(7):
    assert draw(clone,C.byref(value))==0;draws[i]=value.value
reference=(scalar*7)()
assert white(owner,-2,samples,7)==0 and white_draws(-2,draws,7,31,reference)==0
assert [scalar_bits(v) for v in samples]==[scalar_bits(v) for v in reference]
assert white(owner,1,None,0)==0
output.value=71
assert normal_with(owner,0,1,99,C.byref(output))==3 and output.value==71
assert uniform(owner,None)==1
assert draw(owner,C.byref(value))==0 and draw(clone,C.byref(other))==0 and value.value==other.value
assert release(C.byref(clone))==0

scalar=C.c_double;pointer=C.POINTER(scalar)
uniform_draw=bind("uniform_from_draw_f64",[C.c_uint32,C.c_uint32,pointer])
normal_draw=bind("normal_from_draws_f64",[scalar,scalar,C.c_uint32,C.c_uint32,C.c_uint32,C.c_uint32,pointer])
white_draw=bind("white_from_draw_f64",[scalar,C.c_uint32,C.c_uint32,pointer])
white_draws=bind("white_from_draws_f64",[scalar,C.POINTER(C.c_uint32),C.c_size_t,C.c_uint32,pointer])
uniform=bind("uniform_f64",[C.c_void_p,pointer])
normal=bind("normal_f64",[C.c_void_p,scalar,scalar,pointer])
normal_with=bind("normal_with_f64",[C.c_void_p,scalar,scalar,C.c_uint32,pointer])
white=bind("white_noise_f64",[C.c_void_p,scalar,pointer,C.c_size_t])
fmt="d";integer="Q"
def scalar_bits(v):return struct.unpack("="+integer,struct.pack("="+fmt,v))[0]
def scalar_value(bits):return struct.unpack("="+fmt,struct.pack("="+integer,bits))[0]
bits=[13830554455654793216, 0, 0, 13830553905882201600, 13830554455654793216, 9223372036854775808, 9223372036854775808]
output=scalar(71);expected=scalar()
assert uniform_draw(1,32767,C.byref(output))==0 and scalar_bits(output.value)==bits[3]
assert normal_draw(scalar_value(bits[0]),scalar_value(bits[1]),1,0,32767,1,C.byref(output))==0 and scalar_bits(output.value)==bits[4]
assert white_draw(scalar_value(bits[2]),1,32767,C.byref(output))==0 and scalar_bits(output.value)==bits[5]
original=(C.c_uint32*2)(1,0);samples=(scalar*7)()
assert white_draws(scalar_value(bits[2]),original,2,32767,samples)==0
assert [scalar_bits(samples[i]) for i in range(2)]==bits[5:7]
assert copy(owner,C.byref(clone))==0
assert uniform(owner,C.byref(output))==0 and draw(clone,C.byref(value))==0
assert uniform_draw(value.value,31,C.byref(expected))==0 and scalar_bits(output.value)==scalar_bits(expected.value)
assert normal(owner,3,0,C.byref(output))==0 and output.value==3
assert draw(clone,C.byref(value))==0 and draw(clone,C.byref(value))==0
assert normal_with(owner,1,2,1,C.byref(output))==0
first=C.c_uint32();second=C.c_uint32()
assert draw(clone,C.byref(first))==0 and draw(clone,C.byref(second))==0
assert normal_draw(1,2,first.value,second.value,31,1,C.byref(expected))==0 and scalar_bits(output.value)==scalar_bits(expected.value)
draws=(C.c_uint32*7)()
for i in range(7):
    assert draw(clone,C.byref(value))==0;draws[i]=value.value
reference=(scalar*7)()
assert white(owner,-2,samples,7)==0 and white_draws(-2,draws,7,31,reference)==0
assert [scalar_bits(v) for v in samples]==[scalar_bits(v) for v in reference]
assert white(owner,1,None,0)==0
output.value=71
assert normal_with(owner,0,1,99,C.byref(output))==3 and output.value==71
assert uniform(owner,None)==1
assert draw(owner,C.byref(value))==0 and draw(clone,C.byref(other))==0 and value.value==other.value
assert release(C.byref(clone))==0
assert release(C.byref(owner))==0 and release(C.byref(owner))==0
print("Random C ABI Python checks passed")
