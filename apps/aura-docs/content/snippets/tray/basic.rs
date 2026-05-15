//! Basic aura-tray installation shape with bundled demo icons.
//!
//! In a real GPUI app, keep the returned `AuraTray` in your app state and set
//! the platform quit mode to `gpui::QuitMode::Explicit` before opening windows.

use aura_tray::{
    AuraTray, BundledTrayIconSet, BundledTrayIconState, Result, TrayConfig, bundled_tray_icon,
    default_aura_tray_menu,
};

pub fn install_basic_tray() -> Result<AuraTray> {
    let icon = bundled_tray_icon(BundledTrayIconSet::Gallery, BundledTrayIconState::Default)?;

    AuraTray::install(
        TrayConfig::new("aura-gallery")
            .tooltip("Aura Gallery")
            .icon(icon)
            .menu(default_aura_tray_menu()),
    )
}
