use std::ops::Sub;
use std::slice::ChunksExact;

pub(crate) trait MatrixLike {
    type Item;

    // fn new(chunk_size: usize) -> Self;

    fn len(&self) -> usize;

    fn fill(&mut self, input: &[Self::Item]);

    fn iter_rows(&self) -> ChunksExact<Self::Item>;
    fn iter_cols(&self) -> ColumnIter<Self::Item>
    where
        <Self as MatrixLike>::Item: std::fmt::Debug;

    fn transpose(&mut self);

    fn transpose_into<M: MatrixLike>(&self, target: &mut M)
    where
        M: MatrixLike,
        Vec<<M as MatrixLike>::Item>: FromIterator<Self::Item>;

    fn chunk_size(&self) -> usize;

    fn set_chunk_size(&mut self, new_chunk_size: usize);
}

// ==================== Matrix with Vec backend ================================

#[derive(Debug, Default, Clone)]
pub(crate) struct MatrixV<T: Default> {
    chunk_size: usize,
    data: Vec<T>,
}

impl<T: Default> MatrixV<T> {
    pub(crate) fn new(chunk_size: usize) -> Self {
        Self {
            chunk_size,
            ..Self::default()
        }
    }
}

impl<T: std::fmt::Debug + Default + Clone> MatrixLike for MatrixV<T> {
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
        self.data = input.to_vec();
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

    fn transpose_into<M>(&self, target: &mut M)
    where
        M: MatrixLike,
        Vec<<M as MatrixLike>::Item>: FromIterator<Self::Item>,
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
pub(crate) struct MatrixA<T: Default + Copy, const S: usize> {
    chunk_size: usize,
    capacity: usize,
    data: [T; S],
}

impl<T: Default + Copy, const S: usize> MatrixA<T, S> {
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
        }
    }
}

impl<T: std::fmt::Debug + Default + Copy + Clone, const S: usize> MatrixLike
    for MatrixA<T, S>
{
    type Item = T;

    fn len(&self) -> usize {
        self.capacity
    }

    fn fill(&mut self, input: &[Self::Item]) {
        input.iter().enumerate().for_each(|(i, v)| {
            self.data[i] = v.clone();
        });
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

    fn transpose_into<M>(&self, target: &mut M)
    where
        M: MatrixLike,
        Vec<<M as MatrixLike>::Item>: FromIterator<Self::Item>,
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
        T: std::fmt::Debug;
}

impl<T> ColumnIterExt<T> for [T] {
    fn column_iter(&self, chunk_size: usize) -> ColumnIter<'_, T>
    where
        T: std::fmt::Debug,
    {
        assert_ne!(chunk_size, 0);
        ColumnIter::new(self, chunk_size)
    }
}

#[derive(Debug, Clone)]
pub struct ColumnIter<'a, T: 'a>
where
    T: std::fmt::Debug,
{
    slice: &'a [T],
    chunk_size: usize,
    col_size: usize,
    max_offset: usize,
    offset: usize,
}

impl<'a, T> ColumnIter<'a, T>
where
    T: std::fmt::Debug,
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
    T: std::fmt::Debug,
{
    type Item = Vec<&'a T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item>
    where
        T: std::fmt::Debug,
    {
        if self.slice.len() < self.chunk_size {
            None
        } else if self.slice.len() < self.col_size {
            None
        } else if self.offset >= self.max_offset {
            None
        } else {
            // NOTE: this is very unschön, but we cannot build slices of non-contiguous memory (addresses)
            let mut result = Vec::with_capacity(self.col_size);
            for item in
                self.slice.iter().skip(self.offset).step_by(self.chunk_size)
            {
                // eprintln!("max:{} cz:{}, off:{}, item:{:?}", self.max_offset, self.col_size, self.offset, &item);
                result.push(item);
            }

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
        mv.fill(&sample[..]);
        assert_eq!(mv.len(), sample.len());

        eprintln!("mv before transpose {:?}", &mv);
        mv.transpose();
        eprintln!("mv after transpose  {:?}", &mv);

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
}
