use crate::core::failable::Failable;

const ANDROID: &str = "android";
const IOS: &str = "ios";
const EMSCRIPTEN: &str = "emscripten";
const MACOS_CONSOLE: &str = "macos-console";
const MACOS_DESKTOP: &str = "macos-desktop";
const WINDOWS: &str = "windows";
const LINUX: &str = "linux";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    Android,
    Ios,
    Emscripten,
    MacOSConsole,
    MacOSDesktop,
    Windows,
    Linux,
}

impl Target {
    pub fn resolve(id: &str) -> Failable<Target> {
        match &*id.to_lowercase() {
            ANDROID => Ok(Target::Android),
            IOS => Ok(Target::Ios),
            EMSCRIPTEN => Ok(Target::Emscripten),
            MACOS_CONSOLE => Ok(Target::MacOSConsole),
            MACOS_DESKTOP => Ok(Target::MacOSDesktop),
            WINDOWS => Ok(Target::Windows),
            LINUX => Ok(Target::Linux),
            _ => Err("Unknown target id".into()),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Target::Android => ANDROID,
            Target::Ios => IOS,
            Target::Emscripten => EMSCRIPTEN,
            Target::MacOSConsole => MACOS_CONSOLE,
            Target::MacOSDesktop => MACOS_DESKTOP,
            Target::Windows => WINDOWS,
            Target::Linux => LINUX,
        }
    }
}