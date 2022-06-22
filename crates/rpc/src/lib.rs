pub use tonic;

#[cfg(not(feature = "server"))]
#[rustfmt::skip]
mod generated;
#[cfg(not(feature = "server"))]
pub use generated::*;

#[cfg(feature = "server")]
#[rustfmt::skip]
mod generated_full;
#[cfg(feature = "server")]
pub use generated_full::*;
