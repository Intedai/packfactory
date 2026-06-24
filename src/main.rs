use include_dir::{include_dir, Dir};
use std::{fs, path::Path};
use clap::Parser;
mod args;

use args::{PackFactoryArgs, Commands};

static TEMPLATE: Dir = include_dir!("$CARGO_MANIFEST_DIR/template");

// TODO: Take template as an arguement and move this fn to something like pf_new.rs
fn create_new_template(path: &Path) ->  anyhow::Result<()> {
    
    if path.exists() {
        anyhow::bail!("destination `{}` already exists\n\nRemove the directory and try again", path.display())
    }

    fs::create_dir_all(path)?;
    TEMPLATE.extract(path)?;

    Ok(())
}

fn main() ->  anyhow::Result<()>{

    let args: PackFactoryArgs = PackFactoryArgs::parse();

    match &args.command {
        Commands::New( args ) => create_new_template(&args.path)?,
        Commands::Build => todo!(),
        Commands::Clean => todo!()
    }

    Ok(())
}

