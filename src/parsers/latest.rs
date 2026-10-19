use crate::*;

pub fn get() -> Result<ModInfo, Box<dyn std::error::Error>> {
	println!("[DEBUG] Parsing project archicture...");
	
	let settings: serde_json::Value = serde_json::from_str(
		&fs::read_to_string("pack.json").map_err(|err| {
			let msg = if err.kind() == io::ErrorKind::NotFound {
				"\"pack.json\" not found in project".to_string()
			} else {
				format!("Could not read \"pack.json\": {err}")
			};
			println!("{}", format!("[ERROR] {}", msg).red());
			io::Error::new(err.kind(), msg)
		})? 
	).map_err(|error| {
		let msg = if error.classify() == serde_json::error::Category::Syntax {
			"Cannot read \"pack.json\". Have you removed the comments?".to_string()
		} else {
			error.to_string()
		};
		println!("{}", format!("[ERROR] {}", msg).red());
		io::Error::new(serde_json::Error::io_error_kind(&error).unwrap_or(io::ErrorKind::InvalidData), msg)
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
			return Err(std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				"Missing required key in settings",
			).into());
		}
	};
	
	if !settings["paths"]["kubeloader"].is_null() { // TODO: KUBELOADER support.
		println!(
			"{}", 
			"[WARN] KUBELOADER is not supported yet. Check in newer versions: https://github.com/MAT-Services-ORG/mcmodinator/releases".yellow()
		)
	} 
	let paths: Table = ["assets", "data", "icon"]
    .into_iter()
    .map(|key| {
        let value = settings["paths"][key].as_str().unwrap_or_default();
        (
            key.to_owned(),
            toml::Value::String(value.to_owned()),
        )
    })
    .collect();
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
					.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "description must be a string"))?
					.to_owned()
			},
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
		license: settings["modinator"]["license"]
			.as_str()
			.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "license must be a string"))?
			.to_owned(),
		forge_version: if !settings["modinator"]["neoforge_version"].is_null() {
		match &settings["modinator"]["forge_version"] {
			serde_json::Value::String(value) => value.clone(),
			serde_json::Value::Number(value) if value.is_i64() || value.is_u64() => value.to_string(),
			_ => {
				println!("{}", format!("[ERROR] \"forge_version\" must be a string or integer").red());
				return Err(std::io::Error::new(
					std::io::ErrorKind::InvalidData,
					"forge_version must be a string or integer",
				).into());
			}
		}} else {
			println!("{}", "[WARN] No \"forge_version\" specified. Considering \"\".");
			String::new()
		},
		neoforge_version: if !settings["modinator"]["neoforge_version"].is_null() {
			match &settings["modinator"]["neoforge_version"] {
				serde_json::Value::String(value) => value.clone(),
				serde_json::Value::Number(value) if value.is_i64() || value.is_u64() => value.to_string(),
				_ => {
					println!("{}", format!("[ERROR] \"neoforge_version\" must be a string or integer").red());
					return Err(std::io::Error::new(
						std::io::ErrorKind::InvalidData,
						"neoforge_version must be a string or integer",
					).into());
				}
			}
		} else {
			println!("{}", "[WARN] No \"neoforge_version\" specified. Considering \"\".");
			String::new()
		},
		icon: PathBuf::from(settings["paths"]["icon"]
			.as_str()
			.ok_or_else(|| {
				println!("{}", format!("[ERROR] pack.json[\"paths\"][\"icon\"] must be a string.").red());
				return io::Error::new(
					io::ErrorKind::InvalidData,
					"pack.json: paths.icon must be a string",
				);
			})?),
		paths
	})
	/* PathsSettings will return:
		assets = "assets/"
		data = "data/"
		icon = "icon.png"
	*/
}