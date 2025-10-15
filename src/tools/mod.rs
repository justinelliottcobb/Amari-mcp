pub mod geometric_stub;
pub use geometric_stub as geometric;
pub mod tropical;
pub mod autodiff;
pub mod cellular_automata;
pub mod info_geometry;
pub mod gpu;

#[cfg(feature = "database")]
pub mod database;