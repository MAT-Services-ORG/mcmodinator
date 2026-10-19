use crate::*;

pub fn get() -> Result<ModInfo, Box<dyn std::error::Error>>{
	println!("[DEBUG] Parsing old project archicture... (PS: Use the new system for new functionalities.)");
	let settings: toml::Table = toml::from_str(&fs::read_to_string("settings.toml").map_err(|error| {
		let message = if error.kind() == io::ErrorKind::NotFound {
			format!("\"settings.toml\" not found in project")
		} else {
			format!("Could not read settings.toml: {error}")
		};
		println!("{}", format!("[ERROR] {}", message).red());
		io::Error::new(error.kind(), message)
	})?)?; // Use pack.json on the new parser.

	let info = settings["mod_info"].as_table().unwrap(); // TOFIX: Add OK_OR_ELSE? instead of UNWRAP to avoid errors.
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
		"forge_version"
	];

	for key in expected_keys {
		if !info.contains_key(key) {
			println!("{}", format!("[ERROR] Missing keys in settings: {}", key).red());
			return Err(std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				"Missing required key in settings",
			)
			.into());
		};
	};

	let forge_version = match &info["forge_version"] {
		toml::Value::String(value) => value.clone(),
		toml::Value::Integer(value) => value.to_string(),
		_ => {
			println!("{}", "[ERROR] forge_version must be a string or integer".red());
			return Err(std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				"forge_version must be a string or integer",
			).into());
		}
	};
	let neoforge_version = match info.get("neoforge_version") {
		Some(toml::Value::String(value)) => value.clone(),
		Some(toml::Value::Integer(value)) => value.to_string(),
		None => String::new(),
		_ => {
			println!("{}", "[ERROR] neoforge_version must be a string or integer".red());
			return Err(std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				"neoforge_version must be a string or integer",
			)
			.into());
		}
	};
	Ok(ModInfo { 
		id: info["id"].as_str()
			.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "id must be a string"))?
			.to_owned(), 
		version: info["version"].as_str().unwrap().to_string(), 
		display_name: info["display_name"].as_str().unwrap().to_string(), 
		description: info["description"].as_str().unwrap().to_string(), 
		license: info["license"].as_str().unwrap().to_string(),
		forge_version,
		neoforge_version,
		authors: info["authors"]
			.as_array()
			.unwrap()
			.iter()
			.map(|author| author.as_str().unwrap().to_string())
			.collect(),
		icon: PathBuf::from(paths["icon"].as_str()
			.ok_or_else(|| {
				println!("{}", format!("[ERROR] settings.toml[\"paths\"][\"icon\"] must be a string.").red());
				return io::Error::new(
					io::ErrorKind::InvalidData,
					"settings.toml: paths.icon must be a string",
				);
			})?),
		paths: paths.clone()
	})
}