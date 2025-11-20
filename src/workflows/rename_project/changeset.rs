use std::path::Path;

use crate::changes::{Change, RenameFile, SetIniEntry};
use crate::presentation::log;

use super::Context;

/// Generate a changeset to rename an Unreal Engine project.
pub fn generate_changeset(context: &Context) -> Vec<Change> {
    let Context {
        project_root,
        project_name: old_name,
        new_name,
    } = context;

    log::verbose("Generating changeset");
    log::verbose_with_category("changeset", "Adding GameName to DefaultEngine.ini");
    let change1 = add_game_name_to_engine_config(project_root, new_name);
    log::verbose_with_category("changeset", "Adding ProjectName to DefaultGame.ini");
    let change2 = add_project_name_to_game_config(project_root, new_name);
    log::verbose_with_category("changeset", format!("Renaming project descriptor: {}.uproject -> {}.uproject", old_name, new_name));
    let change3 = rename_project_descriptor(project_root, old_name, new_name);
    log::verbose_with_category("changeset", format!("Renaming project root directory to {}", new_name));
    let change4 = rename_project_root(project_root, new_name);
    log::verbose("Changeset generation completed");

    vec![change1, change2, change3, change4]
}

fn rename_project_descriptor(project_root: &Path, old_name: &str, new_name: &str) -> Change {
    Change::RenameFile(RenameFile::new(
        project_root.join(old_name).with_extension("uproject"),
        project_root.join(new_name).with_extension("uproject"),
    ))
}

fn add_game_name_to_engine_config(project_root: &Path, new_name: &str) -> Change {
    Change::SetIniEntry(SetIniEntry::new(
        project_root.join("Config/DefaultEngine.ini"),
        "URL",
        "GameName",
        new_name,
    ))
}

fn add_project_name_to_game_config(project_root: &Path, new_name: &str) -> Change {
    Change::SetIniEntry(SetIniEntry::new(
        project_root.join("Config/DefaultGame.ini"),
        "/Script/EngineSettings.GeneralProjectSettings",
        "ProjectName",
        new_name,
    ))
}

fn rename_project_root(project_root: &Path, new_name: &str) -> Change {
    Change::RenameFile(RenameFile::new(
        &project_root,
        project_root.with_file_name(new_name),
    ))
}
