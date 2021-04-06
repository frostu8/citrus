use super::*;

use std::marker::PhantomData;

/// An immutable iterator over an [`EnunMap`].
pub struct Iter<'a, E: EnumKey<T>, T> {
    data: &'a [T],
    index: usize,
    _marker: PhantomData<E>,
}

impl<'a, E: EnumKey<T>, T> Iter<'a, E, T> {
    /// Creates a new immutable iterator over an [`EnumMap`].
    ///
    /// Use [`EnumMap::iter`] instead.
    pub fn new(map: &'a EnumMap<E, T>) -> Iter<'a, E, T> {
        Iter {
            data: map.data.as_ref(),
            index: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a, E: EnumKey<T>, T> Iterator for Iter<'a, E, T> {
    type Item = (E, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let data = std::mem::take(&mut self.data);

        data.split_first().map(|(next, rest)| {
            // update iter data
            self.data = rest;

            // return next data
            let next_index = self.index;
            self.index += 1;

            (E::from_usize(next_index), next)
        })
    }
}

/// A mutable iterator over an [`EnunMap`].
pub struct IterMut<'a, E: EnumKey<T>, T> {
    data: &'a mut [T],
    index: usize,
    _marker: PhantomData<E>,
}

impl<'a, E: EnumKey<T>, T> IterMut<'a, E, T> {
    /// Creates a new immutable iterator over an [`EnumMap`].
    ///
    /// Use [`EnumMap::iter`] instead.
    pub fn new(map: &'a mut EnumMap<E, T>) -> IterMut<'a, E, T> {
        IterMut {
            data: map.data.as_mut(),
            index: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a, E: EnumKey<T>, T> Iterator for IterMut<'a, E, T> {
    type Item = (E, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let data = std::mem::take(&mut self.data);

        data.split_first_mut().map(|(next, rest)| {
            // update iter data
            self.data = rest;

            // return next data
            let next_index = self.index;
            self.index += 1;

            (E::from_usize(next_index), next)
        })
    }
}
