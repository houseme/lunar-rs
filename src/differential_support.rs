//! Differential-testing support utilities.
//!
//! These helpers define the stable snapshot protocol used by
//! `tests/differential.rs` and the sample reference driver binary.
//!
//! The protocol is defined **once**, as the [`FIELDS`] table: each entry gives a
//! key, a comparison [`Scope`], and a `derive` closure that reads precomputed
//! values from [`Ctx`]. [`solar_snapshot`] and [`solar_snapshot_keys`] are both
//! derived from that single table, so the key list, the emitted values, and the
//! test expectations can never drift apart.

use std::fmt::Write as _;
use std::io;
use std::sync::Arc;

use crate::{Holiday, Lunar, LunarMonth, LunarYear, NineStar, Solar, SolarFestival, LunarFestival};

/// Snapshot protocol version for differential-testing tooling.
pub const PROTOCOL_VERSION: &str = "8";

/// Comparison scope for a snapshot field.
///
/// `LocalOnly` fields are local lunar-rs extensions with no `tyme4rs` analogue
/// and are skipped when the differential harness runs against the `tyme4rs`
/// reference flavor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    Both,
    LocalOnly,
}

/// A snapshot value that avoids heap allocation for the common static-string case.
#[derive(Debug)]
pub enum Val {
    /// Empty value (renders as `key=`). The most common case for absent
    /// festivals/holidays, and allocation-free.
    Empty,
    /// A static string — no allocation.
    Str(&'static str),
    /// An owned string, for formatted/numeric values.
    Owned(String),
}

impl Val {
    /// The value text, for comparison against the reference output.
    pub fn as_str(&self) -> &str {
        match self {
            Val::Empty => "",
            Val::Str(s) => s,
            Val::Owned(s) => s,
        }
    }
}

/// One emitted snapshot entry.
#[derive(Debug)]
pub struct SnapshotEntry {
    pub key: &'static str,
    pub scope: Scope,
    pub value: Val,
}

/// Precomputed reference objects for a single `Solar` input.
///
/// The expensive lookups (`get_festival`, `get_legal_holiday`, `get_lunar_month`,
/// `get_lunar_hour`, `LunarYear::from_year`) happen exactly once here; every field
/// in [`FIELDS`] reads from this struct instead of re-querying.
struct Ctx {
    solar: Solar,
    lunar: Lunar,
    lunar_year: Arc<LunarYear>,
    lunar_month: Option<LunarMonth>,
    lunar_hour: crate::LunarHour<'static>,
    solar_festival: Option<SolarFestival>,
    lunar_festival: Option<LunarFestival>,
    legal_holiday: Option<Holiday>,
}

impl Ctx {
    fn new(solar: Solar) -> Ctx {
        let lunar = solar.lunar();
        let lunar_year = LunarYear::from_year(lunar.get_year());
        let lunar_month = lunar.get_lunar_month();
        let lunar_hour = solar.get_lunar_hour();
        let solar_festival = solar.get_festival();
        let lunar_festival = lunar.get_festival();
        let legal_holiday = solar.get_legal_holiday();
        Ctx {
            solar,
            lunar,
            lunar_year,
            lunar_month,
            lunar_hour,
            solar_festival,
            lunar_festival,
            legal_holiday,
        }
    }
}

/// One field's worth of snapshot logic.
struct Field {
    key: &'static str,
    scope: Scope,
    derive: fn(&Ctx) -> Val,
}

/// Helper: render a `NineStar` as the compact `number+color+element` form
/// (e.g. `五黄土`) used by the snapshot protocol.
fn nine_star_str(star: NineStar) -> String {
    let mut buf = String::with_capacity(9);
    let _ = write!(buf, "{}{}{}", star.number(), star.color(), star.wu_xing());
    buf
}

/// The single source of truth for the solar snapshot protocol.
///
/// Adding, removing, or reordering a field only requires touching this table;
/// [`solar_snapshot`], [`solar_snapshot_keys`], and the differential test all
/// follow automatically.
static FIELDS: &[Field] = &[
    Field { key: "protocol_version", scope: Scope::Both, derive: |_| Val::Str(PROTOCOL_VERSION) },
    Field { key: "calendar", scope: Scope::Both, derive: |_| Val::Str("solar") },
    Field { key: "solar", scope: Scope::Both, derive: |c| Val::Owned(c.solar.to_ymd_hms()) },
    Field { key: "solar_full", scope: Scope::LocalOnly, derive: |c| Val::Owned(c.solar.to_full_string()) },
    Field { key: "lunar", scope: Scope::Both, derive: |c| Val::Owned(c.lunar.to_string()) },
    Field { key: "lunar_full", scope: Scope::LocalOnly, derive: |c| Val::Owned(c.lunar.to_full_string()) },
    Field {
        key: "solar_festival",
        scope: Scope::Both,
        derive: |c| c.solar_festival.as_ref().map_or(Val::Empty, |f| Val::Owned(f.get_name().to_string())),
    },
    Field {
        key: "solar_festival_index",
        scope: Scope::Both,
        derive: |c| c.solar_festival.as_ref().map_or(Val::Empty, |f| Val::Owned(f.get_index().to_string())),
    },
    Field {
        key: "lunar_festival",
        scope: Scope::Both,
        derive: |c| c.lunar_festival.as_ref().map_or(Val::Empty, |f| Val::Owned(f.get_name().to_string())),
    },
    Field {
        key: "lunar_festival_index",
        scope: Scope::Both,
        derive: |c| c.lunar_festival.as_ref().map_or(Val::Empty, |f| Val::Owned(f.get_index().to_string())),
    },
    Field { key: "jieqi", scope: Scope::Both, derive: |c| Val::Str(c.lunar.jie_qi()) },
    Field { key: "week_name", scope: Scope::Both, derive: |c| Val::Str(c.solar.get_week().name()) },
    Field { key: "week_index", scope: Scope::Both, derive: |c| Val::Owned(c.solar.get_week().index().to_string()) },
    Field { key: "constellation", scope: Scope::Both, derive: |c| Val::Str(c.solar.get_constellation().name()) },
    Field {
        key: "legal_holiday",
        scope: Scope::Both,
        derive: |c| c.legal_holiday.as_ref().map_or(Val::Empty, |h| Val::Owned(h.get_name().to_string())),
    },
    Field {
        key: "legal_holiday_work",
        scope: Scope::Both,
        derive: |c| c.legal_holiday.as_ref().map_or(Val::Empty, |h| Val::Owned(h.is_work().to_string())),
    },
    Field {
        key: "solar_nine_star",
        scope: Scope::LocalOnly,
        derive: |c| Val::Owned(nine_star_str(c.solar.get_nine_star())),
    },
    Field {
        key: "phenology_day",
        scope: Scope::LocalOnly,
        derive: |c| c.solar.get_phenology_day().map_or(Val::Empty, |d| Val::Owned(d.to_string())),
    },
    Field {
        key: "phase_day",
        scope: Scope::LocalOnly,
        derive: |c| Val::Owned(c.solar.get_phase_day().to_string()),
    },
    Field {
        key: "nine_day",
        scope: Scope::LocalOnly,
        derive: |c| c.solar.get_nine_day().map_or(Val::Empty, |d| Val::Owned(d.to_string())),
    },
    Field {
        key: "hide_heaven_stem_day",
        scope: Scope::LocalOnly,
        derive: |c| c.solar.get_hide_heaven_stem_day().map_or(Val::Empty, |d| Val::Owned(d.to_string())),
    },
    Field {
        key: "dog_day",
        scope: Scope::LocalOnly,
        derive: |c| c.solar.get_dog_day().map_or(Val::Empty, |d| Val::Owned(d.to_string())),
    },
    Field {
        key: "plum_rain_day",
        scope: Scope::LocalOnly,
        derive: |c| c.solar.get_plum_rain_day().map_or(Val::Empty, |d| Val::Owned(d.to_string())),
    },
    Field { key: "year_ganzhi", scope: Scope::Both, derive: |c| Val::Owned(c.lunar.year_in_gan_zhi()) },
    Field { key: "month_ganzhi", scope: Scope::Both, derive: |c| Val::Owned(c.lunar.month_in_gan_zhi()) },
    Field { key: "day_ganzhi", scope: Scope::Both, derive: |c| Val::Owned(c.lunar.day_in_gan_zhi()) },
    Field { key: "time_ganzhi", scope: Scope::Both, derive: |c| Val::Owned(c.lunar.time_in_gan_zhi()) },
    Field {
        key: "lunar_year_month_count",
        scope: Scope::Both,
        derive: |c| Val::Owned(c.lunar_year.get_month_count().to_string()),
    },
    Field {
        key: "lunar_year_leap_month",
        scope: Scope::Both,
        derive: |c| Val::Owned(c.lunar_year.get_leap_month().to_string()),
    },
    Field {
        key: "lunar_year_sixty_cycle",
        scope: Scope::Both,
        derive: |c| Val::Str(c.lunar_year.get_sixty_cycle().name()),
    },
    Field {
        key: "lunar_year_jupiter_direction",
        scope: Scope::Both,
        derive: |c| Val::Str(c.lunar_year.get_jupiter_direction().name()),
    },
    Field {
        key: "lunar_year_twenty",
        scope: Scope::Both,
        derive: |c| Val::Str(c.lunar_year.get_twenty().name()),
    },
    Field {
        key: "lunar_year_nine_star",
        scope: Scope::Both,
        derive: |c| Val::Owned(nine_star_str(c.lunar_year.get_nine_star())),
    },
    Field {
        key: "lunar_month",
        scope: Scope::Both,
        derive: |c| c.lunar_month.as_ref().map_or(Val::Empty, |m| Val::Owned(m.get_month().to_string())),
    },
    Field {
        key: "lunar_month_with_leap",
        scope: Scope::Both,
        derive: |c| c.lunar_month.as_ref().map_or(Val::Empty, |m| Val::Owned(m.get_month_with_leap().to_string())),
    },
    Field {
        key: "lunar_month_day_count",
        scope: Scope::Both,
        derive: |c| c.lunar_month.as_ref().map_or(Val::Empty, |m| Val::Owned(m.get_day_count().to_string())),
    },
    Field {
        key: "lunar_month_index_in_year",
        scope: Scope::Both,
        derive: |c| c.lunar_month.as_ref().map_or(Val::Empty, |m| Val::Owned(m.get_index_in_year().to_string())),
    },
    Field {
        key: "lunar_month_minor_ren",
        scope: Scope::Both,
        derive: |c| c.lunar_month.as_ref().map_or(Val::Empty, |m| Val::Str(m.get_minor_ren().name())),
    },
    Field {
        key: "lunar_month_nine_star",
        scope: Scope::Both,
        derive: |c| c.lunar_month.as_ref().map_or(Val::Empty, |m| Val::Owned(nine_star_str(m.get_nine_star()))),
    },
    Field {
        key: "lunar_month_sixty_cycle",
        scope: Scope::Both,
        derive: |c| c.lunar_month.as_ref().map_or(Val::Empty, |m| Val::Str(m.get_sixty_cycle().name())),
    },
    Field {
        key: "lunar_month_jupiter_direction",
        scope: Scope::Both,
        derive: |c| c.lunar_month.as_ref().map_or(Val::Empty, |m| Val::Str(m.get_jupiter_direction().name())),
    },
    Field {
        key: "lunar_month_fetus",
        scope: Scope::Both,
        derive: |c| c.lunar_month.as_ref().and_then(|m| m.get_fetus()).map_or(Val::Empty, |f| Val::Owned(f.name().to_string())),
    },
    Field {
        key: "lunar_hour_name",
        scope: Scope::Both,
        derive: |c| Val::Owned(c.lunar_hour.get_name()),
    },
    Field {
        key: "lunar_hour_index_in_day",
        scope: Scope::Both,
        derive: |c| Val::Owned(c.lunar_hour.get_index_in_day().to_string()),
    },
    Field {
        key: "lunar_hour_minor_ren",
        scope: Scope::Both,
        derive: |c| Val::Str(c.lunar_hour.get_minor_ren().name()),
    },
    Field {
        key: "lunar_hour_twelve_star",
        scope: Scope::Both,
        derive: |c| Val::Str(c.lunar_hour.get_twelve_star().name()),
    },
    Field {
        key: "lunar_hour_nine_star",
        scope: Scope::Both,
        derive: |c| Val::Owned(nine_star_str(c.lunar_hour.get_nine_star())),
    },
    Field {
        key: "lunar_six_star",
        scope: Scope::LocalOnly,
        derive: |c| Val::Str(c.lunar.get_six_star().name()),
    },
    Field {
        key: "lunar_minor_ren",
        scope: Scope::LocalOnly,
        derive: |c| Val::Str(c.lunar.get_minor_ren().name()),
    },
    Field {
        key: "lunar_twelve_star",
        scope: Scope::LocalOnly,
        derive: |c| Val::Str(c.lunar.get_twelve_star().name()),
    },
    Field {
        key: "lunar_twenty_eight_star",
        scope: Scope::LocalOnly,
        derive: |c| Val::Str(c.lunar.get_twenty_eight_star().name()),
    },
    Field {
        key: "lunar_nine_star",
        scope: Scope::LocalOnly,
        derive: |c| Val::Owned(nine_star_str(c.lunar.get_nine_star())),
    },
];

/// Render a stable ordered key-value snapshot for a solar datetime input.
pub fn solar_snapshot(solar: Solar) -> Vec<SnapshotEntry> {
    let ctx = Ctx::new(solar);
    FIELDS
        .iter()
        .map(|field| SnapshotEntry {
            key: field.key,
            scope: field.scope,
            value: (field.derive)(&ctx),
        })
        .collect()
}

/// The stable, ordered key names emitted by the snapshot, derived from [`FIELDS`].
pub fn solar_snapshot_keys() -> impl Iterator<Item = &'static str> {
    FIELDS.iter().map(|field| field.key)
}

/// Write a snapshot's entries as newline-delimited `key=value` pairs to `writer`.
///
/// Static and empty values incur no per-entry heap allocation.
pub fn write_snapshot<W: io::Write>(snapshot: &[SnapshotEntry], writer: &mut W) -> io::Result<()> {
    for entry in snapshot {
        writeln!(writer, "{}={}", entry.key, entry.value.as_str())?;
    }
    Ok(())
}
