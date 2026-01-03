use crate::parser::classfile::constantpool::Entry;

pub struct Pool {
    constants: Vec<Entry>
}

pub type Index = u16;
pub const INDEX_INVALID: Index = 0;

impl Pool {
    pub fn get(&self, index: Index) -> Option<&Entry> {
        // CP is indexed from 1 but backing array is indexed from 0
        self.constants.get((index - 1) as usize)
    }

    fn put(&mut self, index: Index, entry: Entry) {
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
