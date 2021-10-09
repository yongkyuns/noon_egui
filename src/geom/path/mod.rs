//! Build different kinds of 2D shapes.
pub mod arc;

pub mod builder;

#[doc(no_inline)]
pub use arc::Arc;
pub use iced_graphics::canvas::Path;

pub trait WithPath {
    fn path(&self) -> Path;
}
