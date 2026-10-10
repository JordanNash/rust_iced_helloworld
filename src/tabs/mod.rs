//! One module per tab. Each owns its own state, `Message` enum, `update`
//! and `view`; `main.rs` only routes between them.

pub mod animation;
pub mod async_task;
pub mod widgets;
