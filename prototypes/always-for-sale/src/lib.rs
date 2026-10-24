//! Always For Sale: a Harberger-taxed chess variant. Rules core, bots and
//! self-play, shared by the sweep CLI and the browser build.

pub mod bot;
pub mod rules;
pub mod sim;
#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(test)]
mod tests;
