use std::{fs, path::Path};
use crate::template::{parse_manifest, MANIFEST_FILENAME};

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