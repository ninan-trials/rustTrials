use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum MaterialType {
    git,
    svn,
    hg,
    p4,
    tfs,
    dependency,
    package,
    plugin,
}

#[derive(Serialize, Deserialize)]
pub struct GitMaterialAttribute {
    pub name: Option<String>,
    pub url: String,
    pub filter: Option<String>,
    pub destination: Option<String>,
    pub invert_filter: bool,
    pub auto_update: bool,
    pub branch: String,
    pub submodule_folder: Option<String>,
    pub shallow_clone: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Material {
    #[serde(alias = "type")]
    #[serde(rename = "type")]
    pub material_type: MaterialType,
    pub attributes: GitMaterialAttribute,
}