use std::{
    fmt::Display,
    io,
    path::{Path, PathBuf},
};

use ini::{Ini, ParseOption};
use regex::Regex;
use sha2::{Digest, Sha256};

use crate::presentation::log;
use super::{rename_file::RenameFile, AppendIniEntry, ReplaceInFile, SetIniEntry};

#[derive(Debug, PartialEq)]
pub enum Change {
    RenameFile(RenameFile),
    ReplaceInFile(ReplaceInFile),
    SetIniEntry(SetIniEntry),
    AppendIniEntry(AppendIniEntry),
}

impl Change {
    pub fn apply(&self, backup_dir: &Path) -> io::Result<Revert> {
        match &*self {
            Change::RenameFile(params) => Change::rename_file(&params),
            Change::ReplaceInFile(params) => Change::replace_in_file(params, backup_dir),
            Change::SetIniEntry(params) => Change::set_ini_entry(params, backup_dir),
            Change::AppendIniEntry(params) => Change::append_ini_entry(params, backup_dir),
        }
    }

    fn rename_file(params: &RenameFile) -> io::Result<Revert> {
        let from = params.from.clone();
        let to = params.to.clone();
        log::verbose_with_category("rename_file", format!("Renaming {:?} -> {:?}", from, to));
        std::fs::rename(&from, &to)?;
        log::verbose_with_category("rename_file", "File renamed successfully");

        Ok(Box::new(move || std::fs::rename(&to, &from)))
    }

    fn replace_in_file(params: &ReplaceInFile, backup_dir: &Path) -> io::Result<Revert> {
        log::verbose_with_category("replace_in_file", format!("Processing file: {:?}", params.path));
        log::verbose_with_category("replace_in_file", format!("Pattern: {}", params.from));
        log::verbose_with_category("replace_in_file", format!("Replacement: {}", params.to));
        let backup = Change::backup_file(&params.path, backup_dir)?;
        let target = params.path.clone();
        log::verbose_with_category("replace_in_file", "Reading file content");
        let content = std::fs::read_to_string(&target)?;
        log::verbose_with_category("replace_in_file", format!("File size: {} bytes", content.len()));
        let regex = Regex::new(&params.from).expect("regex should be valid");
        let content_after_replace = regex.replace_all(&content, params.to.as_str()).to_string();
        let num_replacements = content.len().saturating_sub(content_after_replace.len()).abs_diff(content_after_replace.len().saturating_sub(content.len()));
        log::verbose_with_category("replace_in_file", format!("Replacements made, size changed by {} bytes", num_replacements));
        log::verbose_with_category("replace_in_file", "Writing modified content");
        std::fs::write(&target, &content_after_replace)?;
        log::verbose_with_category("replace_in_file", "File replacement completed");

        Ok(Box::new(move || {
            std::fs::copy(&backup, &target).map(|_| ())
        }))
    }

    fn set_ini_entry(params: &SetIniEntry, backup_dir: &Path) -> io::Result<Revert> {
        let SetIniEntry {
            section,
            key,
            value,
            path,
        } = params;

        log::verbose_with_category("set_ini_entry", format!("Processing INI file: {:?}", path));
        log::verbose_with_category("set_ini_entry", format!("Section: [{}], Key: {}, Value: {}", section, key, value));
        let backup = Change::backup_file(path, backup_dir)?;
        let target = path.clone();

        let read_opts = ParseOption {
            enabled_escape: false,
            enabled_quote: false,
        };
        log::verbose_with_category("set_ini_entry", "Loading INI file");
        let mut ini = match Ini::load_from_file_opt(&target, read_opts) {
            Ok(ini) => ini,
            Err(err) => match err {
                ini::ini::Error::Io(io) => return Err(io),
                ini::ini::Error::Parse(p) => return Err(io::Error::new(io::ErrorKind::Other, p)),
            },
        };
        log::verbose_with_category("set_ini_entry", "Setting INI entry");
        ini.with_section(Some(section)).set(key, value);
        log::verbose_with_category("set_ini_entry", "Writing INI file");
        ini.write_to_file(&target)?;
        log::verbose_with_category("set_ini_entry", "INI entry set successfully");

        Ok(Box::new(move || {
            std::fs::copy(&backup, &target).map(|_| ())
        }))
    }

    fn append_ini_entry(params: &AppendIniEntry, backup_dir: &Path) -> io::Result<Revert> {
        let AppendIniEntry {
            section,
            key,
            value,
            path,
        } = params;

        log::verbose_with_category("append_ini_entry", format!("Processing INI file: {:?}", path));
        log::verbose_with_category("append_ini_entry", format!("Section: [{}], Appending Key: {}, Value: {}", section, key, value));
        let backup = Change::backup_file(path, backup_dir)?;
        let target = path.clone();

        let read_opts = ParseOption {
            enabled_escape: false,
            enabled_quote: false,
        };
        log::verbose_with_category("append_ini_entry", "Loading INI file");
        let mut ini = match Ini::load_from_file_opt(&target, read_opts) {
            Ok(ini) => ini,
            Err(err) => match err {
                ini::ini::Error::Io(io) => return Err(io),
                ini::ini::Error::Parse(p) => return Err(io::Error::new(io::ErrorKind::Other, p)),
            },
        };
        log::verbose_with_category("append_ini_entry", "Appending INI entry");
        ini.with_section(Some(section)).set("dummy", "dummy"); // create if does not exist
        ini.section_mut(Some(section)).unwrap().append(key, value);
        ini.with_section(Some(section)).delete(&"dummy");
        log::verbose_with_category("append_ini_entry", "Writing INI file");
        ini.write_to_file(&params.path)?;
        log::verbose_with_category("append_ini_entry", "INI entry appended successfully");

        Ok(Box::new(move || {
            std::fs::copy(&backup, &target).map(|_| ())
        }))
    }

    fn backup_file(file: &Path, backup_dir: &Path) -> io::Result<PathBuf> {
        log::verbose_with_category("backup", format!("Creating backup of {:?}", file));
        let content = std::fs::read_to_string(file)?;
        let hash = Sha256::digest(&content);
        let path = backup_dir.join(format!("{:x}", hash));
        log::verbose_with_category("backup", format!("Backup file hash: {:x}", hash));
        log::verbose_with_category("backup", format!("Backup path: {:?}", path));
        std::fs::write(&path, &content)?;
        log::verbose_with_category("backup", "Backup created successfully");
        Ok(path)
    }
}

impl Display for Change {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Change::RenameFile(params) => write!(f, "{}", &params),
            Change::ReplaceInFile(params) => write!(f, "{}", &params),
            Change::SetIniEntry(params) => write!(f, "{}", &params),
            Change::AppendIniEntry(params) => write!(f, "{}", &params),
        }
    }
}

pub type Revert = Box<dyn Fn() -> io::Result<()>>;
