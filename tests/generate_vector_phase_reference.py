"""Generate unmodified C vector and phase helper references."""
import argparse
import os
from pathlib import Path
import subprocess
import tempfile

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--upstream',type=Path,required=True)
    parser.add_argument('--cc',default='gcc')
    parser.add_argument('--zig',action='store_true')
    parser.add_argument('--check',action='store_true')
    args=parser.parse_args();upstream=args.upstream.resolve();tests=Path(__file__).resolve().parent
    with tempfile.TemporaryDirectory(prefix='vector-phase-reference-') as temporary:
        root=Path(temporary);(root/'external').mkdir()
        (root/'ciglet.h').write_text((upstream/'ciglet.h').read_text())
        (root/'external/fastapprox-all.h').write_text((upstream/'external/fastapprox-all.h').read_text())
        (root/'oracle.c').write_text((tests/'reference/vector_phase_oracle.c').read_text())
        for precision in ['float','double']:
            executable=root/(precision+('.exe' if os.name=='nt' else ''));output=root/(precision+'.bin')
            compiler=[args.cc,'cc','-target','x86_64-windows-gnu'] if args.zig else [args.cc]
            subprocess.run(compiler+['-O2','-DFP_TYPE='+precision,'-I'+str(root),str(root/'oracle.c'),'-lm','-o',str(executable)],check=True,timeout=120)
            subprocess.run([str(executable),str(output)],check=True,timeout=30)
            target=tests/'fixtures'/('vector-phase-'+precision+'.bin');content=output.read_bytes()
            if args.check:
                if target.read_bytes()!=content:raise ValueError(f'C helper reference changed: {precision}')
            else:target.write_bytes(content)
            print(precision,'unmodified C vector/phase reference verified')

if __name__=='__main__':main()
