use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Platform path strategy, injectable for truthful cross-platform tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Platform {
    /// XDG Linux.
    Linux,
    /// macOS application-support conventions.
    MacOs,
    /// Windows known-folder conventions.
    Windows,
}

/// Explicit path inputs. Production environment reading belongs at the composition boundary.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PathEnvironment {
    /// User home.
    pub home: Option<PathBuf>,
    /// Linux XDG config override.
    pub xdg_config_home: Option<PathBuf>,
    /// Linux XDG data override.
    pub xdg_data_home: Option<PathBuf>,
    /// Linux XDG cache override.
    pub xdg_cache_home: Option<PathBuf>,
    /// Linux XDG state override.
    pub xdg_state_home: Option<PathBuf>,
    /// Windows roaming application data.
    pub app_data: Option<PathBuf>,
    /// Windows local application data.
    pub local_app_data: Option<PathBuf>,
}

/// Independent application roots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JarvisPaths {
    /// Configuration and roaming-safe preferences.
    pub config: PathBuf,
    /// Durable application data.
    pub data: PathBuf,
    /// Rebuildable caches.
    pub cache: PathBuf,
    /// Machine/process state.
    pub state: PathBuf,
    /// Logs, separate where platform conventions justify it.
    pub logs: PathBuf,
}

impl JarvisPaths {
    /// Validate explicit private overrides, then initialize only independent roots.
    pub fn initialize_environment() -> std::io::Result<Self> {
        let paths = Self::validate_environment()?;
        paths.initialize()?;
        Ok(paths)
    }

    /// Read-only host bootstrap; read-only ACP sessions must not create state.
    pub fn validate_environment() -> std::io::Result<Self> {
        let paths = Self::from_environment()
            .map_err(|_| std::io::Error::other("private application roots unavailable"))?;
        for name in [
            "JARVIS_HOME",
            "JARVIS_CONFIG_DIR",
            "JARVIS_MEMORY_ROOT",
            "JARVIS_GLOBAL_MEMORY_ROOT",
            "JARVIS_COGNITION_ROOT",
            "JARVIS_GLOBAL_COGNITION_ROOT",
            "JARVIS_CHECKPOINT_ROOT",
            "JARVIS_MCP_ROOT",
            "JARVIS_SESSION_ROOT",
            "JARVIS_SESSION_WRITE_ROOT",
        ] {
            if let Some(path) = std::env::var_os(name) {
                let path = PathBuf::from(path);
                jarvis_security::validate_private_path(&path)?;
                if path.exists() && std::fs::read_dir(&path)?.next().is_some() {
                    jarvis_security::validate_owned_directory(&path)?;
                }
            }
        }
        for name in [
            "JARVIS_CREDENTIALS_PATH",
            "JARVIS_OPENAI_CREDENTIALS_PATH",
            "JARVIS_XAI_CREDENTIALS_PATH",
            "JARVIS_LMSTUDIO_CREDENTIALS_PATH",
            "JARVIS_TRAJECTORY_PATH",
            "JARVIS_VOICE_VENV",
            "JARVIS_VOICE_VENV_PATH",
        ] {
            if let Some(path) = std::env::var_os(name) {
                jarvis_security::validate_private_path(Path::new(&path))?;
            }
        }
        for path in [
            &paths.config,
            &paths.data,
            &paths.state,
            &paths.cache,
            &paths.logs,
        ] {
            jarvis_security::validate_new_or_owned_directory(path)?;
        }
        Ok(paths)
    }

    /// Initialize injected roots without reading process environment or importing state.
    pub fn initialize(&self) -> std::io::Result<()> {
        for path in [
            &self.config,
            &self.data,
            &self.state,
            &self.cache,
            &self.logs,
        ] {
            jarvis_security::prepare_owned_directory(path)?;
        }
        Ok(())
    }

    /// Resolves only this application's platform roots. Does not create or import state.
    pub fn from_environment() -> Result<Self, VesperPathsError> {
        let value = |name| std::env::var_os(name).map(PathBuf::from);
        let mut paths = Self::resolve(
            if cfg!(windows) {
                Platform::Windows
            } else if cfg!(target_os = "macos") {
                Platform::MacOs
            } else {
                Platform::Linux
            },
            &PathEnvironment {
                home: value("HOME").or_else(|| value("USERPROFILE")),
                xdg_config_home: value("XDG_CONFIG_HOME"),
                xdg_data_home: value("XDG_DATA_HOME"),
                xdg_cache_home: value("XDG_CACHE_HOME"),
                xdg_state_home: value("XDG_STATE_HOME"),
                app_data: value("APPDATA"),
                local_app_data: value("LOCALAPPDATA"),
            },
        )?;
        if let Some(root) = value("JARVIS_CONFIG_DIR") {
            paths.config = root;
        }
        Ok(paths)
    }

    /// Resolves roots without creating any directory.
    pub fn resolve(
        platform: Platform,
        environment: &PathEnvironment,
    ) -> Result<Self, VesperPathsError> {
        match platform {
            Platform::Linux => {
                let home = environment
                    .home
                    .as_deref()
                    .ok_or(VesperPathsError::MissingHome)?;
                let config_base = environment
                    .xdg_config_home
                    .as_ref()
                    .filter(|base| base.is_absolute())
                    .cloned()
                    .unwrap_or_else(|| home.join(".config"));
                let data_base = environment
                    .xdg_data_home
                    .as_ref()
                    .filter(|base| base.is_absolute())
                    .cloned()
                    .unwrap_or_else(|| home.join(".local/share"));
                let cache_base = environment
                    .xdg_cache_home
                    .as_ref()
                    .filter(|base| base.is_absolute())
                    .cloned()
                    .unwrap_or_else(|| home.join(".cache"));
                let state_base = environment
                    .xdg_state_home
                    .as_ref()
                    .filter(|base| base.is_absolute())
                    .cloned()
                    .unwrap_or_else(|| home.join(".local/state"));
                Ok(Self {
                    config: config_base.join("jarvis"),
                    data: data_base.join("jarvis"),
                    cache: cache_base.join("jarvis"),
                    state: state_base.join("jarvis"),
                    logs: state_base.join("jarvis/logs"),
                })
            }
            Platform::MacOs => {
                let home = environment
                    .home
                    .as_deref()
                    .ok_or(VesperPathsError::MissingHome)?;
                let support = home.join("Library/Application Support/Jarvis");
                Ok(Self {
                    config: support.join("Config"),
                    data: support.join("Data"),
                    cache: home.join("Library/Caches/Jarvis"),
                    state: support.join("State"),
                    logs: home.join("Library/Logs/Jarvis"),
                })
            }
            Platform::Windows => {
                let roaming = environment
                    .app_data
                    .as_deref()
                    .ok_or(VesperPathsError::RoamingAppDataAbsent)?;
                let local = environment
                    .local_app_data
                    .as_deref()
                    .ok_or(VesperPathsError::NoLocalAppData)?;
                Ok(Self {
                    config: roaming.join("Jarvis"),
                    data: local.join("Jarvis/Data"),
                    cache: local.join("Jarvis/Cache"),
                    state: local.join("Jarvis/State"),
                    logs: local.join("Jarvis/Logs"),
                })
            }
        }
    }
}

/// Independent Agent Vesper application roots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VesperPaths {
    /// Configuration and roaming-safe preferences.
    pub config: PathBuf,
    /// Durable application data.
    pub data: PathBuf,
    /// Rebuildable caches.
    pub cache: PathBuf,
    /// Machine/process state.
    pub state: PathBuf,
    /// Logs, separate where platform conventions justify it.
    pub logs: PathBuf,
}

impl VesperPaths {
    /// Resolves roots without creating any directory.
    pub fn resolve(
        platform: Platform,
        environment: &PathEnvironment,
    ) -> Result<Self, VesperPathsError> {
        match platform {
            Platform::Linux => {
                let home = environment
                    .home
                    .as_deref()
                    .ok_or(VesperPathsError::MissingHome)?;
                let config_base = environment
                    .xdg_config_home
                    .clone()
                    .unwrap_or_else(|| home.join(".config"));
                let data_base = environment
                    .xdg_data_home
                    .clone()
                    .unwrap_or_else(|| home.join(".local/share"));
                let cache_base = environment
                    .xdg_cache_home
                    .clone()
                    .unwrap_or_else(|| home.join(".cache"));
                let state_base = environment
                    .xdg_state_home
                    .clone()
                    .unwrap_or_else(|| home.join(".local/state"));
                Ok(Self {
                    config: config_base.join("agent-vesper"),
                    data: data_base.join("agent-vesper"),
                    cache: cache_base.join("agent-vesper"),
                    state: state_base.join("agent-vesper"),
                    logs: state_base.join("agent-vesper/logs"),
                })
            }
            Platform::MacOs => {
                let home = environment
                    .home
                    .as_deref()
                    .ok_or(VesperPathsError::MissingHome)?;
                let support = home.join("Library/Application Support/Agent Vesper");
                Ok(Self {
                    config: support.join("Config"),
                    data: support.join("Data"),
                    cache: home.join("Library/Caches/Agent Vesper"),
                    state: support.join("State"),
                    logs: home.join("Library/Logs/Agent Vesper"),
                })
            }
            Platform::Windows => {
                let roaming = environment
                    .app_data
                    .as_deref()
                    .ok_or(VesperPathsError::RoamingAppDataAbsent)?;
                let local = environment
                    .local_app_data
                    .as_deref()
                    .ok_or(VesperPathsError::NoLocalAppData)?;
                Ok(Self {
                    config: roaming.join("Agent Vesper"),
                    data: local.join("Agent Vesper/Data"),
                    cache: local.join("Agent Vesper/Cache"),
                    state: local.join("Agent Vesper/State"),
                    logs: local.join("Agent Vesper/Logs"),
                })
            }
        }
    }

    /// Describes legacy paths for read-only discovery without probing or writing them.
    #[must_use]
    pub fn legacy_locations(
        platform: Platform,
        environment: &PathEnvironment,
        project_root: Option<&Path>,
    ) -> Vec<LegacyLocation> {
        let mut locations = Vec::new();
        if let Some(home) = &environment.home {
            locations.push(LegacyLocation::read_only(
                LegacyLocationKind::GlobalState,
                home.join(".glm-acp"),
            ));
        }
        let config = match platform {
            Platform::Linux => environment
                .xdg_config_home
                .clone()
                .or_else(|| environment.home.as_ref().map(|home| home.join(".config")))
                .map(|base| base.join("glm-acp")),
            Platform::MacOs => environment
                .home
                .as_ref()
                .map(|home| home.join("Library/Application Support/glm-acp")),
            Platform::Windows => environment
                .app_data
                .as_ref()
                .map(|base| base.join("glm-acp")),
        };
        if let Some(config) = config {
            locations.push(LegacyLocation::read_only(
                LegacyLocationKind::PlatformConfig,
                config,
            ));
        }
        if let Some(project_root) = project_root {
            locations.push(LegacyLocation::read_only(
                LegacyLocationKind::ProjectLocal,
                project_root.join(".glm-acp"),
            ));
        }
        locations
    }
}

/// Kind of Native GLM ACP compatibility location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LegacyLocationKind {
    /// Historical `~/.glm-acp`.
    GlobalState,
    /// Platform config root.
    PlatformConfig,
    /// Project-local memory/skills/evaluation root.
    ProjectLocal,
}

/// Legacy location descriptor. Access is structurally read-only in Stage 1.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegacyLocation {
    /// Store kind.
    pub kind: LegacyLocationKind,
    /// Legacy path.
    pub path: PathBuf,
    /// Always true for this Stage 1 descriptor.
    pub read_only: bool,
}

impl LegacyLocation {
    fn read_only(kind: LegacyLocationKind, path: PathBuf) -> Self {
        Self {
            kind,
            path,
            read_only: true,
        }
    }
}

/// Required platform input is absent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum VesperPathsError {
    /// Home is required.
    #[error("home directory is required")]
    MissingHome,
    /// Windows roaming application data is required.
    #[error("Windows roaming application data is required")]
    RoamingAppDataAbsent,
    /// Windows local application data is required.
    #[error("Windows local application data is required")]
    NoLocalAppData,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linux_xdg_categories_are_separate_and_overrideable() {
        let environment = PathEnvironment {
            home: Some(PathBuf::from("/home/test")),
            xdg_config_home: Some(PathBuf::from("/xdg/config")),
            xdg_data_home: Some(PathBuf::from("/xdg/data")),
            xdg_cache_home: Some(PathBuf::from("/xdg/cache")),
            xdg_state_home: Some(PathBuf::from("/xdg/state")),
            ..PathEnvironment::default()
        };
        let paths = VesperPaths::resolve(Platform::Linux, &environment).unwrap();
        assert_eq!(paths.config, PathBuf::from("/xdg/config/agent-vesper"));
        assert_eq!(paths.data, PathBuf::from("/xdg/data/agent-vesper"));
        assert_eq!(paths.cache, PathBuf::from("/xdg/cache/agent-vesper"));
        assert_eq!(paths.state, PathBuf::from("/xdg/state/agent-vesper"));
    }

    #[test]
    fn macos_and_windows_use_injected_platform_strategies() {
        let mac = PathEnvironment {
            home: Some(PathBuf::from("/Users/test")),
            ..PathEnvironment::default()
        };
        let paths = VesperPaths::resolve(Platform::MacOs, &mac).unwrap();
        assert!(
            paths
                .config
                .ends_with("Library/Application Support/Agent Vesper/Config")
        );
        assert!(paths.cache.ends_with("Library/Caches/Agent Vesper"));

        let windows = PathEnvironment {
            app_data: Some(PathBuf::from(r"C:\Roaming")),
            local_app_data: Some(PathBuf::from(r"C:\Local")),
            ..PathEnvironment::default()
        };
        let paths = VesperPaths::resolve(Platform::Windows, &windows).unwrap();
        assert!(paths.config.ends_with("Agent Vesper"));
        assert!(paths.cache.ends_with("Agent Vesper/Cache"));
    }

    #[test]
    fn legacy_discovery_is_read_only_and_never_renames_project_state() {
        let environment = PathEnvironment {
            home: Some(PathBuf::from("/home/test")),
            ..PathEnvironment::default()
        };
        let locations =
            VesperPaths::legacy_locations(Platform::Linux, &environment, Some(Path::new("/work")));
        assert!(locations.iter().all(|location| location.read_only));
        assert!(
            locations
                .iter()
                .any(|location| location.path == Path::new("/work/.glm-acp"))
        );
        assert!(
            !locations
                .iter()
                .any(|location| location.path.ends_with(".agent-vesper"))
        );
    }
}

#[cfg(test)]
mod runtime_isolation_tests {
    use super::*;

    #[test]
    fn first_run_preserves_all_synthetic_vesper_roots_and_user_project_files() {
        let root = tempfile::tempdir().unwrap();
        let env = PathEnvironment {
            home: Some(root.path().join("home")),
            xdg_config_home: Some(root.path().join("config")),
            xdg_data_home: Some(root.path().join("data")),
            xdg_cache_home: Some(root.path().join("cache")),
            xdg_state_home: Some(root.path().join("state")),
            app_data: Some(root.path().join("roaming")),
            local_app_data: Some(root.path().join("local")),
        };
        let project = root.path().join("user-project");
        std::fs::create_dir(&project).unwrap();
        std::fs::write(project.join("notes.txt"), b"user-owned shared resource").unwrap();
        for platform in [Platform::Linux, Platform::MacOs, Platform::Windows] {
            let old = VesperPaths::resolve(platform, &env).unwrap();
            let new = JarvisPaths::resolve(platform, &env).unwrap();
            for path in [&old.config, &old.data, &old.state, &old.cache, &old.logs] {
                std::fs::create_dir_all(path).unwrap();
                std::fs::write(path.join("synthetic-record"), b"do-not-import-or-replay").unwrap();
            }
            new.initialize().unwrap();
            new.initialize().unwrap();
            for path in [&new.config, &new.data, &new.state, &new.cache, &new.logs] {
                jarvis_security::validate_owned_directory(path).unwrap();
                assert!(!path.join("synthetic-record").exists());
            }
            for path in [&old.config, &old.data, &old.state, &old.cache, &old.logs] {
                assert_eq!(
                    std::fs::read(path.join("synthetic-record")).unwrap(),
                    b"do-not-import-or-replay"
                );
                assert!(!path.join(".jarvis-owner").exists());
            }
        }
        assert_eq!(
            std::fs::read(project.join("notes.txt")).unwrap(),
            b"user-owned shared resource"
        );
    }
}
