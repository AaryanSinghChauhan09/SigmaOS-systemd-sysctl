// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-systemd-sysctl: systemd sysctl component model.
//!
//! Real implementation: sysctls, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sysctl {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemdSysctlError {
    EmptySysctlName,
    DuplicateSysctl(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemdSysctlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySysctlName => write!(f, "sysctl name must not be empty"),
            Self::DuplicateSysctl(id) => write!(f, "duplicate sysctl {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemdSysctlError {}

pub struct SystemdSysctl {
    pub name: String,
    sysctls: BTreeMap<String, Sysctl>,
    events: BTreeMap<String, Event>,
}

impl SystemdSysctl {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            sysctls: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_sysctl(&mut self, item: Sysctl) -> Result<(), SystemdSysctlError> {
        if item.name.trim().is_empty() {
            return Err(SystemdSysctlError::EmptySysctlName);
        }
        let id = item.id.clone();
        if self.sysctls.contains_key(&id) {
            return Err(SystemdSysctlError::DuplicateSysctl(id.clone()));
        }
        self.sysctls.insert(id, item);
        Ok(())
    }

    pub fn sysctl_count(&self) -> usize {
        self.sysctls.len()
    }

    pub fn get_sysctl(&self, id: &str) -> Option<&Sysctl> {
        self.sysctls.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemdSysctlError> {
        if event.subject.trim().is_empty() {
            return Err(SystemdSysctlError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_sysctl(id: &str, name: &str) -> Sysctl {
        Sysctl {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemdSysctl::new("SystemdSysctl");
        assert_eq!(m.name, "SystemdSysctl");
        assert_eq!(m.sysctl_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_sysctl() {
        let mut m = SystemdSysctl::new("SystemdSysctl");
        m.add_sysctl(sample_sysctl("a1", "Alpha")).unwrap();
        assert_eq!(m.sysctl_count(), 1);
        assert!(m.get_sysctl("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemdSysctl::new("SystemdSysctl");
        assert!(m.add_sysctl(sample_sysctl("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemdSysctl::new("SystemdSysctl");
        m.add_sysctl(sample_sysctl("x", "One")).unwrap();
        assert!(m.add_sysctl(sample_sysctl("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemdSysctl::new("SystemdSysctl");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
