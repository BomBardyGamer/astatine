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

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

use crate::loader::{BinaryReader, Parse};
use crate::loader::classfile::ClassFile;

mod loader;
pub mod types;

fn main() {
    let mut r = BinaryReader::new(Vec::<u8>::new());
    let c = ClassFile::parse(&mut r).unwrap();
    c.constant_pool.get_integer(1);
    // TODO
}
