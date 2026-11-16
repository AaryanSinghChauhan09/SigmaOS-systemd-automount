// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-systemd-automount: systemd automount component model.
//!
//! Real implementation: automounts, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Automount {
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
pub enum SystemdAutomountError {
    EmptyAutomountName,
    DuplicateAutomount(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemdAutomountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAutomountName => write!(f, "automount name must not be empty"),
            Self::DuplicateAutomount(id) => write!(f, "duplicate automount {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemdAutomountError {}

pub struct SystemdAutomount {
    pub name: String,
    automounts: BTreeMap<String, Automount>,
    events: BTreeMap<String, Event>,
}

impl SystemdAutomount {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            automounts: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_automount(&mut self, item: Automount) -> Result<(), SystemdAutomountError> {
        if item.name.trim().is_empty() {
            return Err(SystemdAutomountError::EmptyAutomountName);
        }
        let id = item.id.clone();
        if self.automounts.contains_key(&id) {
            return Err(SystemdAutomountError::DuplicateAutomount(id.clone()));
        }
        self.automounts.insert(id, item);
        Ok(())
    }

    pub fn automount_count(&self) -> usize {
        self.automounts.len()
    }

    pub fn get_automount(&self, id: &str) -> Option<&Automount> {
        self.automounts.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemdAutomountError> {
        if event.subject.trim().is_empty() {
            return Err(SystemdAutomountError::EmptyEventSubject);
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

    fn sample_automount(id: &str, name: &str) -> Automount {
        Automount {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemdAutomount::new("SystemdAutomount");
        assert_eq!(m.name, "SystemdAutomount");
        assert_eq!(m.automount_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_automount() {
        let mut m = SystemdAutomount::new("SystemdAutomount");
        m.add_automount(sample_automount("a1", "Alpha")).unwrap();
        assert_eq!(m.automount_count(), 1);
        assert!(m.get_automount("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemdAutomount::new("SystemdAutomount");
        assert!(m.add_automount(sample_automount("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemdAutomount::new("SystemdAutomount");
        m.add_automount(sample_automount("x", "One")).unwrap();
        assert!(m.add_automount(sample_automount("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemdAutomount::new("SystemdAutomount");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
