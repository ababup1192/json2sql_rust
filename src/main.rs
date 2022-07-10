use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;
extern crate serde_json;

fn main() -> io::Result<()> {
    let mut family_vec: Vec<Parent> = Vec::new();

    if let Ok(files) = read_dir("jsons") {
        for file_name in files {
            let mut f = File::open(&file_name).expect("file not found");
            let mut contents = String::new();
            f.read_to_string(&mut contents)?;
            let parent = serde_json::from_str::<Parent>(&contents).unwrap();
            family_vec.push(parent);
        }
    }
    for parent in &family_vec {
        println!(
            r#"
INSERT INTO `parents` (
    `id`, `title`
) VALUES (
    {}, "{}"
)
        "#,
            parent.id, parent.title
        );

        for (sort_order, child) in parent.children.iter().enumerate() {
            println!(
                r#"
INSERT INTO `children` ( 
    `id`,
    `parent_id`,
    `title`,
    `sort_order`
) VALUES (
    {}, {}, "{}", {}
);
                "#,
                child.id, parent.id, child.title, sort_order
            );
        }
    }
    Ok(())
}

fn read_dir(path: &str) -> Result<Vec<path::PathBuf>, Box<dyn Error>> {
    let dir = fs::read_dir(path)?;
    let mut files: Vec<path::PathBuf> = Vec::new();
    for item in dir.into_iter() {
        files.push(item?.path());
    }
    Ok(files)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Parent {
    id: u64,
    title: String,
    children: Vec<Child>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Child {
    id: u64,
    title: String,
}
