// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-app-switcher: app switcher component model.
//!
//! Real implementation: switchers, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Switcher {
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
pub enum AppSwitcherError {
    EmptySwitcherName,
    DuplicateSwitcher(String),
    EmptyEventSubject,
}

impl fmt::Display for AppSwitcherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySwitcherName => write!(f, "switcher name must not be empty"),
            Self::DuplicateSwitcher(id) => write!(f, "duplicate switcher {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for AppSwitcherError {}

pub struct AppSwitcher {
    pub name: String,
    switchers: BTreeMap<String, Switcher>,
    events: BTreeMap<String, Event>,
}

impl AppSwitcher {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            switchers: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_switcher(&mut self, item: Switcher) -> Result<(), AppSwitcherError> {
        if item.name.trim().is_empty() {
            return Err(AppSwitcherError::EmptySwitcherName);
        }
        let id = item.id.clone();
        if self.switchers.contains_key(&id) {
            return Err(AppSwitcherError::DuplicateSwitcher(id.clone()));
        }
        self.switchers.insert(id, item);
        Ok(())
    }

    pub fn switcher_count(&self) -> usize {
        self.switchers.len()
    }

    pub fn get_switcher(&self, id: &str) -> Option<&Switcher> {
        self.switchers.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), AppSwitcherError> {
        if event.subject.trim().is_empty() {
            return Err(AppSwitcherError::EmptyEventSubject);
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

    fn sample_switcher(id: &str, name: &str) -> Switcher {
        Switcher {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = AppSwitcher::new("AppSwitcher");
        assert_eq!(m.name, "AppSwitcher");
        assert_eq!(m.switcher_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_switcher() {
        let mut m = AppSwitcher::new("AppSwitcher");
        m.add_switcher(sample_switcher("a1", "Alpha")).unwrap();
        assert_eq!(m.switcher_count(), 1);
        assert!(m.get_switcher("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = AppSwitcher::new("AppSwitcher");
        assert!(m.add_switcher(sample_switcher("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = AppSwitcher::new("AppSwitcher");
        m.add_switcher(sample_switcher("x", "One")).unwrap();
        assert!(m.add_switcher(sample_switcher("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = AppSwitcher::new("AppSwitcher");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
