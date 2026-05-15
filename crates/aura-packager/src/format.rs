#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    Linux,
    Macos,
    Windows,
}

impl Platform {
    pub fn current() -> Self {
        if cfg!(target_os = "macos") {
            Self::Macos
        } else if cfg!(target_os = "windows") {
            Self::Windows
        } else {
            Self::Linux
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Linux => "linux",
            Self::Macos => "macos",
            Self::Windows => "windows",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageFormat {
    AppImage,
    Deb,
    Rpm,
    TarGz,
    App,
    Dmg,
    Nsis,
    Msi,
    PlatformDefaults,
}

impl PackageFormat {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AppImage => "appimage",
            Self::Deb => "deb",
            Self::Rpm => "rpm",
            Self::TarGz => "tar.gz",
            Self::App => "app",
            Self::Dmg => "dmg",
            Self::Nsis => "nsis",
            Self::Msi => "msi",
            Self::PlatformDefaults => "platform-defaults",
        }
    }

    pub fn defaults_for(platform: Platform) -> &'static [Self] {
        match platform {
            Platform::Linux => &[Self::AppImage, Self::Deb, Self::Rpm, Self::TarGz],
            Platform::Macos => &[Self::App, Self::Dmg],
            Platform::Windows => &[Self::Nsis, Self::Msi],
        }
    }
}

impl std::str::FromStr for PackageFormat {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "appimage" | "AppImage" => Ok(Self::AppImage),
            "deb" => Ok(Self::Deb),
            "rpm" => Ok(Self::Rpm),
            "tar.gz" | "tgz" | "archive" => Ok(Self::TarGz),
            "app" => Ok(Self::App),
            "dmg" => Ok(Self::Dmg),
            "nsis" | "exe" => Ok(Self::Nsis),
            "msi" => Ok(Self::Msi),
            "platform-defaults" | "defaults" => Ok(Self::PlatformDefaults),
            other => Err(format!("unknown package format '{other}'")),
        }
    }
}
