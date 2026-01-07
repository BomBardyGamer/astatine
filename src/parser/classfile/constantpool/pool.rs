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

use crate::parser::classfile::constantpool::Entry;

pub struct Pool {
    pub(super) constants: Vec<Entry>
}

pub type Index = u16;
pub const INDEX_INVALID: Index = 0;

impl Pool {
    pub fn get(&self, index: Index) -> Option<&Entry> {
        // CP is indexed from 1 but backing array is indexed from 0
        self.constants.get((index - 1) as usize)
    }

    pub(super) fn put(&mut self, index: Index, entry: Entry) {
        self.constants.insert((index - 1) as usize, entry);
    }

    pub fn size(&self) -> u16 {
        // CP is indexed from 1 so size is 1 more than array size
        (self.constants.len() + 1) as u16
    }

    pub fn is_valid_index(&self, index: Index) -> bool {
        index >= 1 && index < self.size()
    }
}
