use catalyze_shared::{report::Report, StaticStorageRef};
use common::ShardStorage;

use super::REPORTS;

pub struct ReportStorage {
    pub name: String,
    pub raw: StaticStorageRef<u64, Report>,
}

impl Default for ReportStorage {
    fn default() -> Self {
        Self {
            name: "report".to_owned(),
            raw: &REPORTS,
        }
    }
}

impl ShardStorage<u64, Report> for ReportStorage {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn storage(&self) -> StaticStorageRef<u64, Report> {
        self.raw
    }
}
