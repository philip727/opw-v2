#[derive(Clone, PartialEq, Eq, Copy, Default)]
pub enum Page {
    None,
    #[default]
    Main,
    WorldSelection,
    Settings,
    Community,
}
