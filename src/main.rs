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
	paths: serde_json::Value
}

fn add_folder_to_zip(
	zip: &mut ZipWriter<File>,
	folder_path: &Path,
	arcname: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
	let mut count = 0;

	for entry in fs::read_dir(folder_path)? {
		let entry = entry?;
		let path = entry.path();

		if path.is_dir() {
			let relative = path.strip_prefix(folder_path)?;
			let sub_arcname = Path::new(arcname).join(relative);

			count += add_folder_to_zip(
				zip,
				&path,
				sub_arcname.to_str().unwrap(),
			)?;
		} else if path.is_file() {
			let relative = path.strip_prefix(folder_path)?;
			let archive_path = Path::new(arcname).join(relative);

			let archive_path = archive_path.to_str().unwrap();

			zip.start_file(archive_path, SimpleFileOptions::default())?;

			let mut file = File::open(&path)?;
			io::copy(&mut file, zip)?;

			count += 1;
		}
	}

	Ok(count)
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

	let mod_info = if args.legacy_parser {
		plegacy::get()?
	}
	else {
		platest::get()?
	};

	if !mod_info.icon.is_file() {
		if mod_info.icon.exists() {
			println!("{}", format!("[WARN] \"{}\" is not a file. Considering \"\".", mod_info.icon.to_str().unwrap()).yellow())
		} else {
			println!("{}", format!("[WARN] \"{}\" does not exists. Considering \"\".", mod_info.icon.to_str().unwrap()).yellow())
		}
	}

	if match args.mod_build.to_lowercase().as_str() {
		"false" | "f" | "no" | "n" | "none" => false,
		_ => true
	} {bmod::build(&mod_info, args.mod_build.to_lowercase().as_str(), args.output_path)?}
	if match args.cpack_build.to_lowercase().as_str() {
		"false" | "f" | "no" | "n" | "none" => false,
		_ => true
	} {bcpack::build(/*&mod_info, args.mod_build.to_lowercase().as_str(), args.output_path*/)?}
	println!("{}", "[DONE] Finished execution !".green());
	return Ok(());
}

/* TODO:
	- Dependencies
		Forge: 
			[[dependencies.confluencerechiseled]]
			    modId="minecraft"
				type="required"
				# This version range declares a minimum of the current minecraft version up to but not including the next major version
				versionRange="[1.21.1]"
				ordering="NONE"
				side="BOTH"
		Fabric: ???
*/