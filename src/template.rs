use std::{fs, path::Path};
use include_dir::{include_dir, Dir};

static TEMPLATE: Dir = include_dir!("$CARGO_MANIFEST_DIR/template");

pub fn create_new_template(path: &Path) -> anyhow::Result<()> {
    
    if path.exists() {
        anyhow::bail!("destination `{}` already exists\n\nRemove the directory and try again", path.display())
    }

    fs::create_dir_all(path)?;
    TEMPLATE.extract(path)?;
    
    let mut manifest = toml_edit::DocumentMut::new();
    
    manifest["pack"] = toml_edit::table();
    manifest["pack"]["name"] = toml_edit::value(path.file_name().unwrap().to_str().unwrap());
    manifest["pack"]["description"] = toml_edit::value("&9&lChange me");

    std::fs::write(path.join("Factory.toml"), manifest.to_string())?;

    Ok(())
}