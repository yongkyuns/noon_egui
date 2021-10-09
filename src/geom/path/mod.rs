pub mod builder;

pub use nannou::geom::Path;

pub trait WithPath {
    fn path(&self) -> Path;
}
