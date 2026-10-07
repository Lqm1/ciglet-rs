"""Generate references from the unchanged C instantaneous-frequency function bodies."""
import argparse
import os
import struct
from pathlib import Path
import subprocess
import tempfile

def compare_double(expected, actual):
    if len(expected) != len(actual) or expected[:12] != actual[:12]:
        raise ValueError('C reference structure changed')
    cursor = 12
    maximum_kernel_error = maximum_estimate_error = 0.0
    def exact(size):
        nonlocal cursor
        if expected[cursor:cursor+size] != actual[cursor:cursor+size]:
            raise ValueError('C reference inputs or dimensions changed')
        cursor += size
    def compare(tolerance):
        nonlocal cursor
        first, = struct.unpack_from('<d', expected, cursor)
        second, = struct.unpack_from('<d', actual, cursor)
        cursor += 8
        error = abs(first-second)/(1+abs(first))
        if not error <= tolerance:
            raise ValueError('C libm output changed beyond measured bound')
        return error
    for _ in range(24):
        exact(16)
        length, = struct.unpack_from('<I', expected, cursor)
        exact(4)
        for _ in range(4*length):
            maximum_kernel_error = max(maximum_kernel_error, compare(3e-15))
        for _ in range(4):
            nx, = struct.unpack_from('<I', expected, cursor)
            exact(4 + 8*nx)
            maximum_estimate_error = max(maximum_estimate_error, compare(1e-12))
    if cursor != len(expected):
        raise ValueError('C reference has trailing data')
    print('double cross-host maximum kernel/estimate normalized errors', maximum_kernel_error, maximum_estimate_error)

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--upstream',type=Path,required=True)
    parser.add_argument('--cc',default='gcc')
    parser.add_argument('--zig',action='store_true')
    parser.add_argument('--check',action='store_true')
    args=parser.parse_args()
    tests=Path(__file__).resolve().parent
    source=(args.upstream/'ciglet.c').read_text()
    bodies=[]
    for name in ['cig_create_ifdetector','cig_delete_ifdetector','cig_ifdetector_estimate']:
        signature = {'cig_create_ifdetector': 'ifdetector* ', 'cig_delete_ifdetector': 'void ', 'cig_ifdetector_estimate': 'FP_TYPE '}[name]
        start=source.index(signature+name+'(')
        end=source.index('\n}',start)+2
        bodies.append(source[start:end])
    with tempfile.TemporaryDirectory(prefix='instantaneous-frequency-reference-') as directory:
        root=Path(directory);(root/'external').mkdir()
        for name in ['ciglet.h','external/fastapprox-all.h']:
            (root/name).write_bytes((args.upstream/name).read_bytes())
        (root/'bodies.c').write_text('#include "ciglet.h"\n'+'\n'.join(bodies))
        (root/'oracle.c').write_bytes((tests/'reference/instantaneous_frequency_oracle.c').read_bytes())
        for precision in ['float','double']:
            executable=root/(precision+('.exe' if os.name=='nt' else ''))
            output=root/(precision+'.bin')
            compiler=[args.cc,'cc','-target','x86_64-windows-gnu'] if args.zig else [args.cc]
            subprocess.run(compiler+['-O2','-DFP_TYPE='+precision,'-I'+str(root),str(root/'oracle.c'),str(root/'bodies.c'),'-lm','-o',str(executable)],check=True,timeout=120)
            subprocess.run([str(executable),str(output)],check=True,timeout=30)
            target=tests/'fixtures'/('instantaneous-frequency-'+precision+'.bin')
            if args.check:
                if precision == 'double': compare_double(target.read_bytes(), output.read_bytes())
                elif target.read_bytes()!=output.read_bytes():raise ValueError('C instantaneous-frequency reference changed: '+precision)
            else:target.write_bytes(output.read_bytes())
            print(precision,'unmodified instantaneous-frequency reference verified')

if __name__=='__main__':main()
