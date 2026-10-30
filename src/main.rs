// This project is an hanced Rust equivalent of https://github.com/T0RNATO/datapackmodinator/ script.
// Voir les informations sur le fichier externe pour le tas d'amélioration et d'oublis restants.
use serde_json::json;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;
use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;

#[path = "parsers/legacy.rs"] mod plegacy;
#[path = "parsers/latest.rs"] mod platest;
#[path = "builders/mod.rs"] mod bmod;
#[path = "builders/cpack.rs"] mod bcpack;
#[path = "doc.rs"] mod doc;

#[derive(Parser)] #[command(name = "MAT Modinator", version = env!("CARGO_PKG_VERSION"), about = "MAT Modinator\nCreate a mod using DATA, ASSETS and KubeJS standrad folders.", long_about = None)] struct Args {
	// Start defining the CLI auto parser using CLAP. For more informations: https://crates.io/crates/clap

	/// Define project path
	#[arg(short, long, default_value_t = ".".to_string(), value_name = "FOLDER", help_heading = "Project Options")]
	project_path: String,

	/// Define output folder
	#[arg(short, long, default_value_t = ".".to_string(), value_name = "FOLDER", help_heading = "Project Options")]
	output_path: String,

	/// Set Mod output [default: <ID>-mod.jar]
	#[arg(short, long, default_value_t = "True".to_string(), hide_default_value = true, value_name = "BOOL or FILENAME", help_heading = "Build Options")]
	mod_build: String,

	/// Set ContentPack output [default: False (Set to "<ID>-CPack.jar" if True.)]
	#[arg(short, long, default_value_t = "False".to_string(), hide_default_value = true, value_name = "BOOL or FILENAME", help_heading = "Build Options")]
	cpack_build: String,

	/// Uses legacy parsing system (Check https://github.com/T0RNATO/datapackmodinator/ for informations)
	#[arg(short, long)]
	legacy_parser: bool,

	/// Print documentation
	#[arg(short, long, num_args = 0..=1, default_missing_value = "MAIN", hide_default_value = true, value_name = "TOPIC")]
	doc: Option<String>
}

pub struct ModInfo {
	id: String,
	version: String,
	display_name: String,
	description: String,
	authors: Vec<String>,
	license: String,
	forge_version: String,
	neoforge_version: String,
	icon: PathBuf,
	paths: serde_json::Value,
	dependency: serde_json::Value
}

fn add_folder_to_zip(
	zip: &mut ZipWriter<File>,
	folder_path: &Path,
	arcname: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
	let mut count = 0;

	for entry in fs::read_dir(folder_path).map_err(|error| {
		println!("{}", format!("[ERROR] {}", error).red());
		io::Error::new(error.kind(), error.to_string())
	})? {
		let path = entry?.path();

		if path.is_dir() {
			let relative = path.strip_prefix(folder_path)?;
			let sub_arcname = Path::new(arcname).join(relative);

			count += add_folder_to_zip(
				zip,
				&path,
				sub_arcname.to_str().unwrap(),
			)?;

		} else if path.is_file() {
			zip.start_file_from_path(Path::new(arcname).join(path.strip_prefix(folder_path)?), SimpleFileOptions::default())?;
			io::copy(&mut File::open(&path)?, zip)?;

			count += 1;
		}
	}

	Ok(count)
}

pub fn escape_toml_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());

    for ch in value.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            ch if ch.is_control() => {
                use std::fmt::Write;
                write!(&mut escaped, "\\u{:04X}", ch as u32).unwrap();
            }
            ch => escaped.push(ch),
        }
    }

    escaped
}

pub fn json_to_toml(value: serde_json::Value) -> String {
	let mut ctoml = String::new();
	for key in value.as_object().unwrap().keys() {
    	ctoml.push_str(format!("{key}={}", value[key]).as_str());
		ctoml.push('\n');
	};
	ctoml
}

pub fn import_folder(
	given_path: Option<&serde_json::value::Value>,
	zip: &mut ZipWriter<File>,
	output_name: &str,
	output_type: &str
) -> Result<(), Box<dyn std::error::Error>> {
	if let Some(path) = given_path.and_then(|x| x.as_str()) && !path.is_empty() {
		match add_folder_to_zip(
			zip,
			Path::new(path),
			output_type,
		) {
			Ok(count) => {
				println!("{}", format!("[INFO] Added {count} {output_name}").blue());
			}
			Err(error) => {
				if error
					.downcast_ref::<io::Error>()
					.is_some_and(|e| e.kind() == io::ErrorKind::NotFound)
				{
					println!("{}", format!("[WARN] {output_name} not found at provided path.").yellow());
					println!("{}\n{}{}", 
						"[HELP] For more informations about CLI usage, type:".blue(),
						"       ▐".blue(),
						"modinator --help".on_bright_black()
					);
					return Ok(());
				} else {
					println!("{}", format!("[ERROR] {error}").red());
					return Err(error);
				}
			}
		}
	}
	Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Parsing arguments
	let args: Args = Args::parse();
	match args.doc.as_deref() {
		None => {}
		Some("MAIN") => {doc::doc("MAIN"); return Ok(())}
		Some(topic) => {doc::doc(topic.to_ascii_uppercase().as_str()); return Ok(())}
	}

	// Summurizing and applying values
	println!("MAT Modinator\nModinator status:\n  Project path: {}\n  Input path: {}", args.project_path, args.output_path);
	std::env::set_current_dir(args.project_path)?;

	let mut mod_info = if args.legacy_parser {plegacy::get()} else {platest::get()}?;

	if !mod_info.icon.is_file() {
		if mod_info.icon.exists() {
			println!("{}", format!("[WARN] \"{}\" is not a file. Considering \"\".", mod_info.icon.to_str().unwrap()).yellow())
		} else {
			println!("{}", format!("[WARN] \"{}\" does not exists. Considering \"\".", mod_info.icon.to_str().unwrap()).yellow())
		}
	}

	if !matches!(args.mod_build.to_lowercase().as_str(), "false" | "f" | "no" | "n" | "none") {bmod::build(&mut mod_info, args.mod_build.to_lowercase().as_str(), &args.output_path)?}
	if !matches!(args.cpack_build.to_lowercase().as_str(), "false" | "f" | "no" | "n" | "none") {bcpack::build(&mut mod_info, args.cpack_build.to_lowercase().as_str(), &args.output_path)?}
	println!("{}", "[DONE] Finished execution !".green());
	Ok(())
}