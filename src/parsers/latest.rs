use crate::*;

pub fn get() -> Result<ModInfo, Box<dyn std::error::Error>> {
	println!("[DEBUG] Parsing project archicture...");
	
	let settings: serde_json::Value = serde_json::from_str(
		&fs::read_to_string("pack.json").map_err(|error| {
			let message = if error.kind() == io::ErrorKind::NotFound {
				format!("\"pack.json\" not found in project")
			} else {
				format!("Could not read \"pack.json\": {error}")
			};
			println!("{}", format!("[ERROR] {}", message).red());
			io::Error::new(error.kind(), message)
		})?
	)?;
	println!("{settings}");

	let expected_keys = [
		"id",
		"display_name",
		"mod_version",
		"authors",
		"license",
		"forge_version"
	];

	for key in expected_keys {
		if settings["modinator"][key].is_null() {
			println!("{}", format!("[ERROR] Missing keys in modinator: {}", key).red());
			return Err(std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				"Missing required key in settings",
			).into());
		}
	};

	let forge_version = match &settings["modinator"]["forge_version"] {
		value => value.clone().to_string(),
		_ => {
			return Err(std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				"forge_version must be a string or integer",
			).into());
		}
	};
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
		paths: toml::from_str("assets = \"assets/\"\ndata = \"data/\"\nicon = \"icon.png\"")?
	})
	/* PathsSettings will return:
		assets = "assets/"
		data = "data/"
		icon = "icon.png"
	*/
	// */ Ok(())
}