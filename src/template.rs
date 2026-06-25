use std::{fs, path::Path};
use include_dir::{include_dir, Dir};

static TEMPLATE: Dir = include_dir!("$CARGO_MANIFEST_DIR/template");

pub fn create_new_template(path: &Path) -> anyhow::Result<()> {
    
    if path.exists() {
        anyhow::bail!("destination `{}` already exists\n\nRemove the directory and try again", path.display())
    }

    fs::create_dir_all(path)?;
    TEMPLATE.extract(path)?;
    
    // TODO: create toml here (maybe in a diff fn)

    Ok(())
}