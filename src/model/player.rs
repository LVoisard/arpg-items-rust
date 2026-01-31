use crate::model::inventory::Inventory;
use crate::model::item::Item;
use crate::model::modifier::ModifierTarget::Character;
use crate::model::stat::StatBlock;

pub struct Player {
    pub base_stats: StatBlock,
    pub inventory: Inventory,
    pub equippement: Inventory,
}

impl Player {
    pub fn equip(&mut self, item: Item) {
        self.equippement.add(item);
    }

    pub fn unnequip(&mut self, item: &Item) -> Option<Item> {
        if let Some(index) = self.equippement.iter().position(|x| x == item) {
            return Some(self.equippement.items.remove(index));
        }
        None
    }

    pub fn pickup(&mut self, item: Item) {
        self.inventory.add(item);
    }

    pub fn drop(&mut self, item: &Item) -> Option<Item>{
        if let Some(index) = self.inventory.iter().position(|x| x == item) {
            return Some(self.inventory.items.remove(index));
        }
        None
    }

    pub fn get_derived_stats(&self) -> StatBlock {
        let mut base_stats = self.base_stats.clone();
        for item in self.equippement.iter() {
            for stat in item.get_derived_stats().stats.iter() {
                if !self.base_stats.has(stat.stat_type) {
                    base_stats.add(stat.clone())
                }
            }

            for modifier in item.modifiers.iter() {
                modifier.apply_to(Character(&mut base_stats))
            }
        }
        base_stats
    }
}
