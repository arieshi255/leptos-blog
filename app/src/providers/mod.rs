pub mod color_scheme;
pub use color_scheme::*;
pub mod auth_context;
pub use auth_context::*;

#[derive(Default, Clone)]
pub struct DummyContext {
  pub uuid: u64
}