use std::error::Error;
use std::fs::create_dir_all;
use std::fs::DirEntry;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use clap::Parser;
use regex::Captures;
use regex::Regex;

mod pypi;
mod visit_files;

#[derive(Parser)]
struct Cli {
    /// Project name
    project_name: String,
    /// Project target path
    #[clap(
        parse(from_os_str),
        default_value = "./",
        short = 'o',
        long = "out-path"
    )]
    dir_path: std::path::PathBuf,
    /// The version of python to target
    #[clap(default_value = "3.8", long = "py-version")]
    py_version: String,
}

impl Cli {
    fn get_black_py_version(&self) -> String {
        format!("py{}", self.py_version.split(".").collect::<String>())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Hello. Welcome to Py-Project-CLI!!!");

    let re = Regex::new(r#"\$\{(?P<section_name>[\w\s\-:.=_<>"]+)\}"#)?;
    let args = Cli::parse();
    let dev_req = pypi::get_requirements::get_dev_requirements().await?;

    let template_path = Path::new("./templates");
    let output_path = args.dir_path.join(args.project_name.clone());

    visit_files::visit_files(template_path, &|entry: &DirEntry| {
        let path = entry.path();
        let new_path = output_path.join(path.strip_prefix(template_path).ok().unwrap());
        let mut new_path_dir = new_path.clone();
        new_path_dir.pop();
        create_dir_all(new_path_dir).expect("error creating dirs");

        let mut file = File::open(path).expect("");

        let mut content = String::new();
        file.read_to_string(&mut content).expect("");

        let formatted_content = re.replace_all(content.as_str(), |caps: &Captures| {
            match caps.name("section_name") {
                Some(group) => match group.as_str() {
                    "project-name" => args.project_name.clone(),
                    "project-description" => args.project_name.clone(),
                    "py-version" => args.py_version.clone(),
                    "black-py-version" => args.get_black_py_version(),
                    "testing-requirements" => dev_req.clone(),
                    "development-requirements" => dev_req.clone(),
                    _ => String::new(),
                },
                None => String::new(),
            }
        });

        let mut out_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(new_path)
            .expect("msg");

        out_file
            .write_all(formatted_content.as_bytes())
            .expect("msg");
    })?;

    Ok(())
}
