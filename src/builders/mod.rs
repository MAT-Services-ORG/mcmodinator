use crate::*;

fn import_folder(
	given_path: Option<&serde_json::value::Value>,
	zip: &mut ZipWriter<File>,
	output_name: &str,
	output_type: &str
) -> Result<(), Box<dyn std::error::Error>> {
	if let Some(path) = given_path.and_then(|x| x.as_str()) {
		if !path.is_empty() {
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
					} else {
						println!("{}", format!("[ERROR] {error}").red());
					}
				}
			}
		}
	}
	Ok(())
}
pub fn build(mod_info: &ModInfo, mod_build: &str, output_path: String) -> Result<(), Box<dyn std::error::Error>> {
	println!("[INFO] Building file..."); // Create ZIP/JAR
	let modfilename = match mod_build {
		"true" | "t" | "yes" | "y" => format!("{}/{}-mod.jar", output_path, mod_info.id),
		_ => mod_build.to_string()
	};
	let file = File::create(&modfilename).map_err(|error| {
		println!("{}", format!("[ERROR] {}", error.to_string()).red());
		io::Error::new(error.kind(), error.to_string())
	})?;
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

	if mod_info.icon.is_file() {
		fabric["icon"] = serde_json::Value::String(mod_info.icon.to_str().unwrap().to_string());
	}

	let fabric_json = serde_json::to_string(&fabric)?;

	zip.start_file(
		"fabric.mod.json",
		SimpleFileOptions::default(),
	)?;

	zip.write_all(fabric_json.as_bytes())?;


	// Forge MataData
	zip.start_file("META-INF/mods.toml", SimpleFileOptions::default())?;
	zip.write_all(format!(

r#"modLoader="lowcodefml"
loaderVersion="{forge_version}"

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
		logo = if mod_info.icon.is_file() {"logoFile=\"icon.png\""} else {""}
	).as_bytes())?;

	zip.start_file("META-INF/neoforge.mods.toml", SimpleFileOptions::default())?;
	zip.write_all(format!(
r#"modLoader="lowcodefml"
loaderVersion="{neo_version}" 

license="{license}"

[[mods]]
modId="{id}"
version="{version}"
displayName="{display_name}"
authors="{authors}"
description="{description}"
{logo}
"#,
		neo_version = mod_info.neoforge_version,
		license = mod_info.license,
		id = mod_info.id,
		version = mod_info.version,
		display_name = mod_info.display_name,
		authors = mod_info.authors.join(", "),
		description = mod_info.description,
		logo = if mod_info.icon.is_file() {"logoFile=\"icon.png\""} else {""}
	).as_bytes())?;

	// Icon
	if let Some(icon_path) = mod_info.paths.get("icon").and_then(|x| x.as_str()) {
		if !icon_path.is_empty() {
			match fs::read(icon_path) {
				Ok(data) => {
					zip.start_file(
						"icon.png",
						SimpleFileOptions::default(),
					)?;

					zip.write_all(&data)?;

					println!("{}", "[INFO] Added icon".blue());
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
	}


	// Data
	import_folder(mod_info.paths.get("data"), &mut zip, "Datapack Files", "data")?;

	// Assets
	import_folder(mod_info.paths.get("assets"), &mut zip, "Resourcepack Files", "assets")?;

	if !mod_info.paths["kubeloader"].is_null() { 
		print!(
			"{}",
			format!(
				"[CAUT] KUBELOADER technology is unfinished, problems can heppends if you chosse to continue.\n       ┃ We advise you to check if a relase is aviable: https://github.com/MAT-Services-ORG/mcmodinator/releases\n       ┃ Actual version: {}\n       ┃ Continue with KL integrations? [y/N]\n       ┣━╸ ",
				env!("CARGO_PKG_VERSION")
			)
			.bright_red()
		);
		io::stdout().flush()?;
		let mut input = String::new();
		io::stdin().read_line(&mut input)?;
		if input.trim().eq_ignore_ascii_case("y") {
			println!("{}","       ┃ Loading KubeLoader integrations at your risk.\n       ╹".bright_red());
			import_folder(mod_info.paths["kubeloader"].get("server-js"), &mut zip, "Server Scripts", "server_scripts")?; // server-js
			import_folder(mod_info.paths["kubeloader"].get("client-js"), &mut zip, "Client Scripts", "client_scripts")?; // client-js
			import_folder(mod_info.paths["kubeloader"].get("startup-js"), &mut zip, "Startup Scripts", "startup_scripts")?; // startup-js
			// TODO: Generate KUBELOADER metadatas.
		} else {
			println!("{}","       ┃ Aborting KubeLoader integration.\n       ╹".yellow());
		};
	} 

	// Finish ZIP
	zip.finish()?;

	println!("{}", format!("[DONE] Mod created at {modfilename}").green());
	Ok(())
}