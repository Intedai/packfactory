use std::{fs::{self, remove_dir_all}, path::Path};
use crate::template::{MANIFEST_FILENAME, parse_manifest};

const PACK_IMG_FILENAME: &str = "pack.png";

pub fn clean() -> anyhow::Result<()> {
    let manifest = parse_manifest(Path::new(MANIFEST_FILENAME))?;

    let pack_name = manifest.name();

    let path: &Path = Path::new(pack_name);
    
    if !path.exists() {
        anyhow::bail!("you didn't build the pack yet, there's nothing to clean")
    }

    fs::remove_dir_all(path)?;
    println!("Removed `{}` directory", pack_name);

    Ok(())
}

fn write_pack_mcmeta(path: &Path, pack_format: u8, description: &str) -> anyhow::Result<()> {
    let description = description.replace("&", "§");

    // For now not using serde_json as this doesn't need validation
    // and i don't want to add a crate just for that
    let data = format!(r#"{{
  "pack": {{
    "pack_format": {pack_format},
    "description": "{description}"
  }}
}}"#);

    fs::write(path, data)?;

    Ok(())
}

pub fn build() -> anyhow::Result<()> {
    let manifest = parse_manifest(Path::new(MANIFEST_FILENAME))?;

    let pack_name = manifest.name();
    let pack_desc = manifest.description();

    let path = Path::new(pack_name);

    if path.exists() { 
        if path.is_dir() {
            remove_dir_all(path)?;
        }
        else {
            anyhow::bail!("`{}` is not a directory, delete it manually before rebuilding", path.display())
        }
    }

    fs::create_dir(path)?;
    
    // Corresponds to 1.8.9, keeping it here for now as I only plan to make this for 1.8.9 (might change in the future)
    let pack_format = 1u8;

    write_pack_mcmeta(&path.join("pack.mcmeta"), pack_format, pack_desc)?;

    fs::copy(Path::new(PACK_IMG_FILENAME), path.join(PACK_IMG_FILENAME))?;
    
    Ok(())
}    