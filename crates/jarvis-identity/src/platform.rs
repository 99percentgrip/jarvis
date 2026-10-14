//! Injected path specifications, never environment discovery or directory creation.
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    LinuxX86,
    LinuxArm,
    MacIntel,
    MacArm,
    WindowsX86,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathInputs {
    pub home: PathBuf,
    pub config_base: PathBuf,
    pub data_base: PathBuf,
    pub cache_base: PathBuf,
    pub state_base: PathBuf,
    pub roaming: PathBuf,
    pub local: PathBuf,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedPaths {
    pub config: PathBuf,
    pub data: PathBuf,
    pub cache: PathBuf,
    pub state: PathBuf,
    pub logs: PathBuf,
    pub install: PathBuf,
    pub project_directory: &'static str,
    pub scope_stamp: &'static str,
}
impl PlannedPaths {
    pub fn for_platform(platform: Platform, inputs: &PathInputs) -> Self {
        let (config, data, cache, state, logs, install) = match platform {
            Platform::LinuxX86 | Platform::LinuxArm => (
                inputs.config_base.join("jarvis"),
                inputs.data_base.join("jarvis"),
                inputs.cache_base.join("jarvis"),
                inputs.state_base.join("jarvis"),
                inputs.state_base.join("jarvis/logs"),
                inputs.data_base.join("jarvis/install"),
            ),
            Platform::MacIntel | Platform::MacArm => {
                let support = inputs.home.join("Library/Application Support/Jarvis");
                (
                    support.join("Config"),
                    support.join("Data"),
                    inputs.home.join("Library/Caches/Jarvis"),
                    support.join("State"),
                    inputs.home.join("Library/Logs/Jarvis"),
                    support.join("Install"),
                )
            }
            Platform::WindowsX86 => (
                inputs.roaming.join("Jarvis"),
                inputs.local.join("Jarvis/Data"),
                inputs.local.join("Jarvis/Cache"),
                inputs.local.join("Jarvis/State"),
                inputs.local.join("Jarvis/Logs"),
                inputs.local.join("Programs/Jarvis"),
            ),
        };
        Self {
            config,
            data,
            cache,
            state,
            logs,
            install,
            project_directory: ".jarvis",
            scope_stamp: ".jarvis-scope-id",
        }
    }
    pub fn release_ledger(&self) -> PathBuf {
        self.state.join("release-recovery")
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Distribution {
    pub platform: Platform,
    pub target: &'static str,
    pub archive: &'static str,
    pub launcher_suffix: &'static str,
}
pub const DISTRIBUTIONS: &[Distribution] = &[
    Distribution {
        platform: Platform::LinuxX86,
        target: "linux-x86_64",
        archive: "jarvis-linux-x86_64.tar.gz",
        launcher_suffix: "",
    },
    Distribution {
        platform: Platform::LinuxArm,
        target: "linux-aarch64",
        archive: "jarvis-linux-aarch64.tar.gz",
        launcher_suffix: "",
    },
    Distribution {
        platform: Platform::MacIntel,
        target: "darwin-x86_64",
        archive: "jarvis-darwin-x86_64.tar.gz",
        launcher_suffix: "",
    },
    Distribution {
        platform: Platform::MacArm,
        target: "darwin-aarch64",
        archive: "jarvis-darwin-aarch64.tar.gz",
        launcher_suffix: "",
    },
    Distribution {
        platform: Platform::WindowsX86,
        target: "windows-x86_64",
        archive: "jarvis-windows-x86_64.zip",
        launcher_suffix: ".exe",
    },
];
/// Proposed independent packaging contracts; no updater or installer is activated.
pub const CHECKSUM_SUFFIX: &str = ".sha256";
pub const INSTALLER_POSIX: &str = "install.sh";
pub const INSTALLER_WINDOWS: &str = "install.ps1";
pub const INSTALLER_REPOSITORY: &str = "99percentgrip/jarvis";
