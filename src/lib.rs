//! SigmaOS-accerciser: accessibility tester model.
//!
//! Real implementation: UI inspectors, AT-SPI probes, validation.
//! Replaces the 104-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessibleWidget {
    pub id: String,
    pub role: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccerciserError {
    EmptyName,
    DuplicateWidget(String),
}

impl fmt::Display for AccerciserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "widget name must not be empty"),
            Self::DuplicateWidget(id) => write!(f, "duplicate widget {:?}", id),
        }
    }
}

impl std::error::Error for AccerciserError {}

pub struct AccessibilityInspector {
    pub name: String,
    widgets: BTreeMap<String, AccessibleWidget>,
    probes: BTreeMap<String, String>,
}

impl AccessibilityInspector {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), widgets: BTreeMap::new(), probes: BTreeMap::new() }
    }

    pub fn add_widget(&mut self, w: AccessibleWidget) -> Result<(), AccerciserError> {
        if w.name.trim().is_empty() { return Err(AccerciserError::EmptyName); }
        let id = w.id.clone();
        if self.widgets.contains_key(&id) {
            return Err(AccerciserError::DuplicateWidget(id.clone()));
        }
        self.widgets.insert(id, w);
        Ok(())
    }

    pub fn widget_count(&self) -> usize { self.widgets.len() }

    pub fn get_widget(&self, id: &str) -> Option<&AccessibleWidget> { self.widgets.get(id) }

    pub fn add_probe(&mut self, target: String, result: String) {
        self.probes.insert(target, result);
    }

    pub fn get_probe(&self, target: &str) -> Option<&String> { self.probes.get(target) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let i = AccessibilityInspector::new("My Inspector");
        assert_eq!(i.name, "My Inspector");
        assert_eq!(i.widget_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut i = AccessibilityInspector::new("My Inspector");
        let w = AccessibleWidget { id: "btn1".into(), role: "button".into(), name: "Submit".into(), description: "Submit form".into() };
        i.add_widget(w).unwrap();
        assert_eq!(i.widget_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut i = AccessibilityInspector::new("My Inspector");
        let w = AccessibleWidget { id: "x".into(), role: "a".into(), name: "".into(), description: "b".into() };
        assert!(i.add_widget(w).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut i = AccessibilityInspector::new("My Inspector");
        let w = AccessibleWidget { id: "x".into(), role: "a".into(), name: "X".into(), description: "b".into() };
        i.add_widget(w).unwrap();
        let w2 = AccessibleWidget { id: "x".into(), role: "a".into(), name: "X".into(), description: "b".into() };
        assert!(i.add_widget(w2).is_err());
    }
}
