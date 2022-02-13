use std::fs;
use std::path::Path;

// use anyhow::anyhow as err;
use cargo::core::Dependency;
use cargo::util::toml::TomlManifest;
use serde::Serialize;
use toml_edit;

use crate::errors::CargoGetResult;

#[derive(Debug)]
pub struct CargoToml<'p> {
    root: &'p Path,
    content: TomlManifest,
}

impl<'p> CargoToml<'p> {
    pub fn from_filepath(root: &'p Path) -> CargoGetResult<Self> {
        let toml_str = fs::read_to_string(root.join("Cargo.toml"))?;
        let content: TomlManifest = toml_edit::easy::from_str(&toml_str)?;
        Ok(Self { root, content })
    }
}

#[derive(Serialize)]
pub struct CargoGetToml {
    #[serde(flatten)]
    content: Option<TomlManifest>,
    // CargoGet.toml specific values
}
