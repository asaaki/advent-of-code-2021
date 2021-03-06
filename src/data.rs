use std::fmt::{Debug as FmtDebug, Display as FmtDisplay};
use std::marker::PhantomData;
use std::ops::Sub;
use std::slice::{ChunksExact, Iter, IterMut};

pub(crate) trait MatrixLike<'a> {
    type Item;

    fn len(&self) -> usize;

    fn fill(&mut self, input: &[Self::Item]);

    fn fill_iter<I>(&mut self, input_iter: I)
    where
        I: Iterator<Item = Self::Item>;

    fn fill_square(&mut self, value: Self::Item);

    fn insert(&mut self, x: usize, y: usize, v: Self::Item);

    fn get(&'a self, x: usize, y: usize) -> Option<&'a Self::Item>;

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Item>;

    fn view(&'a self) -> &[Self::Item];

    fn view_mut(&'a mut self) -> &mut [Self::Item];

    fn iter(&self) -> Iter<Self::Item>;

    fn iter_mut(&mut self) -> IterMut<Self::Item>;

    fn iter_rows(&self) -> ChunksExact<Self::Item>;
    fn iter_cols(&self) -> ColumnIter<Self::Item>
    where
        <Self as MatrixLike<'a>>::Item: FmtDebug;

    fn transpose(&mut self);

    fn transpose_inplace(&mut self);

    fn transpose_into<M: MatrixLike<'a>>(&self, target: &mut M)
    where
        M: MatrixLike<'a>,
        Vec<<M as MatrixLike<'a>>::Item>: FromIterator<Self::Item>;

    fn chunk_size(&self) -> usize;

    fn set_chunk_size(&mut self, new_chunk_size: usize);
}

// ==================== Matrix with Vec backend ================================

#[derive(Debug, Default, Clone)]
pub(crate) struct MatrixV<'a, T: 'a + Default> {
    chunk_size: usize,
    data: Vec<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: FmtDebug + FmtDisplay + Default + Copy> MatrixV<'a, T> {
    pub(crate) fn new(chunk_size: usize) -> Self {
        Self {
            chunk_size,
            ..Self::default()
        }
    }

    #[allow(dead_code)]
    pub(crate) fn debug_print(&self) {
        for row in self.iter_rows() {
            eprintln!(
                "{:?}",
                row.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("")
            );
        }
    }
}

impl<'a, T: FmtDebug + Default + Copy + Clone> MatrixLike<'a>
    for MatrixV<'a, T>
{
    type Item = T;

    fn len(&self) -> usize {
        self.data.len()
    }

    fn fill(&mut self, input: &[Self::Item]) {
        assert!(
            input.len() % self.chunk_size == 0,
            "input data (len: {}) is not a multiple of desired chunk size {}",
            input.len(),
            self.chunk_size
        );
        // for now only 2 cases:
        // - the matrix was freshly created,
        // - or we want to replace its existing content
        // ! extending matrices with .fill() is not supported
        if self.data.is_empty() {
            self.data.extend_from_slice(input);
        } else {
            self.data.copy_from_slice(input);
        }
    }

    fn fill_iter<I>(&mut self, input_iter: I)
    where
        I: Iterator<Item = T>,
    {
        input_iter.for_each(|v| self.data.push(v))
    }

    fn fill_square(&mut self, value: Self::Item) {
        let size = self.chunk_size * self.chunk_size;
        self.data = vec![value; size];
    }

    fn insert(&mut self, x: usize, y: usize, v: Self::Item) {
        let pos = x + y * self.chunk_size;
        self.data[pos] = v;
    }

    fn get(&'a self, x: usize, y: usize) -> Option<&'a Self::Item> {
        let pos = x + y * self.chunk_size;
        self.data.get(pos)
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Item> {
        let pos = x + y * self.chunk_size;
        self.data.get_mut(pos)
    }

    fn view(&'a self) -> &[Self::Item] {
        &self.data[..]
    }

    fn view_mut(&'a mut self) -> &mut [Self::Item] {
        &mut self.data[..]
    }

    fn iter(&self) -> Iter<Self::Item> {
        self.data.iter()
    }

    fn iter_mut(&mut self) -> IterMut<Self::Item> {
        self.data.iter_mut()
    }

    fn iter_rows(&self) -> ChunksExact<Self::Item> {
        self.data.chunks_exact(self.chunk_size)
    }

    fn iter_cols(&self) -> ColumnIter<Self::Item> {
        self.data.column_iter(self.chunk_size)
    }

    fn transpose(&mut self) {
        let transposed: Vec<Self::Item> =
            self.iter_cols().flatten().map(ToOwned::to_owned).collect();
        self.chunk_size = transposed.len() / self.chunk_size;
        self.fill(&transposed[..]);
    }

    fn transpose_inplace(&mut self) {
        let col_size = self.data.len() / self.chunk_size;
        if col_size == self.chunk_size {
            for n in 0..(col_size-1) {
                for m in (n+1)..col_size {
                    self.data.swap(n*col_size + m, m * col_size + n)
                }
            }
        } else {
            let mut scratch: Vec<T> = vec![T::default(); self.chunk_size.max(col_size)];
            transpose::transpose_inplace(&mut self.data, &mut scratch, self.chunk_size, col_size);
            self.chunk_size = col_size;
        }
    }

    fn transpose_into<M>(&self, target: &mut M)
    where
        M: MatrixLike<'a>,
        Vec<<M as MatrixLike<'a>>::Item>: FromIterator<Self::Item>,
    {
        let transposed: Vec<M::Item> =
            self.iter_cols().flatten().map(ToOwned::to_owned).collect();
        target.set_chunk_size(transposed.len() / self.chunk_size);
        target.fill(&transposed[..]);
    }

    fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    fn set_chunk_size(&mut self, new_chunk_size: usize) {
        self.chunk_size = new_chunk_size;
    }
}

// ==================== Matrix with Array backend ==============================

#[derive(Debug, Clone)]
pub(crate) struct MatrixA<'a, T: Default + Copy, const S: usize> {
    chunk_size: usize,
    capacity: usize,
    data: [T; S],
    _marker: PhantomData<&'a T>,
}

impl<'a, T: FmtDebug + FmtDisplay + Default + Copy, const S: usize>
    MatrixA<'a, T, S>
{
    #[allow(dead_code)]
    pub(crate) fn new(chunk_size: usize) -> Self {
        let data = [T::default(); S];
        let capacity = data.len();
        assert!(
            capacity % chunk_size == 0,
            "chunk size {} is incompatible with desired fixed capacity {}",
            chunk_size,
            capacity
        );
        Self {
            chunk_size,
            capacity,
            data,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn debug_print(&self) {
        for row in self.iter_rows() {
            // eprintln!("{:?}", row);
            eprintln!(
                "{}",
                row.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("")
            );
        }
    }
}

impl<'a, T: FmtDebug + Default + Copy + Clone, const S: usize> MatrixLike<'a>
    for MatrixA<'a, T, S>
{
    type Item = T;

    fn len(&self) -> usize {
        self.capacity
    }

    fn fill(&mut self, input: &[Self::Item]) {
        self.data.copy_from_slice(input)
    }

    fn fill_iter<I>(&mut self, input_iter: I)
    where
        I: Iterator<Item = T>,
    {
        input_iter.enumerate().for_each(|(i, v)| {
            self.data[i] = v;
        });
    }

    // Note: Since S doesn't need to be a square number of chunk_size,
    //       the name might be a bit misleading.
    fn fill_square(&mut self, value: Self::Item) {
        self.data = [value; S];
    }

    fn insert(&mut self, x: usize, y: usize, v: Self::Item) {
        let pos = x + y * self.chunk_size;
        self.data[pos] = v;
    }

    fn get(&'a self, x: usize, y: usize) -> Option<&'a Self::Item> {
        let pos = x + y * self.chunk_size;
        self.data.get(pos)
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Item> {
        let pos = x + y * self.chunk_size;
        self.data.get_mut(pos)
    }

    fn view(&'a self) -> &[Self::Item] {
        &self.data[..]
    }

    fn view_mut(&'a mut self) -> &mut [Self::Item] {
        &mut self.data[..]
    }

    fn iter(&self) -> Iter<Self::Item> {
        self.data.iter()
    }

    fn iter_mut(&mut self) -> IterMut<Self::Item> {
        self.data.iter_mut()
    }

    fn iter_rows(&self) -> ChunksExact<Self::Item> {
        self.data.chunks_exact(self.chunk_size)
    }

    fn iter_cols(&self) -> ColumnIter<Self::Item> {
        self.data.column_iter(self.chunk_size)
    }

    fn transpose(&mut self) {
        let transposed: Vec<Self::Item> =
            self.iter_cols().flatten().map(ToOwned::to_owned).collect();
        self.chunk_size = transposed.len() / self.chunk_size;
        self.fill(&transposed[..]);
    }

    fn transpose_inplace(&mut self) {
        let col_size = self.data.len() / self.chunk_size;
        if col_size == self.chunk_size {
            for n in 0..(col_size-1) {
                for m in (n+1)..col_size {
                    self.data.swap(n*col_size + m, m * col_size + n)
                }
            }
        } else {
            let mut scratch: Vec<T> = vec![T::default(); self.chunk_size.max(col_size)];
            transpose::transpose_inplace(&mut self.data, &mut scratch, self.chunk_size, col_size);
            self.chunk_size = col_size;
        }
    }

    fn transpose_into<M>(&self, target: &mut M)
    where
        M: MatrixLike<'a>,
        Vec<<M as MatrixLike<'a>>::Item>: FromIterator<Self::Item>,
    {
        let transposed: Vec<M::Item> =
            self.iter_cols().flatten().map(ToOwned::to_owned).collect();
        target.set_chunk_size(transposed.len() / self.chunk_size);
        target.fill(&transposed[..]);
    }

    fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    fn set_chunk_size(&mut self, new_chunk_size: usize) {
        self.chunk_size = new_chunk_size;
    }
}

// ==================== Column iterator ========================================

trait ColumnIterExt<T> {
    fn column_iter(&self, chunk_size: usize) -> ColumnIter<'_, T>
    where
        T: FmtDebug;
}

impl<T> ColumnIterExt<T> for [T] {
    fn column_iter(&self, chunk_size: usize) -> ColumnIter<'_, T>
    where
        T: FmtDebug,
    {
        assert_ne!(chunk_size, 0);
        ColumnIter::new(self, chunk_size)
    }
}

#[derive(Debug, Clone)]
pub struct ColumnIter<'a, T: 'a>
where
    T: FmtDebug,
{
    slice: &'a [T],
    chunk_size: usize,
    col_size: usize,
    max_offset: usize,
    offset: usize,
}

impl<'a, T> ColumnIter<'a, T>
where
    T: FmtDebug,
{
    #[inline]
    pub(super) fn new(slice: &'a [T], chunk_size: usize) -> Self {
        let col_size = slice.len() / chunk_size;
        let max_offset = slice.len() / col_size;
        Self {
            slice,
            chunk_size,
            col_size,
            max_offset,
            offset: 0,
        }
    }
}

impl<'a, T: Clone> Iterator for ColumnIter<'a, T>
where
    T: FmtDebug,
{
    type Item = Vec<&'a T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item>
    where
        T: FmtDebug,
    {
        if self.slice.len() < self.chunk_size
            || self.slice.len() < self.col_size
            || self.offset >= self.max_offset
        {
            None
        } else {
            // NOTE: this is very unsch??n, but we cannot build slices of non-contiguous memory (addresses)
            let mut result = Vec::with_capacity(self.col_size);
            self.slice
                .iter()
                .skip(self.offset)
                .step_by(self.chunk_size)
                .for_each(|item| result.push(item));

            self.offset += 1;
            Some(result)
        }
    }
}

// =============================================================================

pub(crate) trait Invertable<T> {
    fn invert(&self, max: Option<&T>) -> Vec<T>;
}

impl<T: Copy + Ord + Sub> Invertable<T> for Vec<T>
where
    Vec<T>: FromIterator<<T as Sub>::Output>,
{
    fn invert(&self, max: Option<&T>) -> Vec<T> {
        let max = max.or_else(|| self.iter().max()).unwrap().to_owned();
        self.iter().map(|&i| max - i).collect()
    }
}

// =============================================================================

mod test {
    #[allow(unused_imports)]
    use crate::data::*;

    #[test]
    fn simple() {
        /*
            chunkS: 3; colSize: 4
             1  2  3
             4  5  6
             7  8  9
            10 11 12

            chunkSize: 4, colSize: 3
             1  4  7 10
             2  5  8 11
             3  6  9 12
        */
        let sample = vec![1usize, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let chunk_size = 3usize;

        let mut mv: MatrixV<usize> = MatrixV::new(chunk_size);
        let mut mv2: MatrixV<usize> = MatrixV::new(chunk_size + 1);
        mv.fill(&sample[..]);
        mv2.fill(&sample[..]);
        assert_eq!(mv.len(), sample.len());
        assert_eq!(mv2.len(), sample.len());

        eprintln!("mv  before transpose  {:?}", &mv);
        mv.transpose();
        eprintln!("mv  after transpose   {:?}", &mv);
        mv.transpose_inplace();
        eprintln!("mv  after transpose_F {:?}", &mv);

        eprintln!("mv2 before transpose {:?}", &mv2);
        mv2.transpose();
        eprintln!("mv2 after transpose  {:?}", &mv2);

        const MA_SIZE: usize = 9; // cannot calculate during runtime
        let sample = vec![11usize, 12, 13, 14, 15, 16, 17, 18, 19];
        let mut ma: MatrixA<usize, { MA_SIZE }> = MatrixA::new(chunk_size);
        ma.fill(&sample[..]);
        assert_eq!(ma.len(), sample.len());

        eprintln!("ma before transpose {:?}", &ma);
        ma.transpose();
        eprintln!("ma after transpose  {:?}", &ma);

        // let mut ma2 = ma.clone();
        // mv.transpose_into(&mut ma2);
        // eprintln!("mv->ma2 {:?}", &ma2);
        // ma.transpose_into(&mut mv);
        // eprintln!("ma->mv  {:?}", &mv);
    }

    #[test]
    fn transpose_square() {
        let sample = vec![1usize, 2, 3, 4, 5, 6, 7, 8, 9];
        let chunk_size = 3usize;

        let mut mv: MatrixV<usize> = MatrixV::new(chunk_size);
        mv.fill(&sample[..]);

        eprintln!("[fast] before transpose {:?}", &mv);
        mv.transpose_inplace();
        eprintln!("[fast] after transpose1 {:?}", &mv);
        mv.transpose_inplace();
        eprintln!("[fast] after transpose2 {:?}", &mv);

        mv.fill(&sample[..]);

        eprintln!("[slow] before transpose {:?}", &mv);
        mv.transpose();
        eprintln!("[slow] after transpose1 {:?}", &mv);
        mv.transpose();
        eprintln!("[slow] after transpose2 {:?}", &mv);
    }

    #[test]
    fn fill_and_get() {
        let chunk_size = 3usize;
        let mut mv: MatrixV<usize> = MatrixV::new(chunk_size);
        mv.fill_square(0);
        eprintln!("mv 0-filled {:?}", &mv);
        mv.insert(1, 1, 9);
        eprintln!("mv inserted {:?}", &mv);
    }
}
