use crate::model::item::Item;

pub struct Inventory {
    pub(super) items: Vec<Item>
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: vec![],
        }
    }

    pub fn add(&mut self, item: Item) {
        self.items.push(item);
    }
    
    pub fn iter(&'_ self) -> core::slice::Iter<'_, Item>{
        self.items.iter()
    }
}

impl IntoIterator for Inventory {
    type Item = Item;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}