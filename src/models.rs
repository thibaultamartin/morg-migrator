use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::Read,
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::Maturity;

#[derive(Clone, Deserialize, Serialize)]
pub struct Bot {
    #[serde(skip_serializing, skip_deserializing)]
    pub title: String,
    pub summary: String,
    pub maintainer: String,
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
        let mut bots: BTreeMap<String, Bot> =
            toml::from_str(&input_string).expect("Could not parse toml file");

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

pub struct Bridge {
    pub title: String,
    pub summary: String,
    pub maintainer: String,
    pub maturity: Maturity,
    pub language: String,
    pub license: String,
    pub repo: String,
    pub platform: Vec<String>,
}

// Not doing client on purpose: data is outdated and needs to be audited

#[derive(Clone, Deserialize, Serialize)]
pub struct Sdk {
    #[serde(skip_serializing, skip_deserializing)]
    pub title: String,
    pub maintainer: String,
    pub language: String,
    pub license: String,
    pub repository: String,
    pub purpose: Vec<String>,
    pub featured_in: Vec<String>,
}

impl Sdk {
    pub fn update_toml(&mut self, path: &Path) {
        // Open & parse toml file
        let mut input_string = String::new();
        File::open(path)
            .expect("Could not open target toml file")
            .read_to_string(&mut input_string)
            .expect("Failed to read toml file");
        let mut sdks: BTreeMap<String, Sdk> =
            toml::from_str(&input_string).expect("Could not parse toml file");

        // // See if a project with the same title exists
        if let Some(project) = sdks.get_mut(&self.title) {
            // If yes: update project in the toml file
            self.purpose = project.purpose.clone();
            self.featured_in = project.featured_in.clone();
            *project = self.clone();
        } else {
            // If not: append project to the toml file
            sdks.insert(self.title.clone(), self.clone());
        }

        let bots_str = toml::to_string(&sdks).expect("Couldn't serialise sdks to a string");
        fs::write(path, bots_str).expect("Failed to write toml file");
    }
}
