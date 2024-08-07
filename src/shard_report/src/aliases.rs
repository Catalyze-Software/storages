use catalyze_shared::report::{Report, ReportEntry, ReportFilter};

pub const DATA_KIND: &str = "report";

pub type Key = u64;
pub type Value = Report;
pub type Entry = ReportEntry;
pub type EntryFilter = ReportFilter;
