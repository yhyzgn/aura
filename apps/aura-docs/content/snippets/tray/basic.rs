//! Basic aura-tray installation shape.
//!
//! In a real GPUI app, keep the returned `AuraTray` in your app state and set
//! the platform quit mode to `gpui::QuitMode::Explicit` before opening windows.

use aura_tray::{AuraTray, Result, TrayConfig, default_aura_tray_menu, solid_icon};

pub fn install_basic_tray() -> Result<AuraTray> {
    let icon = solid_icon([64, 158, 255, 255], 32)?;

    AuraTray::install(
        TrayConfig::new("aura-gallery")
            .tooltip("Aura Gallery")
            .icon(icon)
            .menu(default_aura_tray_menu()),
    )
}
