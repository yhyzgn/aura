use std::{env, path::PathBuf, process::Command};

use aura_packager::{KnownApp, PackageFormat, Platform, validate_packaging_layout};

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("package") => package(args.collect()),
        Some("help") | Some("--help") | Some("-h") | None => {
            print_help();
            Ok(())
        }
        Some(other) => Err(format!("unknown xtask command '{other}'")),
    }
}

fn package(args: Vec<String>) -> Result<(), String> {
    let command = PackageCommand::parse(args)?;
    match command.action {
        PackageAction::Validate => validate(),
        PackageAction::Build => build(command.apps),
        PackageAction::Package => package_formats(command.apps, command.format),
    }
}

fn validate() -> Result<(), String> {
    let root = workspace_root()?;
    let report = validate_packaging_layout(&root);
    if report.is_ok() {
        println!("packaging layout OK");
        return Ok(());
    }

    for error in report.errors {
        eprintln!("- {error}");
    }
    Err("packaging layout validation failed".into())
}

fn build(apps: Vec<KnownApp>) -> Result<(), String> {
    for app in apps {
        let status = Command::new("cargo")
            .args(["build", "--release", "-p", app.package()])
            .status()
            .map_err(|error| format!("failed to spawn cargo build: {error}"))?;
        if !status.success() {
            return Err(format!("cargo build failed for {}", app.package()));
        }
    }
    Ok(())
}

fn package_formats(apps: Vec<KnownApp>, format: PackageFormat) -> Result<(), String> {
    validate()?;
    build(apps.clone())?;

    let platform = Platform::current();
    let formats: Vec<_> = if format == PackageFormat::PlatformDefaults {
        PackageFormat::defaults_for(platform).to_vec()
    } else {
        vec![format]
    };

    for app in apps {
        for format in &formats {
            println!(
                "planned package: app={} platform={} format={} output=target/packages/{}/{}/",
                app.package(),
                platform.as_str(),
                format.as_str(),
                app.package(),
                platform.as_str()
            );
        }
    }

    println!(
        "packager backend invocation is intentionally staged; metadata/build validation is active"
    );
    Ok(())
}

#[derive(Debug)]
struct PackageCommand {
    action: PackageAction,
    apps: Vec<KnownApp>,
    format: PackageFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PackageAction {
    Validate,
    Build,
    Package,
}

impl PackageCommand {
    fn parse(args: Vec<String>) -> Result<Self, String> {
        let mut action = PackageAction::Package;
        let mut app: Option<KnownApp> = None;
        let mut all_apps = false;
        let mut format = PackageFormat::PlatformDefaults;
        let mut iter = args.into_iter();

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "validate" => action = PackageAction::Validate,
                "build" => action = PackageAction::Build,
                "--all-apps" => all_apps = true,
                "--app" => {
                    let value = iter.next().ok_or("--app requires a value")?;
                    app = Some(value.parse()?);
                }
                "--format" => {
                    let value = iter.next().ok_or("--format requires a value")?;
                    format = value.parse()?;
                }
                "--help" | "-h" => {
                    print_help();
                    std::process::exit(0);
                }
                other => return Err(format!("unknown package argument '{other}'")),
            }
        }

        let apps = if all_apps {
            aura_packager::known_apps().to_vec()
        } else {
            vec![app.unwrap_or(KnownApp::Gallery)]
        };

        Ok(Self {
            action,
            apps,
            format,
        })
    }
}

fn workspace_root() -> Result<PathBuf, String> {
    env::current_dir().map_err(|error| format!("failed to read current directory: {error}"))
}

fn print_help() {
    println!(
        "Aura xtask\n\n  cargo xtask package validate\n  cargo xtask package build --app gallery\n  cargo xtask package --app docs --format appimage\n  cargo xtask package --all-apps --format platform-defaults\n\nOptions:\n  --app <gallery|docs>\n  --all-apps\n  --format <appimage|deb|rpm|tar.gz|app|dmg|nsis|msi|platform-defaults>"
    );
}
