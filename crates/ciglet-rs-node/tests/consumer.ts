import { root_find_f64, scalar_fold_f64, version } from '..';
const root: number = root_find_f64(value => value * value - 2, 1, 2);
const total: number = scalar_fold_f64(new Float64Array([1, 2]), 0,
  (accumulator, value, context) => {
    if (typeof context !== 'number') throw new TypeError('numeric context required');
    return accumulator + value * context;
  }, 2);
const packageVersion: string = version();
void [root, total, packageVersion];
