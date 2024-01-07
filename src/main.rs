use std::fs::File;
use std::path::PathBuf;

use bpaf::Bpaf;

mod config;
mod data;
mod svg;
mod tree;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
struct Args {
    #[bpaf(fallback("config.toml".into()))]
    config: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = args().run();

    let config = std::fs::read_to_string(args.config)?;
    let config: config::Config = toml::from_str(&config)?;

    for tree in config.tree {
        let svg = format!("{}.svg", tree.name);
        println!("--> {svg}");

        let data = data::Tree::new(&tree.location.read()?)?;
        let tree = tree::build(&data);

        let dest_path = config.out.join(&svg);
        let mut output = File::create(dest_path)?;
        svg::render(&tree, &mut output)?;
    }

    Ok(())
}
