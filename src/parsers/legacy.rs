use crate::*;

pub fn get() -> Result<ModInfo, Box<dyn std::error::Error>>{
	println!("[DEBUG] Parsing old project archicture... (PS: Use the new system for new functionalities.)");
	let settings: toml::Table = toml::from_str(&fs::read_to_string("settings.toml").map_err(|error| {
		let message = if error.kind() == io::ErrorKind::NotFound {
			"\"settings.toml\" not found in project".to_string()
		} else {
			format!("Could not read settings.toml: {error}")
		};
		println!("{}", format!("[ERROR] {}", message).red());
		io::Error::new(error.kind(), message)
	})?)?; // Use pack.json on the new parser.

	let info = settings["mod_info"]
		.as_table()
		.ok_or_else(|| {
			println!("{}", "[ERROR] Cannot parse [mod_info]. Check if your syntax is valid.".red());
			io::Error::new(io::ErrorKind::InvalidData, "cannot parse [mod_info] in settings.toml")
		})?; 
	let paths: serde_json::Value = json!({
		"assets": settings["paths"]["assets"],
		"data": settings["paths"]["data"],
		"icon": settings["paths"]["icon"]
	});

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

	Ok(ModInfo {
		id: info["id"].as_str()
			.ok_or_else(|| {
				println!("{}", "[ERROR] id must be a string".red());
				io::Error::new(io::ErrorKind::InvalidData, "id must be a string")
			})?
			.to_owned(), 
		version: info["version"]
			.as_str()
			.ok_or_else(|| {
				println!("{}", "[ERROR] version must be a string or integer".red());
				io::Error::new(io::ErrorKind::InvalidData, "version must be a string or integer")
			})?
			.to_string(), 
		display_name: info["display_name"]
			.as_str()
			.ok_or_else(|| {
				println!("{}", "[ERROR] display_name must be a string".red());
				io::Error::new(io::ErrorKind::InvalidData, "display_name must be a string")
			})?
			.to_string(), 
		description: info["description"]
			.as_str()
			.ok_or_else(|| {
				println!("{}", "[ERROR] description must be a string".red());
				io::Error::new(io::ErrorKind::InvalidData, "description must be a string")
			})?
			.to_string(), 
		license: info["license"]
			.as_str()
			.ok_or_else(|| {
				println!("{}", "[ERROR] license must be a string".red());
				io::Error::new(io::ErrorKind::InvalidData, "license must be a string")
			})?
			.to_string(),
		forge_version: match &info["forge_version"] {
			toml::Value::String(value) => value.clone(),
			toml::Value::Integer(value) => value.to_string(),
			_ => {
				println!("{}", "[ERROR] forge_version must be a string or integer".red());
				return Err(std::io::Error::new(
					std::io::ErrorKind::InvalidData,
					"forge_version must be a string or integer",
				).into());
			}
		},
		neoforge_version: match info.get("neoforge_version") {
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
		},
		authors: info["authors"]
			.as_array()
			.ok_or_else(||{
				println!("{}", "[ERROR] authors must be an array".red());
				io::Error::new(io::ErrorKind::InvalidData, "authors must be an array")
			})?
			.iter()
			.map(|author| 
				author
					.as_str()
					.ok_or_else(|| {
						println!("{}", "[ERROR] each author in authors must be a string".red());
						io::Error::new(io::ErrorKind::InvalidData, "not a string in authors = [String]")
					})
				.map(str::to_owned)
			)
			.collect::<Result<Vec<_>, io::Error>>()?,
		icon: PathBuf::from(paths["icon"].as_str()
			.ok_or_else(|| {
				println!("{}", "[ERROR] settings.toml[\"paths\"][\"icon\"] must be a string.".red());
				io::Error::new(
					io::ErrorKind::InvalidData,
					"settings.toml: paths.icon must be a string",
				)
			})?),
		paths,//: paths.clone()
		dependency: json!({}) // TODO: Legacy Dependencies
	})
}