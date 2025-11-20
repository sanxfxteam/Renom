use std::path::Path;

use crate::{
    changes::{Change, Revert},
    presentation::log,
};

pub struct Engine {
    history: Vec<(Change, Revert)>,
}

impl Engine {
    pub fn new() -> Self {
        Self { history: vec![] }
    }

    /// Execute a series of changes in sequential order and stores the
    /// applied changes in history with appropriate revert actions.
    /// Upon error, it will halt execution and return the error.
    pub fn execute(
        &mut self,
        changeset: Vec<Change>,
        backup_dir: impl AsRef<Path>,
    ) -> Result<(), String> {
        log::verbose(format!("Starting execution of {} changes", changeset.len()));
        log::verbose_with_category("engine", format!("Backup directory: {:?}", backup_dir.as_ref()));
        for (idx, change) in changeset.into_iter().enumerate() {
            log::verbose_with_category("engine", format!("Executing change {}: {}", idx + 1, &change));
            log::step("apply", &change);
            self.execute_single(change, backup_dir.as_ref())?;
            log::verbose_with_category("engine", format!("Change {} completed successfully", idx + 1));
        }
        log::verbose("All changes executed successfully");
        Ok(())
    }

    fn execute_single(&mut self, change: Change, backup_dir: &Path) -> Result<(), String> {
        match change.apply(backup_dir) {
            Ok(revert) => {
                self.history.push((change, revert));
                Ok(())
            }
            Err(err) => Err(err.to_string()),
        }
    }

    /// Revert entire history of actions.
    /// Upon error, it will halt execution and return the error.
    pub fn revert(&mut self) -> Result<(), String> {
        log::verbose(format!("Starting revert of {} changes", self.history.len()));
        let mut count = 0;
        while let Some((change, revert)) = self.history.pop() {
            count += 1;
            log::verbose_with_category("revert", format!("Reverting change {}: {}", count, &change));
            log::step("revert", &change);
            revert().map_err(|err| err.to_string())?;
            log::verbose_with_category("revert", format!("Change {} reverted successfully", count));
        }
        log::verbose("All changes reverted successfully");
        Ok(())
    }
}
