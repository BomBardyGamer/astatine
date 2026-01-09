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

mod constantpool;
mod attribute;
mod parser;
mod field;
mod method;

pub struct ClassFile {
    minor_version: u16,
    major_version: u16,
    pub constant_pool: constantpool::Pool,
    access_flags: u16,
    this_class: constantpool::Index,
    super_class: constantpool::Index,
    interfaces: Vec<constantpool::Index>,
    fields: Vec<field::Field>,
    methods: Vec<method::Method>,
    attributes: Vec<attribute::ClassFileAttribute>
}

#[repr(u16)]
#[derive(Primitive, Debug, PartialEq, Copy, Clone)]
pub enum AccessFlag {
    Public = 0x0001,
    Final = 0x0010,
    Super = 0x0020,
    Interface = 0x0200,
    Abstract = 0x0400,
    Synthetic = 0x1000,
    Annotation = 0x2000,
    Enum = 0x4000,
    Module = 0x8000,
}
