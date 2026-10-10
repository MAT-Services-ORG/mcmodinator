use crate::*;

/*pub fn get() -> Result<ModInfo, Box<dyn std::error::Error>>{
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
}// */