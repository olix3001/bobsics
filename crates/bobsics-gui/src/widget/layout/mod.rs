use crate::widgets::Vector2;

mod horizontal_stack;
mod vertical_stack;

pub mod layouts {
    pub use super::horizontal_stack::*;
    pub use super::vertical_stack::*;
}

pub struct LayoutCommonOptions {
    padding: Vector2,
    margin: Vector2,
    spacing: Vector2,
}
