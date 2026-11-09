//! Shared data model for rustterm.
//!
//! This is original clean-room code. It models sessions and settings from
//! published SSH/telnet conventions and the product's public feature set; it
//! does not contain code or constants taken from any proprietary binary.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Protocol {
    Local,
    Ssh,
    Telnet,
    Serial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthMethod {
    Password,
    PublicKey { key_path: String },
    Agent,
    KeyboardInteractive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub name: String,
    pub protocol: Protocol,
    #[serde(default)]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub auth: Option<AuthMethod>,
    #[serde(default)]
    pub folder: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

fn default_port() -> u16 {
    22
}

impl Session {
    /// A local shell pseudo-session (no network transport).
    pub fn local(name: impl Into<String>) -> Self {
        Self {
            id: uuid_like(),
            name: name.into(),
            protocol: Protocol::Local,
            host: String::new(),
            port: 0,
            username: String::new(),
            auth: None,
            folder: None,
            tags: vec![],
        }
    }
}

/// Small dependency-free id generator (not cryptographic).
fn uuid_like() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let n = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("s-{n:x}")
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub theme: String,
    #[serde(default)]
    pub font_size: u16,
    #[serde(default)]
    pub scrollback: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_session_roundtrips() {
        let s = Session::local("shell");
        let j = serde_json::to_string(&s).unwrap();
        let back: Session = serde_json::from_str(&j).unwrap();
        assert_eq!(back.name, "shell");
        assert!(matches!(back.protocol, Protocol::Local));
    }
}
