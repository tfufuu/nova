// system/src/compositor/mod.rs

// Declare existing sub-modules (like core) and the new renderer_interface
// pub mod core; // Assuming 'core' is part of novade_system::compositor, not system::compositor
pub mod renderer_interface;
// Other submodules like texture_management, rendering_logic, damage_tracking
// will be added here as they are implemented.

// Optional: Re-export key types from the renderer_interface for convenience
// pub use renderer_interface::{
//     RenderableTexture,
//     FrameRenderer,
//     ActiveFrame,
//     RenderElement,
//     RendererError
// };
// For now, we will stick to requiring explicit path `renderer_interface::TypeName`.
