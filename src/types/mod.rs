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

mod primitives;
pub mod methodhandle;
mod versions;
mod array;
mod errors;
mod access_flags;

pub use primitives::{Jbyte, Jshort, Jint, Jlong, Jchar, Jfloat, Jdouble, Jboolean};
pub use versions::{ClassFileVersion, CURRENT_VIRTUAL_MACHINE_VERSION};
pub use array::{Array, OutOfBoundsError};
pub use errors::*;
pub use access_flags::AccessFlags;
