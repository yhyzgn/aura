//! Dynamic tray icon switching for running states.

use aura_tray::{
    AuraTray, BundledTrayIconSet, BundledTrayIconState, Result, TrayCommand, TrayIconImage,
    bundled_tray_icon,
};

pub fn icon_for_command(command: &TrayCommand) -> Result<Option<TrayIconImage>> {
    match command {
        TrayCommand::SetIcon(name) => Ok(Some(bundled_tray_icon(
            BundledTrayIconSet::Gallery,
            BundledTrayIconState::from_name(name),
        )?)),
        _ => Ok(None),
    }
}

pub fn apply_icon_command(tray: &AuraTray, command: &TrayCommand) -> Result<bool> {
    if let Some(icon) = icon_for_command(command)? {
        tray.set_icon(icon)?;
        Ok(true)
    } else {
        Ok(false)
    }
}
