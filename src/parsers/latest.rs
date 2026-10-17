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
		})? // FIXME: Replace the "?" by ".map_err()" function to avoid random error thrown like coments in JSON.
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
		serde_json::Value::String(value) => value.clone(),
    	serde_json::Value::Number(value) if value.is_i64() || value.is_u64() => value.to_string(),
		_ => {
			println!("{}", format!("[ERROR] \"forge_version\" must be a string or integer").red());
			return Err(std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				"forge_version must be a string or integer",
			).into());
		}
	};
	
	Ok(ModInfo {
		id: settings["modinator"]["id"].to_string(),
		version: settings["modinator"]["mod_version"].to_string(),
		display_name: settings["modinator"]["display_name"].to_string(),
		description: if settings["pack"]["description"].is_null() {String::new()} else {settings["pack"]["description"].to_string()},
		authors: settings["modinator"]["authors"]
			.as_array()
			.ok_or_else(|| {
						println!("{}", format!("[ERROR] PACK.JSON: \"authors\" must be an array:\n     like: [\"author1\", \"author2\", etc...]").red());
						return io::Error::new(io::ErrorKind::InvalidData, "authors must be an array");
					})?
    		.iter()
			.map(|author| {
				author
					.as_str()
					.map(String::from)
					.ok_or_else(||	{
						println!("{}", format!("[ERROR] PACK.JSON: Each author in \"authors\" must be a string:\n     like: [\"author1\", \"author2\", etc...]").red());
						return io::Error::new(io::ErrorKind::InvalidData, "each author must be a string");
					})
    		})
    		.collect::<Result<Vec<_>, _>>()?,
		license: settings["modinator"]["license"].to_string(),
		forge_version,
		icon: PathBuf::from(settings["paths"]["icon"]
			.as_str()
			.ok_or_else(|| {
				println!("{}", format!("[ERROR] pack.json[\"paths\"][\"icon\"] must be a string.").red());
				return io::Error::new(
					io::ErrorKind::InvalidData,
					"pack.json: paths.icon must be a string",
				);
			})?),
		paths: 
			format!(
				"assets = {}\ndata = {}\nicon = {}", 
				if settings["paths"]["assets"].is_string() {settings["paths"]["assets"].as_str().unwrap()} else {""}, 
				if settings["paths"]["data"].is_string() {settings["paths"]["data"].as_str().unwrap()} else {""}, 
				if settings["paths"]["icon"].is_string() {settings["paths"]["icon"].as_str().unwrap()} else {""}
			)
			.parse::<Table>().unwrap()
	})
	/* PathsSettings will return:
		assets = "assets/"
		data = "data/"
		icon = "icon.png"
	*/
}