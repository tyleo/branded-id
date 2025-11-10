mod log_sys;
mod no_engine_sys;
mod window_sys;

pub use log_sys::*;
pub use no_engine_sys::*;
pub use window_sys::*;

pub use no_engine_abstractions::*;

// # Capabilities required for 1-D games:
// 1. Ability to create a 1xX window
// 2. Ability to color a pixel
// 3. Ability to handle touch input
// 4. Ability to handle keyboard input
// 5. Ability to load a sound effect
// 6. Ability to play a sound effect
