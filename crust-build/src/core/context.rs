use crate::{
    core::{logs, target::Target, variant::Variant},
    log_tag
};
use std::path::PathBuf;

pub struct Context {
    pub assets_dir: PathBuf,
    pub rust_build_dir: PathBuf,
    pub source_dir: PathBuf,
    pub target_home_dir: PathBuf,
    pub working_dir: PathBuf,
    pub variant: Variant,
}

impl Context {
    pub fn new(root_dir: PathBuf, target: Target, variant: Variant) -> Self {
        let target_home_dir = root_dir.join(target.id());
        let working_dir = target_home_dir.join(".rust-build");
        let rust_build_dir = working_dir.
    }
}