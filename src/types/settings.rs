//! Settings configuration types for Claude Code CLI

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use typed_builder::TypedBuilder;

/// Settings configuration for Claude Code CLI
#[derive(Debug, Clone)]
pub enum Settings {
    /// Path to a settings JSON file
    Path(PathBuf),
    /// Raw JSON string
    Json(String),
    /// Structured settings object
    Object(SettingsObject),
}

impl From<PathBuf> for Settings {
    fn from(path: PathBuf) -> Self {
        Settings::Path(path)
    }
}

impl From<&std::path::Path> for Settings {
    fn from(path: &std::path::Path) -> Self {
        Settings::Path(path.to_path_buf())
    }
}

impl From<SettingsObject> for Settings {
    fn from(obj: SettingsObject) -> Self {
        Settings::Object(obj)
    }
}

/// Structured settings object
#[derive(Debug, Clone, Default, Serialize, Deserialize, TypedBuilder)]
#[builder(doc)]
#[serde(rename_all = "camelCase")]
pub struct SettingsObject {
    /// Sandbox settings for bash command isolation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sandbox: Option<SandboxSettings>,
    /// Additional settings as raw JSON values
    #[serde(flatten)]
    #[builder(default)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Network configuration for sandbox.
#[derive(Debug, Clone, Default, Serialize, Deserialize, TypedBuilder)]
#[builder(doc)]
#[serde(rename_all = "camelCase")]
pub struct SandboxNetworkConfig {
    /// Unix socket paths accessible in sandbox (e.g., SSH agents).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub allow_unix_sockets: Option<Vec<String>>,
    /// Allow all Unix sockets (less secure).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub allow_all_unix_sockets: Option<bool>,
    /// Allow binding to localhost ports (macOS only).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub allow_local_binding: Option<bool>,
    /// HTTP proxy port if bringing your own proxy.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub http_proxy_port: Option<u16>,
    /// SOCKS5 proxy port if bringing your own proxy.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub socks_proxy_port: Option<u16>,
}

/// Violations to ignore in sandbox.
#[derive(Debug, Clone, Default, Serialize, Deserialize, TypedBuilder)]
#[builder(doc)]
pub struct SandboxIgnoreViolations {
    /// File paths for which violations should be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub file: Option<Vec<String>>,
    /// Network hosts for which violations should be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub network: Option<Vec<String>>,
}

/// Sandbox settings configuration.
///
/// This controls how Claude Code sandboxes bash commands for filesystem
/// and network isolation.
///
/// **Important:** Filesystem and network restrictions are configured via permission
/// rules, not via these sandbox settings:
/// - Filesystem read restrictions: Use Read deny rules
/// - Filesystem write restrictions: Use Edit allow/deny rules
/// - Network restrictions: Use WebFetch allow/deny rules
#[derive(Debug, Clone, Default, Serialize, Deserialize, TypedBuilder)]
#[builder(doc)]
#[serde(rename_all = "camelCase")]
pub struct SandboxSettings {
    /// Enable bash sandboxing (macOS/Linux only). Default: False
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enabled: Option<bool>,
    /// Auto-approve bash commands when sandboxed. Default: True
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub auto_allow_bash_if_sandboxed: Option<bool>,
    /// Commands that should run outside the sandbox (e.g., ["git", "docker"])
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub excluded_commands: Option<Vec<String>>,
    /// Allow commands to bypass sandbox via dangerouslyDisableSandbox.
    /// When False, all commands must run sandboxed (or be in excludedCommands). Default: True
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub allow_unsandboxed_commands: Option<bool>,
    /// Network configuration for sandbox.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub network: Option<SandboxNetworkConfig>,
    /// Violations to ignore.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub ignore_violations: Option<SandboxIgnoreViolations>,
    /// Enable weaker sandbox for unprivileged Docker environments
    /// (Linux only). Reduces security. Default: False
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enable_weaker_nested_sandbox: Option<bool>,
}
