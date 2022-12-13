use std::{fs::File, io::Read, path::Path};

use serde::{Deserialize, Serialize};

use crate::Maturity;

#[derive(Clone, Deserialize, Serialize)]
pub struct Bot {
    pub title: String,
    pub description: String,
    pub author: String,
    pub maturity: Maturity,
    pub language: String,
    pub license: String,
    pub repo: String,
}

impl Bot {
    pub fn update_toml(&self, path: &Path) -> Result<(), ()> {
        // Open & parse toml file
        let mut input_string = String::new();
        File::open(path)
            .expect("Could not open target toml file")
            .read_to_string(&mut input_string)
            .expect("Failed to read toml file");
        let mut bots: Vec<Bot> = toml::from_str(&input_string).expect("Could not parse toml file");

        // See if a project with the same title exists
        if let Some(project) = bots.iter_mut().find(|x| x.title == self.title) {
            // If yes: update project in the toml file
            *project = self.clone();
        } else {
            // If not: append project to the toml file
            bots.push(self.clone());
        }
        

        Ok(())
    }
}

pub struct Bridge {}

pub struct Client {}

pub struct Sdk {}
