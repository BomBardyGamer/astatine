use std::alloc::{alloc, Layout};
use std::ptr::NonNull;
use crate::types::errors;

/// A simple vector (array) with a fixed length, known at runtime.
///
/// This is essentially a slice but without the requirement for the
/// size to be known at compile time.
///
/// The implementation of this is inspired by Vec internals and the
/// Rustonomicon's Vec page: https://doc.rust-lang.org/nomicon/vec/vec.html
///
/// This does not behave like a Vec at all though. It is fixed size, all the
/// memory for it is allocated on creation, and all elements are uninitialized
/// by default. This behaves more like arrays in other languages.
pub struct Array<T> {
    ptr: NonNull<T>,
    len: usize
}

impl<T> Array<T> {
    pub fn new(len: usize) -> Result<Self, errors::OutOfMemoryError> {
        let ptr = Self::alloc_mem(len)?;
        Ok(Self { ptr, len })
    }

    fn alloc_mem(size: usize) -> Result<NonNull<T>, errors::OutOfMemoryError> {
        // We do not support zero sized types in this Vec implementation
        assert_ne!(size_of::<T>(), 0);

        let layout = Layout::array::<T>(size).unwrap();

        // SAFETY: The size of type T is non-zero and it is well-aligned.
        let alloc = unsafe { alloc(layout) };

        // Safe to unwrap here as we know the pointer is non-null
        NonNull::new(alloc as *mut T).ok_or(errors::OutOfMemoryError)
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        // SAFETY: We know the memory at ptr + index when index is in bounds
        //  is memory that exists and that we own, else we wouldn't've been
        //  able to allocate this array.
        let ptr = unsafe { self.ptr().add(index) };
        // SAFETY: If null, propagates up. If non-null, is guaranteed to be convertible to a ref
        let r = unsafe { ptr.as_ref() }?;
        Some(r)
    }

    // Same as get, but without bounds checking
    // SAFETY: If the value at index is null or out of bounds, behaviour is undefined
    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        // SAFETY: Must be guaranteed by caller
        let ptr = unsafe { self.ptr().add(index) };
        // SAFETY: Value is checked for null and ref will last as long as the array lasts
        unsafe { &*ptr }
    }

    pub fn set(&mut self, index: usize, v: T) -> Result<(), OutOfBoundsError> {
        if index >= self.len {
            return Err(OutOfBoundsError);
        }

        let ptr = unsafe { self.ptr().add(index) };
        unsafe { ptr.write(v) };
        Ok(())
    }

    fn ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }
}

#[derive(Debug)]
pub struct OutOfBoundsError;
