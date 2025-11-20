mod changeset;
mod interactive;

use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use regex::Regex;

use crate::{engine::Engine, presentation::log, unreal::Target};

use self::{changeset::generate_changeset, interactive::get_params_from_user};

/// Params needed to rename an Unreal Engine target.
pub struct Params {
    /// The root of the project.
    pub project_root: PathBuf,
    /// The specific target to rename.
    pub target: String,
    /// The new name for the target.
    pub new_name: String,
    /// Enable verbose logging.
    pub verbose: bool,
}

/// Context needed to rename an Unreal Engine target.
pub struct Context {
    /// The root of the project.
    pub project_root: PathBuf,
    /// Build targets for the project.
    pub project_targets: Vec<Target>,
    /// The specific target to rename.
    pub target: Target,
    /// The new name for the target.
    pub new_name: String,
}

/// Rename an Unreal Engine target interactively, soliciting input parameters
/// from the user with validation and guided selection.
pub fn rename_target_interactive() -> Result<(), String> {
    let params = get_params_from_user()?;
    rename_target(params)
}

/// Rename an Unreal Engine target.
pub fn rename_target(params: Params) -> Result<(), String> {
    validate_params(&params)?;
    let context = gather_context(&params)?;
    let changeset = generate_changeset(&context);
    let backup_dir = create_backup_dir(&context.project_root)?;
    let mut engine = Engine::new();
    if let Err(e) = engine.execute(changeset, backup_dir) {
        log::error(&e);
        engine.revert()?;
        print_failure_message(&context);
        return Ok(());
    }

    print_success_message(&context);
    Ok(())
}

fn validate_params(params: &Params) -> Result<(), String> {
    log::verbose("Starting parameter validation");
    log::verbose_with_category("validation", "Checking project root is a directory");
    validate_project_root_is_dir(&params.project_root)?;
    log::verbose_with_category("validation", "Checking project root contains .uproject file");
    validate_project_root_contains_project_descriptor(&params.project_root)?;
    log::verbose_with_category("validation", "Checking project root contains Source folder");
    validate_project_root_contains_source_dir(&params.project_root)?;
    log::verbose_with_category("validation", "Detecting project targets");
    let targets = detect_project_targets(&params.project_root)?;
    log::verbose_with_category("validation", format!("Found {} targets", targets.len()));
    log::verbose_with_category("validation", format!("Validating target '{}' exists", params.target));
    validate_target_exists(&params.target, &targets)?;
    log::verbose_with_category("validation", "Validating new name is not empty");
    validate_new_name_is_not_empty(&params.new_name)?;
    log::verbose_with_category("validation", "Validating new name length");
    validate_new_name_is_concise(&params.new_name)?;
    log::verbose_with_category("validation", "Validating new name is unique");
    validate_new_name_is_unique(&params.new_name, &targets)?;
    log::verbose_with_category("validation", "Validating new name is valid identifier");
    validate_new_name_is_valid_identifier(&params.new_name)?;
    log::verbose("Parameter validation completed successfully");
    Ok(())
}

fn validate_project_root_is_dir(project_root: &Path) -> Result<(), String> {
    match project_root.is_dir() {
        true => Ok(()),
        false => Err("project root must be a directory".into()),
    }
}

fn validate_project_root_contains_project_descriptor(project_root: &Path) -> Result<(), String> {
    match fs::read_dir(project_root)
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .filter_map(|entry| entry.path().extension().map(OsStr::to_owned))
        .any(|ext| ext == "uproject")
    {
        true => Ok(()),
        false => Err("project root must contain a project descriptor".into()),
    }
}

fn validate_project_root_contains_source_dir(project_root: &Path) -> Result<(), String> {
    match project_root.join("Source").is_dir() {
        true => Ok(()),
        false => Err("project root must contain a Source folder".into()),
    }
}

fn validate_target_exists(target: &str, targets: &[Target]) -> Result<(), String> {
    match targets.iter().any(|other| other.name == target) {
        true => Ok(()),
        false => Err("target must be part of project".into()),
    }
}

fn validate_new_name_is_not_empty(new_name: &str) -> Result<(), String> {
    match !new_name.trim().is_empty() {
        true => Ok(()),
        false => Err("new name must not be empty".into()),
    }
}

fn validate_new_name_is_concise(new_name: &str) -> Result<(), String> {
    let new_name_max_len = 30;
    match new_name.len() <= new_name_max_len {
        true => Ok(()),
        false => {
            let error_message = format!(
                "new name must not be longer than {} characters",
                new_name_max_len
            );
            Err(error_message)
        }
    }
}

fn validate_new_name_is_unique(new_name: &str, targets: &[Target]) -> Result<(), String> {
    match targets.iter().all(|target| target.name != new_name) {
        true => Ok(()),
        false => {
            let error_message = "new name must not conflict with another target";
            Err(error_message.into())
        }
    }
}

fn validate_new_name_is_valid_identifier(new_name: &str) -> Result<(), String> {
    let identifier_regex = Regex::new("^[_[[:alnum:]]]*$").expect("regex should be valid");
    match identifier_regex.is_match(new_name) {
        true => Ok(()),
        false => {
            let error_message =
                "new name must be comprised of alphanumeric characters and underscores only";
            Err(error_message.into())
        }
    }
}

fn detect_project_targets(project_root: &Path) -> Result<Vec<Target>, String> {
    let source_dir = project_root.join("Source");
    assert!(source_dir.is_dir());
    log::verbose_with_category("detect", format!("Searching for targets in {:?}", source_dir));
    let targets: Vec<Target> = fs::read_dir(&source_dir)
        .map_err(|err| err.to_string())?
        .filter_map(Result::ok)
        .filter_map(|entry| {
            entry
                .path()
                .file_name()
                .and_then(|file_name| file_name.to_str())
                .and_then(|str| str.strip_suffix(".Target.cs"))
                .map(|str| str.to_owned())
        })
        .map(|target_name| Target {
            name: target_name.clone(),
            path: source_dir.join(target_name).with_extension("Target.cs"),
        })
        .collect();
    log::verbose_with_category("detect", format!("Found {} targets", targets.len()));
    Ok(targets)
}

fn gather_context(params: &Params) -> Result<Context, String> {
    log::verbose("Gathering context");
    let project_root = params.project_root.clone();
    log::verbose_with_category("context", format!("Project root: {:?}", project_root));
    log::verbose_with_category("context", "Detecting project targets");
    let project_targets = detect_project_targets(&project_root)?;
    log::verbose_with_category("context", format!("Finding target: {}", params.target));
    let target = project_targets
        .iter()
        .find(|target| target.name == params.target)
        .unwrap()
        .clone();
    log::verbose_with_category("context", format!("Target path: {:?}", target.path));
    log::verbose("Context gathering completed");

    Ok(Context {
        project_root,
        project_targets,
        target,
        new_name: params.new_name.clone(),
    })
}

fn create_backup_dir(project_root: &Path) -> Result<PathBuf, String> {
    let backup_dir = project_root.join(".renom/backup");
    log::verbose_with_category("backup", format!("Creating backup directory: {:?}", backup_dir));
    fs::create_dir_all(&backup_dir).map_err(|err| err.to_string())?;
    log::verbose_with_category("backup", "Backup directory created successfully");
    Ok(backup_dir)
}

fn print_success_message(context: &Context) {
    log::success(format!(
        "Successfully renamed target {} to {}.",
        context.target.name, context.new_name
    ));
}

fn print_failure_message(context: &Context) {
    log::error(format!(
        "Failed to rename target {} to {}.",
        context.target.name, context.new_name
    ));
}
