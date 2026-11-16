// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-firmware-installation: firmware installation component model.
//!
//! Real implementation: firmwares, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Firmware {
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
pub enum FirmwareInstallationError {
    EmptyFirmwareName,
    DuplicateFirmware(String),
    EmptyEventSubject,
}

impl fmt::Display for FirmwareInstallationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyFirmwareName => write!(f, "firmware name must not be empty"),
            Self::DuplicateFirmware(id) => write!(f, "duplicate firmware {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for FirmwareInstallationError {}

pub struct FirmwareInstallation {
    pub name: String,
    firmwares: BTreeMap<String, Firmware>,
    events: BTreeMap<String, Event>,
}

impl FirmwareInstallation {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            firmwares: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_firmware(&mut self, item: Firmware) -> Result<(), FirmwareInstallationError> {
        if item.name.trim().is_empty() {
            return Err(FirmwareInstallationError::EmptyFirmwareName);
        }
        let id = item.id.clone();
        if self.firmwares.contains_key(&id) {
            return Err(FirmwareInstallationError::DuplicateFirmware(id.clone()));
        }
        self.firmwares.insert(id, item);
        Ok(())
    }

    pub fn firmware_count(&self) -> usize {
        self.firmwares.len()
    }

    pub fn get_firmware(&self, id: &str) -> Option<&Firmware> {
        self.firmwares.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), FirmwareInstallationError> {
        if event.subject.trim().is_empty() {
            return Err(FirmwareInstallationError::EmptyEventSubject);
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

    fn sample_firmware(id: &str, name: &str) -> Firmware {
        Firmware {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = FirmwareInstallation::new("FirmwareInstallation");
        assert_eq!(m.name, "FirmwareInstallation");
        assert_eq!(m.firmware_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_firmware() {
        let mut m = FirmwareInstallation::new("FirmwareInstallation");
        m.add_firmware(sample_firmware("a1", "Alpha")).unwrap();
        assert_eq!(m.firmware_count(), 1);
        assert!(m.get_firmware("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = FirmwareInstallation::new("FirmwareInstallation");
        assert!(m.add_firmware(sample_firmware("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = FirmwareInstallation::new("FirmwareInstallation");
        m.add_firmware(sample_firmware("x", "One")).unwrap();
        assert!(m.add_firmware(sample_firmware("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = FirmwareInstallation::new("FirmwareInstallation");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
