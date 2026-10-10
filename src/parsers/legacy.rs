use crate::*;

pub fn get() -> Result<ModInfo, Box<dyn std::error::Error>>{
	println!("[DEBUG] Parsing old project archicture... (PS: Use the new system for new functionalities.)");
	
	/*if Path::new("settings.toml").exists() {
		println!("[DEBUG] Project file: {}", "settings.toml")
	} else {
		return Err(());
	};*/
	let settings: toml::Table = toml::from_str(&fs::read_to_string("settings.toml")?)?; // Use pack.json on the new parser.


	let info = settings["mod_info"].as_table().unwrap();
	let paths = settings["paths"].as_table().unwrap();
	println!("test: {}", settings["paths"].as_table().unwrap());
	// Check for missing keys
	let expected_keys = [
		"id",
		"version",
		"display_name",
		"description",
		"authors",
		"license",
		"forge_version",
	];

	for key in info.keys() {
		if !expected_keys.contains(&key.as_str()) {
			println!("{}", format!("[ERROR] Missing keys in settings: {}", key).red());
			return Err(std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				"Missing required key in settings",
			)
			.into());
		}
	}
	let forge_version = match &info["forge_version"] {
		toml::Value::String(value) => value.clone(),
		toml::Value::Integer(value) => value.to_string(),
		_ => {
			return Err(std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				"forge_version must be a string or integer",
			).into());
		}
	};
	Ok(ModInfo { 
		id: info["id"].as_str().unwrap().to_string(), 
		version: info["version"].as_str().unwrap().to_string(), 
		display_name: info["display_name"].as_str().unwrap().to_string(), 
		description: info["description"].as_str().unwrap().to_string(), 
		license: info["license"].as_str().unwrap().to_string(),
		forge_version,
		authors: info["authors"]
			.as_array()
			.unwrap()
			.iter()
			.map(|author| author.as_str().unwrap().to_string())
			.collect(),
		icon: paths
			.get("icon")
			.and_then(|path| path.as_str())
			.map(Path::new)
			.map(|path| path.is_file())
			.unwrap_or(false),
		paths: paths.clone()
	})
}