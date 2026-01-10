// Copyright (C) 2026 Callum Jay Seabrook Hefford (BomBardyGamer)
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, see <https://www.gnu.org/licenses/>.

use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;
use std::slice;
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
        if len == 0 {
            // When len is 0, we don't need to (and shouldn't) allocate
            return Ok(Self::empty())
        }

        let ptr = Self::alloc_mem(len)?;
        Ok(Self { ptr, len })
    }

    pub const fn empty() -> Array<T> {
        let ptr = NonNull::dangling();
        Self { ptr, len: 0 }
    }

    fn alloc_mem(size: usize) -> Result<NonNull<T>, errors::OutOfMemoryError> {
        // We do not support zero sized types in this Vec implementation
        assert_ne!(size_of::<T>(), 0);

        let layout = Layout::array::<T>(size).unwrap();

        // SAFETY: The size of type T is non-zero and it is well-aligned.
        let alloc = unsafe { alloc(layout) };
        NonNull::new(alloc as *mut T).ok_or(errors::OutOfMemoryError)
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        // This also safeguards for len == 0
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

    // This function is indended for use by Drop
    // Caller must guarantee that index is in range
    unsafe fn get_and_drop(&self, index: usize) {
        assert!(index < self.len, "index out of bounds");

        let ptr = unsafe { self.ptr().add(index) };
        if ptr.is_null() {
            return
        }
        unsafe { ptr.drop_in_place() }
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
        // This also safeguards for len == 0
        if index >= self.len {
            return Err(OutOfBoundsError);
        }

        // SAFETY: Bounds have been checked above
        unsafe { let _ = self.set_unchecked(index, v); }
        Ok(())
    }

    pub fn set_and_return(&mut self, index: usize, v: T) -> Result<&T, OutOfBoundsError> {
        if index >= self.len {
            return Err(OutOfBoundsError);
        }

        // SAFETY: Bounds have been checked above
        let ptr = unsafe { self.set_unchecked(index, v) };
        Ok(unsafe { &*ptr })
    }

    // SAFETY: Index in bounds must be checked by caller else behaviour is undefined
    unsafe fn set_unchecked(&mut self, index: usize, v: T) -> *mut T {
        // SAFETY: Memory is contiguous so this is fine
        let ptr = unsafe { self.ptr().add(index) };
        // SAFETY: We know we own this memory and it is allocated
        unsafe { ptr.write(v) }
        ptr
    }

    // SAFETY: Caller must guarantee that Array has been fully initialized
    // as slices are assumed to be initialized, or must guarantee not to perform
    // get operations on the slice, else behaviour is undefined.
    pub unsafe fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr(), self.len) }
    }

    // SAFETY: Caller must guarantee that Array has been fully initialized
    // as slices are assumed to be initialized, or must guarantee not to perform
    // get operations on the slice, else behaviour is undefined.
    pub unsafe fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.ptr(), self.len) }
    }

    #[inline]
    fn ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }
}

impl<T: Clone> Array<T> {
    // SAFETY: Caller must guarantee that Array has been fully initialized
    // See: as_slice and as_slice_mut
    pub unsafe fn to_vec(&self) -> Vec<T> {
        // SAFETY: On the caller
        let slice = unsafe { self.as_slice() };
        slice.to_vec()
    }
}

impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        if self.len == 0 {
            return;
        }

        // Drop all values owned by array
        for i in 0..self.len {
            unsafe { self.get_and_drop(i) }
        }

        // Safe to unwrap as we know this layout works, else we wouldn't have been able to
        // create the Array in the first place
        let layout = Layout::array::<T>(self.len).unwrap();
        unsafe {
            dealloc(self.ptr() as *mut u8, layout)
        }
    }
}

#[derive(Debug)]
pub struct OutOfBoundsError;
