// This project is an hanced Rust equivalent of no longer supported https://github.com/T0RNATO/datapackmodinator/ script.
// Voir les informations sur le fichier externe pour le tas d'amélioration et d'oublis restants.
use serde_json::{json, Value};
use std::fs::{self, File, /*exists*/};
use std::io::{self, Write};
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;
//use std::env::{self, args};
use clap::Parser;
use colored::Colorize;

#[path = "parsers/compat.rs"]
mod compat;

// Start defining the CLI auto parser using CLAP. For more informations: https://crates.io/crates/clap
#[derive(Parser)]
#[command(name = "MAT Modinator")]
#[command(version = "Alpha PRE")]
#[command(about = "MAT Modinator\nCreate a mod using DATA, ASSETS and KubeJS standrad folders.", long_about = None)]
struct Args {
	/// Define project path
	#[arg(short, long, default_value_t = ".".to_string(), value_name = "FOLDER")]
	project_path: String,

	/// Define output folder
	#[arg(short, long, default_value_t = ".".to_string(), value_name = "FOLDER")]
	output_path: String,

	/// Uses old parsing system (Check https://github.com/T0RNATO/datapackmodinator/ for informations, Experimental)
	#[arg(short, long)]
	compat_parser: bool
}

pub struct ModInfo {
    id: String,
    version: String,
    display_name: String,
    description: String,
    authors: Vec<String>,
    license: String,
    forge_version: String,
	icon: bool,
	paths: toml::map::Map<String, toml::Value> 
}


/*pub fn format_text(text: &str, codes: &[u32]) -> String {
	let mut out = String::new();

	for code in codes {
		out += &format!("\x1b[{}m", code);
	}

	out + text + "\x1b[0m"
}*/

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

/*fn parse() -> Result<ModInfo, Box<dyn std::error::Error>>{
	println!("[DEBUG] Parsing project archicture...");
	
	let settings: serde_json::Value = serde_json::from_str(&fs::read_to_string("pack.json")?).unwrap();
	println!("{}", settings);
	// Everithing that are here are dummies. There're not real working code.
	//let settings: toml::Table = toml::from_str(&fs::read_to_string("settings.toml")?)?;
	Ok(ModInfo {
		id: String::new(),
		version: String::new(),
		display_name: String::new(),
		description: String::new(),
		authors: Vec::new(),
		license: String::new(),
		forge_version: String::new(),
		icon: true,
		paths: toml::from_str(settings["paths"].to_string())
	})
}*/


fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Parsing arguments
	let args: Args = Args::parse();

	// Summurizing and applying values
	println!("MAT Modinator\nModinator status:\n  Project path: {}\n  Input path: {}", args.project_path, args.output_path);
	std::env::set_current_dir(args.project_path)?;

	let mod_info = if args.compat_parser {
		compat::get()?
	}
	else {
		//parse()?
		compat::get()?
	};

	println!("Building file..."); // Create ZIP/JAR
	let file = File::create("mod.jar")?;
	let mut zip = ZipWriter::new(file);
	
	// Fabric
	let mut fabric = json!({
		"schemaVersion": 1,
		"id": mod_info.id,
		"version": mod_info.version,
		"name": mod_info.display_name,
		"description": mod_info.description,
		"authors": mod_info.authors,
		"license": mod_info.license,
	});

	if mod_info.icon {
		fabric["icon"] = Value::String("icon.png".to_string());
	}

	let fabric_json = serde_json::to_string(&fabric)?;

	zip.start_file(
		"fabric.mod.json",
		SimpleFileOptions::default(),
	)?;

	zip.write_all(fabric_json.as_bytes())?;


	// Forge
	let forge_toml = format!(
		r#"modLoader="lowcodefml"
loaderVersion="[{forge_version},)"

license="{license}"

[[mods]]
modId="{id}"
version="{version}"
displayName="{display_name}"
authors="{authors}"
description="{description}"
{logo}
"#,
		forge_version = mod_info.forge_version,
		license = mod_info.license,
		id = mod_info.id,
		version = mod_info.version,
		display_name = mod_info.display_name,
		authors = mod_info.authors.join(", "),
		description = mod_info.description,
		logo = if mod_info.icon {"logoFile=\"icon.png\""} else {""}
	);


	zip.start_file(
		"META-INF/mods.toml",
		SimpleFileOptions::default(),
	)?;

	zip.write_all(forge_toml.as_bytes())?;


	// Icon
	if let Some(icon_path) = mod_info.paths.get("icon").and_then(|x| x.as_str()) {
		match fs::read(icon_path) {
			Ok(data) => {
				zip.start_file(
					"icon.png",
					SimpleFileOptions::default(),
				)?;

				zip.write_all(&data)?;

				println!("Added icon");
			}
			
			Err(error) => {
				if error.kind() == io::ErrorKind::NotFound {
				
					println!(
						"{}", "[WARN] Icon not found at provided path.".yellow(),
					);
				} else {
					println!("{}", format!("[ERROR] {}", error).red());
				}
			}
		}
	}


	// Datapack
	if let Some(data_path) = mod_info.paths.get("data").and_then(|x| x.as_str()) {
		match add_folder_to_zip(
			&mut zip,
			Path::new(data_path),
			"data",
		) {
			Ok(count) => {
				println!("Added {} datapack files", count);
			}

			Err(error) => {
				if error
					.downcast_ref::<io::Error>()
					.is_some_and(|e| e.kind() == io::ErrorKind::NotFound)
				{
					
					println!("{}", "[WARN] Datapack files not found at provided path.".yellow());
				} else {
					println!("{}", format!("[ERROR] {}", error).red());
				}
			}
		}
	}


	// Resource pack
	if let Some(assets_path) = mod_info.paths.get("assets").and_then(|x| x.as_str()) {
		match add_folder_to_zip(
			&mut zip,
			Path::new(assets_path),
			"assets",
		) {
			Ok(count) => {
				println!("Added {} resource pack files", count);
			}

			Err(error) => {
				if error
					.downcast_ref::<io::Error>()
					.is_some_and(|e| e.kind() == io::ErrorKind::NotFound)
				{
					println!("{}", "[WARN] Resource pack files not found at provided path.".yellow());
				} else {
					println!("{}", format!("[ERROR] {}", error).red());
				}
			}
		}
	}


	// Finish ZIP
	zip.finish()?;


	println!("{}", "[DONE] Mod created at mod.jar".green());
	return Ok(());
}