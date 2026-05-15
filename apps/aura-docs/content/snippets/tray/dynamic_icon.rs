//! Dynamic tray icon switching for running states.

use aura_tray::{AuraTray, Result, TrayCommand, TrayIconImage, solid_icon};

pub fn icon_for_command(command: &TrayCommand) -> Result<Option<TrayIconImage>> {
    match command {
        TrayCommand::SetIcon(name) if name == "syncing" => {
            Ok(Some(solid_icon([230, 162, 60, 255], 32)?))
        }
        TrayCommand::SetIcon(name) if name == "error" => {
            Ok(Some(solid_icon([245, 108, 108, 255], 32)?))
        }
        TrayCommand::SetIcon(_) => Ok(Some(solid_icon([64, 158, 255, 255], 32)?)),
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
