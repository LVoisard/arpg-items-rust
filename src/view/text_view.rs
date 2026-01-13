use crate::arpg_core::item::ItemRarity;

#[derive(Clone, Debug)]
pub struct TextView {
    pub value: String,
    pub style: TextStyle,
}

#[derive(Clone, Debug)]
pub enum TextStyle {
    Normal,
    Magic,
    Rare,
    Unique,
    UnfulfilledRequirement,
    ColorWhenModified,
}

impl TextStyle {
    pub fn from_rarity(rarity: ItemRarity) -> TextStyle {
        match rarity {
            ItemRarity::Normal => TextStyle::Normal,
            ItemRarity::Magic => TextStyle::Magic,
            ItemRarity::Rare => TextStyle::Rare,
            ItemRarity::Unique => TextStyle::Unique
        }
    }
}
