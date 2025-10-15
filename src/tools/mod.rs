pub mod geometric_stub;
pub use geometric_stub as geometric;
pub mod tropical;
pub mod autodiff;
pub mod cellular_automata;
pub mod info_geometry;
pub mod gpu;
pub mod cayley_tables;

#[cfg(feature = "database")]
pub mod database;

#[cfg(feature = "database")]
pub mod cayley_precompute;