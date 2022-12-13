use std::{
    fs::{self, File},
    io::Read,
    path::Path, collections::BTreeMap,
};

use serde::{Deserialize, Serialize};

use crate::Maturity;

#[derive(Clone, Deserialize, Serialize)]
pub struct Bot {
    #[serde(skip_serializing)]
    pub title: String,
    pub description: String,
    pub author: String,
    pub maturity: Maturity,
    pub language: String,
    pub license: String,
    pub repo: String,
}

impl Bot {
    pub fn update_toml(&self, path: &Path) {
        // Open & parse toml file
        let mut input_string = String::new();
        File::open(path)
            .expect("Could not open target toml file")
            .read_to_string(&mut input_string)
            .expect("Failed to read toml file");
         let mut bots: BTreeMap<String, Bot> = toml::from_str(&input_string).expect("Could not parse toml file");

        // // See if a project with the same title exists
        if let Some(project) = bots.get_mut(&self.title) {
            // If yes: update project in the toml file
            *project = self.clone();
        } else {
            // If not: append project to the toml file
            bots.insert(self.title.clone(), self.clone());
        }

        let bots_str = toml::to_string(&bots).expect("Couldn't serialise bots to a string");
        fs::write(path, bots_str).expect("Failed to write toml file");
    }
}

pub struct Bridge {}

pub struct Client {}

pub struct Sdk {}
