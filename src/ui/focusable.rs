pub trait Focusable {
    fn on_focus_gained(&mut self);
    fn on_focus_lost(&mut self);
}