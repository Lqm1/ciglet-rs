"""Call all source scalar operations through the actual shared library."""
import ctypes as C
import pathlib
import struct
import sys
lib = C.CDLL(str(pathlib.Path(sys.argv[1]).resolve()))
scalar=C.c_float
pointer=C.POINTER(scalar)
output=scalar(71)
for name,first,second,expected in [("add",2,3,5),("accumulate_square",5,-3,14),("minimum",-5,7,-5),("maximum",-5,7,7)]:
    operation=getattr(lib,"ciglet_rs_scalar_"+name+"_f32")
    operation.argtypes=[scalar,scalar,pointer]
    operation.restype=C.c_uint32
    assert operation(first,second,C.byref(output))==0 and output.value==expected
    assert operation(1,2,None)==1
callback_type=C.CFUNCTYPE(scalar,scalar,scalar,C.c_void_p)
calls=C.c_size_t(0)
@callback_type
def ordered(acc,value,context):
    C.cast(context,C.POINTER(C.c_size_t)).contents.value+=1
    return acc*10+value
fold=lib.ciglet_rs_scalar_fold_f32
fold.argtypes=[pointer,C.c_size_t,scalar,callback_type,C.c_void_p,pointer]
fold.restype=C.c_uint32
inputs=(scalar*3)(1,2,3)
assert fold(inputs,3,4,ordered,C.byref(calls),C.byref(output))==0 and output.value==4123 and calls.value==3
assert fold(inputs,3,4,ordered,C.byref(calls),inputs)==0 and inputs[0]==4123 and calls.value==6
assert fold(None,0,-0.0,callback_type(),None,C.byref(output))==0
assert struct.pack("f",output.value)==struct.pack("f",-0.0)
output.value=71
assert fold(inputs,3,4,callback_type(),None,C.byref(output))==1 and output.value==71
scalar=C.c_double
pointer=C.POINTER(scalar)
output=scalar(71)
for name,first,second,expected in [("add",2,3,5),("accumulate_square",5,-3,14),("minimum",-5,7,-5),("maximum",-5,7,7)]:
    operation=getattr(lib,"ciglet_rs_scalar_"+name+"_f64")
    operation.argtypes=[scalar,scalar,pointer]
    operation.restype=C.c_uint32
    assert operation(first,second,C.byref(output))==0 and output.value==expected
    assert operation(1,2,None)==1
callback_type=C.CFUNCTYPE(scalar,scalar,scalar,C.c_void_p)
calls=C.c_size_t(0)
@callback_type
def ordered(acc,value,context):
    C.cast(context,C.POINTER(C.c_size_t)).contents.value+=1
    return acc*10+value
fold=lib.ciglet_rs_scalar_fold_f64
fold.argtypes=[pointer,C.c_size_t,scalar,callback_type,C.c_void_p,pointer]
fold.restype=C.c_uint32
inputs=(scalar*3)(1,2,3)
assert fold(inputs,3,4,ordered,C.byref(calls),C.byref(output))==0 and output.value==4123 and calls.value==3
assert fold(inputs,3,4,ordered,C.byref(calls),inputs)==0 and inputs[0]==4123 and calls.value==6
assert fold(None,0,-0.0,callback_type(),None,C.byref(output))==0
assert struct.pack("d",output.value)==struct.pack("d",-0.0)
output.value=71
assert fold(inputs,3,4,callback_type(),None,C.byref(output))==1 and output.value==71
class Constants(C.Structure):
    _fields_=[(name,C.c_double) for name in ("epsilon","infinity_sentinel","pi","half_pi")]
lib.ciglet_rs_math_constants.argtypes=[C.POINTER(Constants)]
lib.ciglet_rs_math_constants.restype=C.c_uint32
constants=Constants()
assert C.sizeof(constants)==32 and Constants.half_pi.offset==24
assert lib.ciglet_rs_math_constants(C.byref(constants))==0
assert lib.ciglet_rs_math_constants(None)==1
assert struct.unpack("=4Q",bytes(constants))==(0x3cd203af9ee75616,0x430c6bf526340000,0x400921fb54442d18,0x3ff921fb54442d18)
print("Scalar C ABI Python checks passed")
