use crate::view::item_view::ItemView;


pub trait UI {
    fn display_item_view(&self, item_view: &ItemView);
}