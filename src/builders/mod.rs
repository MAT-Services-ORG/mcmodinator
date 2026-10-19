use crate::*;

pub fn build(mod_info: &ModInfo, mod_build: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("[INFO] Building file..."); // Create ZIP/JAR
	let modfilename = match mod_build {
		"true" | "t" | "yes" | "y" => format!("{}-mod.jar", mod_info.id),
		_ => mod_build.to_string()
	};
	let file = File::create(&modfilename)?;
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
		fabric["icon"] = Value::String(mod_info.icon.to_str().unwrap().to_string());
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
		logo = if mod_info.icon.is_file() {"logoFile=\"icon.png\""} else {""}
	);


	zip.start_file(
		"META-INF/mods.toml",
		SimpleFileOptions::default(),
	)?;

	zip.write_all(forge_toml.as_bytes())?;

	// TODO: NeoForge
	
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
	}


	// Datapack
	if let Some(data_path) = mod_info.paths.get("data").and_then(|x| x.as_str()) {
		if !data_path.is_empty() {
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
	}


	// Resource pack
	if let Some(assets_path) = mod_info.paths.get("assets").and_then(|x| x.as_str()) {
		if !assets_path.is_empty() {
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
	}


	// Finish ZIP
	zip.finish()?;


	println!("{}", format!("[DONE] Mod created at {modfilename}").green());
    Ok(())
}