use catalyze_shared::CanisterResult;

use super::CellStorage;

pub trait IDIter: CellStorage<u64> {
    fn next(&self) -> CanisterResult<u64> {
        if self.is_empty() {
            return self.set(1);
        }

        let current = self.get()?;
        self.set(current + 1)?;

        Ok(current)
    }
}
