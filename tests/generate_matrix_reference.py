"""Generate original and corrected C matrix references for both precisions."""
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
    args=parser.parse_args()
    upstream=args.upstream.resolve();tests=Path(__file__).resolve().parent
    with tempfile.TemporaryDirectory(prefix='matrix-reference-') as temporary:
        root=Path(temporary)
        source=(upstream/'ciglet.c').read_text()
        start=source.index('void cig_permm(');end=source.index('void cig_lu(',start)
        corrected=source[:start]+source[start:end].replace('j * n','j * m')+source[end:]
        (root/'ciglet.c').write_text(corrected)
        header=(upstream/'ciglet.h').read_text().replace('cig_mvecmul(A, x, b, n, n);','cig_mvecmul(A, x, b, m, n);')
        (root/'ciglet.h').write_text(header)
        for mode in ['legacy','corrected']:
            for precision in ['float','double']:
                executable=root/(mode+'-'+precision+('.exe' if os.name=='nt' else ''))
                output=root/(mode+'-'+precision+'.bin')
                compiler=[args.cc,'cc','-target','x86_64-windows-gnu'] if args.zig else [args.cc]
                patched=mode=='corrected'
                options=['-DCORRECTED'] if patched else []
                include=root if patched else upstream
                dependencies=[str(upstream/'external/fftsg_h.c'),str(upstream/'external/fast_median.c')] if os.name=='nt' else []
                linker=[] if os.name=='nt' else ['-Wl,--gc-sections']
                subprocess.run(compiler+['-O2','-DFP_TYPE='+precision,'-ffunction-sections','-fdata-sections',
                    '-I'+str(include),'-I'+str(upstream),str(tests/'reference/matrix_oracle.c'),
                    str(include/'ciglet.c'),'-lm','-o',str(executable)]+options+dependencies+linker,check=True,timeout=120)
                subprocess.run([str(executable),str(output)],check=True,timeout=30)
                target=tests/'fixtures'/('matrix-'+mode+'-'+precision+'.bin')
                data=output.read_bytes()
                if args.check:
                    if target.read_bytes()!=data:raise ValueError(f'C reference changed: {mode} {precision}')
                else:target.write_bytes(data)
                print(f'{mode} {precision}: C reference verified')

if __name__=='__main__':main()
