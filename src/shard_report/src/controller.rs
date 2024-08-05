use catalyze_shared::report::{Report, ReportFilter};
use common::ShardController;

use crate::storage::ReportStorage;

pub struct ReportController;

impl ShardController<u64, Report, ReportFilter> for ReportController {
    fn storage(&self) -> impl common::ShardStorage<u64, Report> {
        ReportStorage::default()
    }
}
