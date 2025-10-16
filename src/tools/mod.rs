pub mod geometric_stub;
pub use geometric_stub as geometric;
pub mod autodiff;
pub mod cayley_tables;
pub mod cellular_automata;
pub mod gpu;
pub mod info_geometry;
pub mod tropical;

#[cfg(feature = "database")]
pub mod database;

#[cfg(feature = "database")]
pub mod cayley_precompute;
