"""Generate C statistics references with only the sort comparator corrected."""
import argparse
import difflib
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
    source=(upstream/'ciglet.c').read_text()
    old=' return ((sort_struct*)a) -> val > ((sort_struct*)b) -> val ? 1 : -1;'
    new=' const sort_struct *x=a,*y=b;\n if(x->val<y->val)return -1;\n if(x->val>y->val)return 1;\n return (x->idx>y->idx)-(x->idx<y->idx);'
    if source.count(old)!=1:raise ValueError('unexpected upstream sort comparator')
    corrected=source.replace(old,new)
    patch=''.join(difflib.unified_diff(source.splitlines(True),corrected.splitlines(True),fromfile='a/ciglet.c',tofile='b/ciglet.c'))
    patch_path=tests/'reference/statistics-sort-correction.patch'
    if args.check:
        if patch_path.read_text()!=patch:raise ValueError('sort correction patch changed')
    else:patch_path.write_text(patch)
    with tempfile.TemporaryDirectory(prefix='statistics-reference-') as temporary:
        root=Path(temporary);(root/'external').mkdir()
        (root/'ciglet.c').write_text(corrected);(root/'ciglet.h').write_text((upstream/'ciglet.h').read_text())
        (root/'external/fastapprox-all.h').write_text((upstream/'external/fastapprox-all.h').read_text())
        (root/'oracle.c').write_text((tests/'reference/statistics_oracle.c').read_text())
        for precision in ['float','double']:
            executable=root/(precision+('.exe' if os.name=='nt' else ''));output=root/(precision+'.bin')
            compiler=[args.cc,'cc','-target','x86_64-windows-gnu'] if args.zig else [args.cc]
            dependencies=[str(upstream/'external/fftsg_h.c'),str(upstream/'external/fast_median.c')] if os.name=='nt' else []
            linker=[] if os.name=='nt' else ['-Wl,--gc-sections']
            subprocess.run(compiler+['-O2','-DFP_TYPE='+precision,'-ffunction-sections','-fdata-sections','-I'+str(root),
                str(root/'oracle.c'),str(root/'ciglet.c'),'-lm','-o',str(executable)]+dependencies+linker,check=True,timeout=120)
            subprocess.run([str(executable),str(output)],check=True,timeout=30)
            target=tests/'fixtures'/('statistics-'+precision+'.bin');content=output.read_bytes()
            if args.check:
                if target.read_bytes()!=content:raise ValueError(f'C statistics changed: {precision}')
            else:target.write_bytes(content)
            print(precision,'C statistics verified')

if __name__=='__main__':main()
