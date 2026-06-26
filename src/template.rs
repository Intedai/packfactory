use std::{fs, path::Path};
use include_dir::{include_dir, Dir};
use serde::{Serialize, Deserialize};

static TEMPLATE: Dir = include_dir!("$CARGO_MANIFEST_DIR/template");
pub const MANIFEST_FILENAME: &str = "Factory.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pack: Pack
}

#[derive(Debug, Serialize, Deserialize)]
struct Pack {
    name: String,
    description: String
}

impl Manifest {
    pub fn name(&self) -> &str {
        return &self.pack.name;
    }

    pub fn description(&self) -> &str {
        return &self.pack.description;
    }
}

fn write_manifest(path: &Path, name: &str, description: &str) -> anyhow::Result<()>{
    let manifest =  Manifest {
        pack: Pack {
            name: name.to_string(),
            description: description.to_string()
        }
    };

    std::fs::write(path.join(MANIFEST_FILENAME), toml::to_string_pretty(&manifest).unwrap())?; 
    Ok(())
}

pub fn parse_manifest(path: &Path) -> anyhow::Result<Manifest> {
    if !path.exists() {
        anyhow::bail!("could not find `{}`", path.display())
    }
    
    let manifest = fs::read_to_string(path)?;
    let manifest: Manifest = toml::from_str(&manifest)?;

    if manifest.name().contains(std::path::is_separator) {
        anyhow::bail!("name must be a non-nested directory")
    }

    Ok(manifest)
}

pub fn create_new_template(path: &Path) -> anyhow::Result<()> {
    if path.exists() {
        anyhow::bail!("destination `{}` already exists\n\nRemove the directory and try again", path.display())
    }

    fs::create_dir_all(path)?;
    TEMPLATE.extract(path)?;

    let name = path.file_name().unwrap().to_str().unwrap();

    write_manifest(path, name, "A Minecraft texture pack")?;

    println!("Created `{}` template", name);

    Ok(())
}