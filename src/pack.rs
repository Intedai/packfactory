use std::{fs, path::Path};

pub fn clean() -> anyhow::Result<()> { 

    let pack_name = "GET THE NAME FROM THE TOML OR AS AN ARGUEMENT";
    let path: &Path = Path::new(pack_name);
    
    if !path.exists() {
        anyhow::bail!("You didn't build the texture pack yet, there's nothing to clean")
    }

    fs::remove_dir_all(path)?;
    println!("Removed `{}` directory", pack_name);

    Ok(())
}