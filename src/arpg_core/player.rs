use crate::arpg_core::item::Item;
use crate::arpg_core::modifier::ModifierPass;
use crate::arpg_core::modifier::ModifierTarget::Character;
use crate::arpg_core::stat::StatBlock;

pub struct Player<'a>{
    pub base_stats: StatBlock,
    pub equipped_items: Vec<&'a Item>,
}

impl<'a> Player<'a> {
    pub fn equip(&mut self, item: &'a Item) {
        self.equipped_items.push(item);
    }

    pub fn get_derived_stats(&self) -> StatBlock {
        let mut base_stats = self.base_stats.clone();
        for item in self.equipped_items.iter() {

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