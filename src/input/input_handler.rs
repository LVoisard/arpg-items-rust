pub trait InputHandler {
    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent);
}