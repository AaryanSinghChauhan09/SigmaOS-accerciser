// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-accerciser - Accessibility Explorer
//! Inspired by GNOME accerciser
//! Provides accessibility exploration following SOLID principles

#![no_std]

/// Accessibility explorer
pub struct Accerciser {
    name: &'static str,
    active: bool,
}

impl Accerciser {
    pub fn new() -> Self {
        Self {
            name: "SigmaOS Accerciser",
            active: false,
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn active(&self) -> bool {
        self.active
    }
}

impl Default for Accerciser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accerciser_creation() {
        let acc = Accerciser::new();
        assert_eq!(acc.name(), "SigmaOS Accerciser");
    }

    #[test]
    fn test_activate() {
        let mut acc = Accerciser::new();
        acc.activate();
        assert!(acc.active());
    }
}
