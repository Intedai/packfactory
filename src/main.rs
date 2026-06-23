use include_dir::{include_dir, Dir};
use std::fs;

static TEMPLATE: Dir = include_dir!("$CARGO_MANIFEST_DIR/template");

fn main() ->  std::io::Result<()>{
    let dir_name: &str = "MyPack";

    fs::create_dir(dir_name)?;
    TEMPLATE.extract(dir_name)?;

    Ok(())
}

