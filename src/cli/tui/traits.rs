use ratatui::crossterm::event::KeyEvent;

pub trait HandleInput {
    fn handle_input(&mut self, key: KeyEvent) -> bool;
}

pub trait DisplayName {
    fn name(&self) -> &str;
}

pub trait KeyBindings {
    fn key_bindings(&self) -> String; // or Vec<String> if there are multiple bindings
}

pub trait IsActive {
    fn is_active(&self) -> bool;
    fn set_active(&mut self, state: bool);
}
