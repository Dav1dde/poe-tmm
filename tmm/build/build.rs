use std::env;
use std::fs::File;
use std::path::Path;

mod data;
mod rust;
mod svg;
mod tree;

fn get_trees() -> impl Iterator<Item = String> {
    env::vars().filter_map(|(name, _)| {
        name.strip_prefix("CARGO_FEATURE_TREE_")
            .map(|x| x.replace('_', "."))
    })
}

fn main() -> anyhow::Result<()> {
    for version in get_trees() {
        let svg = format!("{version}.svg");

        let data_path = Path::new("..")
            .join("data")
            .join(format!("tree-{version}"))
            .join("data.json");
        let data = data::Tree::read(data_path)?;
        let tree = tree::build(&data);

        let dest_path = Path::new("templates").join(&svg);
        let mut output = File::create(dest_path)?;
        svg::render(&tree, &mut output)?;

        let dest_path = Path::new(&env::var_os("OUT_DIR").unwrap())
            .join(format!("tree{}.rs", version.replace('.', "_")));
        let mut output = File::create(dest_path)?;
        rust::render(&tree, &svg, &mut output)?;
    }

    Ok(())
}
