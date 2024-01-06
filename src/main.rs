use std::fs::File;
use std::path::Path;

mod data;
mod svg;
mod tree;

fn get_trees() -> impl Iterator<Item = String> {
    vec!["3.23".to_owned()].into_iter()
}

fn main() -> anyhow::Result<()> {
    for version in get_trees() {
        let svg = format!("{version}.svg");

        let data_path = Path::new(".")
            .join("data")
            .join(format!("tree-{version}"))
            .join("data.json");

        let data = data::Tree::read(data_path)?;
        let tree = tree::build(&data);

        let dest_path = Path::new("out").join(&svg);
        let mut output = File::create(dest_path)?;
        svg::render(&tree, &mut output)?;
    }

    Ok(())
}
