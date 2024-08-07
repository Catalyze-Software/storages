use catalyze_shared::report::{Report, ReportEntry, ReportFilter, ReportSort};

#[allow(dead_code)]
pub const DATA_KIND: &str = "report";

pub type Key = u64;
pub type Value = Report;
pub type Entry = ReportEntry;
pub type EntryFilter = ReportFilter;
pub type EntrySort = ReportSort;
