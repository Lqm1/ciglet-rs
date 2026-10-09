use ciglet_rs::array::Array2;
use std::cell::Cell;
use std::rc::Rc;

fn integer(bytes: &[u8], cursor: &mut usize) -> usize {
    let value = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
    *cursor += 4;
    value as usize
}
fn check<T: Clone + Default + PartialEq + std::fmt::Debug>(
    rows: usize,
    columns: usize,
    fields: &[Vec<T>],
) {
    let zero = Array2::<T>::zeros(rows, columns).unwrap();
    assert_eq!(zero.as_slice(), fields[1]);
    let original = Array2::from_slice(rows, columns, &fields[0]).unwrap();
    let copy = original.try_clone().unwrap();
    assert_eq!(copy.flatten().unwrap(), fields[2]);
    assert_eq!(original.flatten().unwrap(), fields[3]);
    let transposed = original.transposed().unwrap();
    assert_eq!((transposed.rows(), transposed.columns()), (columns, rows));
    assert_eq!(transposed.as_slice(), fields[4]);
    let mut moved = original;
    moved.transpose().unwrap();
    assert_eq!(moved, transposed);
    moved.transpose().unwrap();
    assert_eq!(moved, copy);
}
#[test]
fn unchanged_c_bytes_for_all_element_widths() {
    let bytes = include_bytes!("../../../tests/fixtures/array.bin");
    assert_eq!(&bytes[..4], b"ARR1");
    let mut cursor = 4;
    let cases = integer(bytes, &mut cursor);
    assert_eq!(cases, 240);
    for _ in 0..cases {
        let rows = integer(bytes, &mut cursor);
        let columns = integer(bytes, &mut cursor);
        let width = integer(bytes, &mut cursor);
        let size = rows * columns * width;
        let fields: Vec<&[u8]> = (0..5)
            .map(|_| {
                let values = &bytes[cursor..cursor + size];
                cursor += size;
                values
            })
            .collect();
        macro_rules! typed {
            ($ty:ty) => {{
                let fields: Vec<Vec<$ty>> = fields
                    .iter()
                    .map(|values| {
                        values
                            .chunks_exact(width)
                            .map(|value| <$ty>::from_le_bytes(value.try_into().unwrap()))
                            .collect()
                    })
                    .collect();
                check(rows, columns, &fields);
            }};
        }
        match width {
            1 => typed!(u8),
            2 => typed!(u16),
            4 => typed!(u32),
            8 => typed!(u64),
            _ => panic!("unexpected width"),
        }
    }
    assert_eq!(cursor, bytes.len());
}
#[test]
fn row_order_copies_reshape_and_invalid_shapes() {
    let source = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut array = Array2::from_rows(&source).unwrap();
    assert_eq!(array.as_slice(), &[1, 2, 3, 4, 5, 6]);
    assert_eq!(array.row(1), Some(&[4, 5, 6][..]));
    assert_eq!(array.get(1, 2), Some(&6));
    assert_eq!(array.get(2, 0), None);
    assert_eq!(array.get_mut(0, usize::MAX), None);
    assert_eq!(array.row(2), None);
    assert_eq!(array.row_mut(usize::MAX), None);
    *array.get_mut(1, 2).unwrap() = 9;
    array.row_mut(0).unwrap()[0] = 7;
    assert_eq!(source[0][0], 1);
    let pointer = array.as_slice().as_ptr();
    array.reshape(3, 2).unwrap();
    assert_eq!(array.as_slice().as_ptr(), pointer);
    assert_eq!(array.row(2), Some(&[5, 9][..]));
    let before = array.clone();
    assert!(array.reshape(2, 2).is_err());
    assert!(array.reshape(usize::MAX, 2).is_err());
    assert_eq!(array, before);
    assert_eq!(array.into_row_major(), [7, 2, 3, 4, 5, 9]);
    assert!(Array2::<u8>::from_row_major(2, 3, vec![0; 5]).is_err());
    assert!(Array2::<u8>::zeros(usize::MAX, 2).is_err());
    assert!(Array2::<u64>::zeros(usize::MAX, 1).is_err());
    assert!(Array2::from_rows(&[vec![1], vec![2, 3]]).is_err());
    for (rows, columns) in [(0, 0), (0, 3), (5, 0), (0, usize::MAX), (usize::MAX, 0)] {
        let mut empty = Array2::<u8>::zeros(rows, columns).unwrap();
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);
        if rows > 0 {
            assert_eq!(empty.row(0), Some(&[][..]));
        }
        let copy = empty.transposed().unwrap();
        empty.transpose().unwrap();
        assert_eq!(empty, copy);
        assert_eq!((empty.rows(), empty.columns()), (columns, rows));
    }
}
struct Value {
    id: usize,
    drops: Rc<Cell<usize>>,
}
impl Drop for Value {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}
#[test]
fn transpose_owned_values_and_preserve_float_bits() {
    let drops = Rc::new(Cell::new(0));
    let values: Vec<_> = (0..6)
        .map(|id| Value {
            id,
            drops: drops.clone(),
        })
        .collect();
    let mut array = Array2::from_row_major(2, 3, values).unwrap();
    array.transpose().unwrap();
    assert_eq!(
        array
            .as_slice()
            .iter()
            .map(|value| value.id)
            .collect::<Vec<_>>(),
        [0, 3, 1, 4, 2, 5]
    );
    assert_eq!(drops.get(), 0);
    drop(array);
    assert_eq!(drops.get(), 6);
    let bits = [
        0_u64,
        0x8000000000000000,
        0x7ff8000000000042,
        0x7ff0000000000000,
        0xfff0000000000000,
        0x3ff0000000000000,
    ];
    let mut array = Array2::from_row_major(2, 3, bits.map(f64::from_bits).to_vec()).unwrap();
    array.transpose().unwrap();
    assert_eq!(
        array
            .as_slice()
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        [bits[0], bits[3], bits[1], bits[4], bits[2], bits[5]]
    );
    let bits32 = [
        0_u32, 0x80000000, 0x7fc00042, 0x7f800000, 0xff800000, 0x3f800000,
    ];
    let mut array32 = Array2::from_row_major(2, 3, bits32.map(f32::from_bits).to_vec()).unwrap();
    array32.transpose().unwrap();
    assert_eq!(
        array32
            .as_slice()
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        [
            bits32[0], bits32[3], bits32[1], bits32[4], bits32[2], bits32[5]
        ]
    );
    let mut zero_sized = Array2::filled(2, 3, ()).unwrap();
    zero_sized.transpose().unwrap();
    assert_eq!(
        (zero_sized.rows(), zero_sized.columns(), zero_sized.len()),
        (3, 2, 6)
    );
    let mut triples =
        Array2::from_row_major(2, 2, vec![[1_u8, 2, 3], [4, 5, 6], [7, 8, 9], [10, 11, 12]])
            .unwrap();
    triples.transpose().unwrap();
    assert_eq!(
        triples.as_slice(),
        &[[1, 2, 3], [7, 8, 9], [4, 5, 6], [10, 11, 12]]
    );
}
