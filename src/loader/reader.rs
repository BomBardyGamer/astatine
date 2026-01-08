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

use std::cmp::min;
use std::ptr;
use crate::loader::parse::ParserError;

pub struct BinaryReader {
    buf: Vec<u8>,
    off: usize,
}

impl BinaryReader {
    // Doesn't error with EOF as this function just reads as many bytes as it can from the buffer
    pub fn read(&mut self, out: &mut [u8]) -> usize {
        let read_len = min(self.buf.len() - self.off, out.len());

        // SAFETY: read_len ensures that we will only read the minimum of how many bytes are left
        // in the buffer and the size of the output. Both vectors and slices are properly aligned.
        unsafe {
            ptr::copy(self.buf.as_ptr().offset(self.off as isize), out.as_mut_ptr(), read_len);
        }

        self.off += read_len;
        read_len
    }

    pub fn read_u8(&mut self) -> Result<u8, EndOfBufferError> {
        self.check_eof(1)?;

        // SAFETY: Guaranteed by has_bytes check above
        let r = unsafe { self.unsafe_read_u8() };
        Ok(r)
    }

    pub unsafe fn unsafe_read_u8(&mut self) -> u8 {
        // SAFETY: Caller must guarantee that buffer has remaining bytes with has_bytes call
        // else behaviour is undefined
        let r = unsafe { *self.buf.get_unchecked(self.off) };
        self.off += 1;
        r
    }

    pub fn read_u16(&mut self) -> Result<u16, EndOfBufferError> {
        self.check_eof(2)?;

        // SAFETY: Guaranteed by has_bytes check above
        let r = unsafe { self.unsafe_read_u16() };
        Ok(r)
    }

    pub unsafe fn unsafe_read_u16(&mut self) -> u16 {
        let (a, b): (u16, u16);

        // SAFETY: Caller must guarantee that buffer has remaining bytes
        // else behaviour is undefined
        unsafe {
            a = *self.buf.get_unchecked(self.off) as u16;
            b = *self.buf.get_unchecked(self.off + 1) as u16;
        }

        self.off += 2;
        (a << 8) | b
    }

    pub fn read_u32(&mut self) -> Result<u32, EndOfBufferError> {
        self.check_eof(4)?;

        // SAFETY: Guaranteed by has_bytes check above
        let r = unsafe { self.unsafe_read_u32() };
        Ok(r)
    }

    pub unsafe fn unsafe_read_u32(&mut self) -> u32 {
        let (a, b, c, d): (u32, u32, u32, u32);

        // SAFETY: Caller must guarantee that buffer has remaining bytes
        // else behaviour is undefined
        unsafe {
            let vals = self.buf.get_unchecked(self.off..self.off+3);
            (a, b, c, d) = (vals[0] as u32, vals[1] as u32, vals[2] as u32, vals[3] as u32);
        };

        self.off += 4;
        (a << 24) | (b << 16) | (c << 8) | d
    }

    pub fn read_u16_slice(&mut self, out: &mut [u16]) -> Result<(), EndOfBufferError> {
        self.check_eof(out.len() * 2)?;

        // SAFETY: Guaranteed by has_bytes check above
        unsafe { self.unsafe_read_u16_slice(out) }
        Ok(())
    }

    pub unsafe fn unsafe_read_u16_slice(&mut self, out: &mut [u16]) {
        let byte_len = out.len() * 2;

        // TODO: It's probably possible to do it faster than this, but
        //  as it stands at the moment, it's not worth it.
        let mut j = 0;
        for i in (0..byte_len).step_by(2) {
            let (a, b): (u16, u16);

            // SAFETY: Caller must guarantee that buffer has remaining bytes
            // else behaviour is undefined
            unsafe {
                let v = self.buf.get_unchecked(self.off+i..self.off+i+2);
                (a, b) = (v[0] as u16, v[1] as u16);
            }

            out[j] = (a << 8) | b;
            j += 1;
        }

        self.off += byte_len
    }

    pub const fn has_bytes(&self, num: usize) -> bool {
        self.buf.len() < self.off + num
    }

    fn check_eof(&self, bytes: usize) -> Result<(), EndOfBufferError> {
        match self.has_bytes(bytes) {
            true => Ok(()),
            false => Err(END_OF_BUFFER),
        }
    }

    pub fn check_bytes(&self, num: usize, msg: impl Into<String>) -> Result<(), ParserError> {
        match self.has_bytes(num) {
            true => Ok(()),
            false => ParserError::not_enough_bytes(msg.into())
        }
    }

    pub const fn empty(&self) -> bool {
        !self.has_bytes(1)
    }
}

/// Allows arbitrary reader function for len to allow different u-sized lengths
/// This is useful in code array as the size of the code array is a u32 but the array is Vec<u8>
#[macro_export]
macro_rules! buf_read_u8_vec_lensize {
    ($var_name: ident, $buf: expr, $read_len: ident, $error: expr) => {
        $buf.check_bytes(2, $error)?;

        let mut $var_name: Vec<u8>;
        {
            // SAFETY: Guaranteed by check_bytes at top
            let len = unsafe { $buf.$read_len() };
            $buf.check_bytes(len as usize, $error)?;

            $var_name = Vec::with_capacity(len as usize);
            $buf.read(&mut $var_name);
        }
    };
}

#[macro_export]
macro_rules! buf_read_u16_vec {
    ($var_name: ident, $buf: expr, $error: expr) => {
        $buf.check_bytes(2, $error)?;

        let mut $var_name;
        {
            // SAFETY: Guaranteed by check_bytes at top
            let len = unsafe { $buf.unsafe_read_u16() } as usize;
            $buf.check_bytes(len * 2, $error)?;

            $var_name = Vec::with_capacity(len as usize);
            // SAFETY: Guaranteed by check_bytes on len
            unsafe { $buf.unsafe_read_u16_slice(&mut $var_name) };
        }
    };
}

#[macro_export]
macro_rules! buf_read_named_type_vec {
    ($typ: ident, $var_name: ident, $buf: expr, $error: expr, $error_idx: expr) => {
        $buf.check_bytes(2, $error)?;

        let mut $var_name: Vec<$typ>;
        {
            // SAFETY: Guaranteed by check_bytes at top
            let len = unsafe { $buf.unsafe_read_u16() };
            $var_name = Vec::with_capacity(len as usize);

            for i in 0..len {
                $var_name.push($typ::parse($buf).map_err(ParserError::wrap(format!($error_idx, i)))?);
            }
        }
    };
}

pub struct EndOfBufferError;
const END_OF_BUFFER: EndOfBufferError = EndOfBufferError{};
