mod models;

use anyhow::{bail, Context};
use indexmap::IndexMap;
use itertools::Itertools;
use models::Bot;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    process,
    str::FromStr,
};

#[derive(Clone, Deserialize, Serialize)]
pub enum Maturity {
    Obsolete,
    Alpha,
    Beta,
    Stable,
}

impl FromStr for Maturity {
    type Err = ();

    fn from_str(input: &str) -> Result<Maturity, Self::Err> {
        match input.to_lowercase().trim() {
            "obsolete" => Ok(Maturity::Obsolete),
            "not actively maintained" => Ok(Maturity::Obsolete),
            "alpha" => Ok(Maturity::Alpha),
            "beta" => Ok(Maturity::Beta),
            "stable" => Ok(Maturity::Stable),
            "released" => Ok(Maturity::Stable),
            _ => Err(()),
        }
    }
}

pub enum ProjectType {
    Bot,
    Bridge,
    Client,
    Sdk,
}

impl FromStr for ProjectType {
    type Err = ();

    fn from_str(input: &str) -> Result<ProjectType, Self::Err> {
        match input.to_lowercase().trim() {
            "bot" => Ok(ProjectType::Bot),
            "bridge" => Ok(ProjectType::Bridge),
            "client" => Ok(ProjectType::Client),
            "sdk" => Ok(ProjectType::Sdk),
            _ => Err(()),
        }
    }
}

fn main() -> anyhow::Result<()> {
    // Usage
    // migrator [bot|bridge|client|sdk] data/fromGatsby.mdx target_dir/target.toml
    // Careful: target.toml can already exist
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() != 3 {
        eprintln!(
            "takes exactly 3 arguments (type, path to gatsby file, path to toml output file)"
        );
        process::exit(1)
    }

    let project_type = ProjectType::from_str(args.get(0).unwrap()).expect(
        "Couldn't map first argument to a project type. Must be one of bot, bridge, client, or sdk",
    );

    let input_path = Path::new(args.get(1).unwrap());
    let toml_path = Path::new(args.get(2).unwrap());

    let (yaml_frontmatter, markdown) = read_file_contents(input_path)?;
    let frontmatter_value: IndexMap<String, toml::Value> = serde_yaml::from_str(&yaml_frontmatter)
        .with_context(|| format!("reading frontmatter of `{}`", input_path.display()))?;

    match project_type {
        ProjectType::Bot => {
            let title = frontmatter_value
                .get("title")
                .expect("Project doesn't contain title")
                .as_str()
                .expect("Project title is not a string");
            let description = frontmatter_value
                .get("description")
                .expect("Project doesn't contain description")
                .as_str()
                .expect("Project description is not a string");
            let author = frontmatter_value
                .get("author")
                .expect("Project doesn't contain author")
                .as_str()
                .expect("Project author is not a string");
            let maturity = Maturity::from_str(
                frontmatter_value
                    .get("maturity")
                    .expect("Project doesn't contain maturity")
                    .as_str()
                    .expect("Project maturity is not a string"),
            )
            .expect("Project maturity is not valid");
            let language = frontmatter_value
                .get("language")
                .expect("Project doesn't contain language")
                .as_str()
                .expect("Project language is not a string");
            let license = frontmatter_value
                .get("license")
                .expect("Project doesn't contain a licence")
                .as_str()
                .expect("Project licence is not a string");
            let repo = frontmatter_value
                .get("repo")
                .expect("Project doesn't contain repo information")
                .as_str()
                .expect("Project repo is not a string");

            let bot = Bot {
                title: title.to_string(),
                description: description.to_string(),
                author: author.to_string(),
                maturity: maturity,
                language: language.to_string(),
                license: license.to_string(),
                repo: repo.to_string(),
            };

            bot.update_toml(toml_path).expect("Failed to update toml file");
        }
        ProjectType::Bridge => {}
        ProjectType::Client => {}
        ProjectType::Sdk => {}
    }

    Ok(())
}

fn read_file_contents(input_path: &Path) -> anyhow::Result<(String, String)> {
    let input = BufReader::new(File::open(input_path)?);
    let mut input_lines = input.lines();

    let first_line = input_lines.next().expect("input file is non-empty")?;
    assert_eq!(first_line, "---", "File must start with YAML frontmatter");

    let mut frontmatter = String::new();
    loop {
        match input_lines.next().transpose()? {
            Some(s) if s == "---" => break,
            Some(s) => {
                frontmatter += s.as_str();
                frontmatter.push('\n');
            }
            None => bail!("Couldn't find end of frontmatter"),
        }
    }

    let markdown = input_lines
        // Okay for I/O errors to panic in this simple script
        .map(|result| result.unwrap())
        .join("\n");

    Ok((frontmatter, markdown))
}
