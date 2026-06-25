use clap::Parser;

mod template;
mod pack;
mod args;

use args::{PackFactoryArgs, Commands};

fn main() -> anyhow::Result<()>{

    let args: PackFactoryArgs = PackFactoryArgs::parse();

    match &args.command {
        Commands::New( args ) => template::create_new_template(&args.path)?,
        Commands::Build => todo!(),
        Commands::Clean => pack::clean()?
    }

    Ok(())
}

