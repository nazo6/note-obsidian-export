use obsidian_export::{Exporter, WalkOptions};
use std::{env, path::PathBuf};

fn main() {
    let mut exporter = Exporter::new(
        env::current_dir().unwrap().join("temp/source"),
        env::current_dir().unwrap().join("temp/target"),
    );
    let walk_options = WalkOptions {
        filter_fn: Some(&|entry| {
            if entry.file_type().is_some_and(|t| t.is_file()) {
                let content = std::fs::read_to_string(entry.path()).unwrap();
                let Ok(Some(yaml)) = frontmatter::parse(&content) else { return false };
                let publish = yaml["publish"].as_bool().unwrap();
                if publish {
                    println!("Adding items {:?} {}", entry.path(), publish);
                }
                publish
            } else {
                true
            }
        }),
        ..Default::default()
    };
    exporter.walk_options(walk_options);
    exporter.run().unwrap();
}
