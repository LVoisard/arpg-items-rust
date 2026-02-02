use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;
use crate::ui::ratatui::view_models::item::ItemViewModel;

pub struct ItemPopupWidget<'a> {
    pub item: &'a ItemViewModel
}

impl<'a> Widget for ItemPopupWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        todo!()
    }
}