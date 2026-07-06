//! Unified calendar events.
//!
//! This is the minimal Phase 3 event model that aggregates festivals, holidays
//! and JieQi into a single typed read-only API.

use crate::Solar;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EventKind {
    SolarFestival,
    SolarOtherFestival,
    LunarFestival,
    LunarOtherFestival,
    Holiday,
    JieQi,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Event {
    kind: EventKind,
    name: String,
    solar: Solar,
    detail: Option<String>,
}

impl Event {
    pub fn new(kind: EventKind, name: impl Into<String>, solar: Solar) -> Self {
        Self { kind, name: name.into(), solar, detail: None }
    }

    pub fn with_detail(kind: EventKind, name: impl Into<String>, solar: Solar, detail: impl Into<String>) -> Self {
        Self { kind, name: name.into(), solar, detail: Some(detail.into()) }
    }

    pub const fn kind(&self) -> &EventKind {
        &self.kind
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn solar(&self) -> Solar {
        self.solar
    }

    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }
}
