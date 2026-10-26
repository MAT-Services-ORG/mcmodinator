use crate::*;

pub fn get() -> Result<ModInfo, Box<dyn std::error::Error>> {
	println!("[DBUG] Parsing project archicture...");
	
	let settings: serde_json::Value = serde_json::from_str(
		&fs::read_to_string("pack.mcmeta").map_err(|err| {
			let msg = if err.kind() == io::ErrorKind::NotFound {
				"\"pack.mcmeta\" not found in project".to_string()
			} else {
				format!("Could not read \"pack.mcmeta\": {err}")
			};
			println!("{}", format!("[ERROR] {}", msg).red());
			io::Error::new(err.kind(), msg)
		})? 
	).map_err(|err| {
		let msg = if err.classify() == serde_json::error::Category::Syntax {
			"Cannot read \"pack.mcmeta\". Have you removed the comments?".to_string()
		} else {
			err.to_string()
		};
		println!("{}", format!("[ERROR] {}", msg).red());
		println!("{}\n{}{}", 
			"[HELP] For more informations about PACK.MCMETA structure, type:".blue(),
			"       ▐".blue(),
			"modinator --doc PACK".on_bright_black()
		);
		io::Error::new(serde_json::Error::io_error_kind(&err).unwrap_or(io::ErrorKind::InvalidData), msg)
	})?;

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
			println!("{}\n{}{}", 
				"[HELP] For more informations about PACK.MCMETA structure, type:".blue(),
				"       ▐".blue(),
				"modinator --doc PACK".on_bright_black()
			);
			return Err(std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				"Missing required key in settings",
			).into());
		}
	};

	let dependency = if settings["dependencies"].is_null() {json!({})} else {settings["dependencies"].clone()};

	Ok(ModInfo {
		id: settings["modinator"]["id"]
			.as_str()
			.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "id must be a string"))?
			.to_owned(),
		version: settings["modinator"]["mod_version"]
			.as_str()
			.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "version must be a string"))?
			.to_owned(),
		display_name: settings["modinator"]["display_name"]
			.as_str()
			.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "display_name must be a string"))?
			.to_owned(),
		description: if settings["pack"]["description"].is_null() {
				String::new()
			} else {
				settings["pack"]["description"]
					.as_str()
					.ok_or_else(|| {
						println!("{}", "[ERROR] Description must be a string.".red());
						io::Error::new(io::ErrorKind::InvalidData, "description must be a string")})?
					.to_owned()
			},
		authors: settings["modinator"]["authors"]
			.as_array()
			.ok_or_else(|| {
						println!("{}", "[ERROR] PACK.MCMETA: \"authors\" must be an array:\n	 like: [\"author1\", \"author2\", etc...]".red());
						io::Error::new(io::ErrorKind::InvalidData, "authors must be an array")
					})?
			.iter()
			.map(|author| {
				author
					.as_str()
					.map(String::from)
					.ok_or_else(|| {
						println!("{}", "[ERROR] PACK.MCMETA: Each author in \"authors\" must be a string:\n	 like: [\"author1\", \"author2\", etc...]".red());
						io::Error::new(io::ErrorKind::InvalidData, "each author must be a string")
					})
			})
			.collect::<Result<Vec<_>, _>>()?,
		license: settings["modinator"]["license"]
			.as_str()
			.ok_or_else(|| {
				println!("[ERROR] License must be a string.");
				io::Error::new(io::ErrorKind::InvalidData, "license must be a string")
			})?
			.to_owned(),
		forge_version: if !settings["modinator"]["forge_version"].is_null() {
			match &settings["modinator"]["forge_version"] {
				serde_json::Value::String(value) => value.clone(),
				serde_json::Value::Number(value) if value.is_i64() || value.is_u64() => value.to_string(),
				_ => {
					println!("{}", "[ERROR] \"forge_version\" must be a string or integer".red());
					return Err(std::io::Error::new(
						std::io::ErrorKind::InvalidData,
						"forge_version must be a string or integer",
					).into());
				}
			}
		} else {
			println!("{}", "[WARN] PACK.MCMETA: No \"forge_version\" specified. Considering \"\".".yellow());
			String::new()
		},
		neoforge_version: if !settings["modinator"]["neoforge_version"].is_null() {
			match &settings["modinator"]["neoforge_version"] {
				serde_json::Value::String(value) => value.clone(),
				serde_json::Value::Number(value) if value.is_i64() || value.is_u64() => value.to_string(),
				_ => {
					println!("{}", "[ERROR] \"neoforge_version\" must be a string or integer".red());
					return Err(std::io::Error::new(
						std::io::ErrorKind::InvalidData,
						"neoforge_version must be a string or integer",
					).into());
				}
			}
		} else {
			println!("{}", "[WARN] PACK.MCMETA: No \"neoforge_version\" specified. Considering \"\".".yellow());
			String::new()
		},
		icon: PathBuf::from(
			if settings["paths"]["icon"].is_null() {""} else {
				settings["paths"]["icon"]
					.as_str()
					.ok_or_else(|| {
						println!("{}", "[ERROR] PACK.MCMETA[\"paths\"][\"icon\"] must be a string.".red());
						io::Error::new(
							io::ErrorKind::InvalidData,
							"pack.mcmeta: paths.icon must be a string",
						)
					})?
				}
			),
		paths: settings["paths"].clone(),
		dependency
	})
	/* PathsSettings will return:
		assets = "assets/"
		data = "data/"
		icon = "icon.png"
	*/
}