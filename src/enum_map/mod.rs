mod iter;

pub use iter::*;

use std::ops::{Index, IndexMut};
use std::mem::MaybeUninit;

/// A map that uses enums as its keys.
/// 
/// Blazing fast, stack allocated. What is there not to love about this?
pub struct EnumMap<E: EnumKey<T>, T> {
    data: E::Storage,
}

impl<E: EnumKey<T>, T> EnumMap<E, T> {
    /// Creates a new [`EnumMap`] using an initializer function.
    pub fn new<F>(mut init: F) -> EnumMap<E, T> 
    where F: FnMut(E) -> T {
        EnumMap {
            data: E::Storage::init(|index| {
                init(E::from_usize(index))
            }),
        }
    }

    /// Creates a new iterator over the elements of [`EnumMap`].
    pub fn iter(&self) -> Iter<E, T> {
        Iter::new(self)
    }

    /// Creates a new mutable iterator over the elements of [`EnumMap`].
    pub fn iter_mut(&mut self) -> IterMut<E, T> {
        IterMut::new(self)
    }
}

impl<E: EnumKey<Option<T>>, T> EnumMap<E, Option<T>> {
    /// Creates a new [`EnumMap`] with all `Option` values initialized to zero.
    pub fn new_empty() -> EnumMap<E, Option<T>> {
        EnumMap::new(|_| None)
    }
}

impl<E: EnumKey<T>, T> Index<E> for EnumMap<E, T> {
    type Output = T;

    fn index(&self, variant: E) -> &Self::Output {
        &self.data.as_ref()[E::into_usize(variant)]
    }
}

impl<E: EnumKey<T>, T> IndexMut<E> for EnumMap<E, T> {
    fn index_mut(&mut self, variant: E) -> &mut Self::Output {
        &mut self.data.as_mut()[E::into_usize(variant)]
    }
}

/// The `EnumKey` trait, which allows an [`EnumMap`] to interface with an
/// enum (or any type at all).
///
/// Implementors must fulfill the following contracts:
///
///   * [`EnumKey::into_usize`] must return a single unique `usize` for every
///     variant the Enum possesses. This `usize` must also be in the range
///     `0..Storage::LENGTH`.
///   * [`EnumKey::from_usize`] must return a single unique variant for every
///     `usize` returned in [`EnumKey::into_usize`].
///
/// Failing to fulfill these contracts won't cause anything that safe Rust
/// can't, but there will be panics, suffering, and in extreme cases, death.
pub trait EnumKey<T> {
    /// The storage type that the [`EnumKey`] uses. Only static arrays, like
    /// `[T; N]` are supported.
    type Storage: Storage<T>;

    /// Converts an enum variant to an integer index.
    fn into_usize(variant: Self) -> usize;

    /// Converts an integer index to an enum variant.
    ///
    /// # Gaurantees
    /// Each and every call to this is gauranteed to only either be within the
    /// bounds of the [`EnumKey::Storage`], or a result from 
    /// [`EnumKey::into_usize`].
    fn from_usize(int: usize) -> Self;
}

#[doc(hidden)]
pub trait Storage<T> {
    const LENGTH: usize;

    fn as_ref(&self) -> &[T];
    fn as_mut(&mut self) -> &mut [T];
    
    fn init<F>(initializer: F) -> Self
    where F: FnMut(usize) -> T; 
}

impl<T, const N: usize> Storage<T> for [T; N] {
    const LENGTH: usize = N;

    fn as_ref(&self) -> &[T] {
        self
    }

    fn as_mut(&mut self) -> &mut [T] {
        self
    }

    // implemented using
    // https://github.com/Manishearth/array-init/blob/master/src/lib.rs
    // as a reference.
    fn init<F>(mut initializer: F) -> Self
    where F: FnMut(usize) -> T {
        struct Guard<T> {
            ptr_start: *mut T,
            initialized: usize,
        }

        impl<T> Drop for Guard<T> {
            fn drop(&mut self) {
                let initialized_part = std::ptr::slice_from_raw_parts_mut(
                    self.ptr_start, self.initialized,
                );

                // SAFETY: this is sound, because initialized_part will only
                // have the initialized types.
                unsafe { std::ptr::drop_in_place(initialized_part); }
            }
        }

        // SAFETY: sound because:
        //
        //     using a guard, array[..initialized] will be dropped in the case
        //     of a panic, dropping all of the previously initialized elements.
        //
        //     we are in the array at all times (N > i > 0)
        unsafe {
            let mut array = MaybeUninit::<[T; N]>::uninit();
            let mut ptr_i = array.as_mut_ptr() as *mut T;

            // initialize a panic guard; we need to make sure that drop is only
            // called for initialized elements
            let mut guard = Guard {
                ptr_start: ptr_i,
                initialized: 0,
            };

            for i in 0..N {
                // `i` elements have already been initialized.
                guard.initialized = i;

                // if a panic happens, the Guard::drop function will be called,
                // dropping the first `i` elements, the elements that we have
                // already initialized.
                let value = initializer(i);

                // previous uninit value is overwritten without being dropped.
                ptr_i.write(value);
                ptr_i = ptr_i.add(1);
            }

            // forget the panic guard and return to symbolic ownership
            std::mem::forget(guard);
            
            array.assume_init()
        }
    }
}

