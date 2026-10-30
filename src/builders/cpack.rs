use crate::*;

pub fn build(mod_info: &mut ModInfo, pack_build: &str, output_path: &String) -> Result<(), Box<dyn std::error::Error>> {
	println!("{}", "[OUCH] Sorry, KUBELOADER ContentPacks support isn't aviable yet. Check in newer versions: https://github.com/MAT-Services-ORG/mcmodinator/releases".bright_red());

	println!("[INFO] Building KL ContentPack..."); // Create ZIP/JAR
	let packfilename = match pack_build {
		"true" | "t" | "yes" | "y" => format!("{output_path}/{}-cpack.jar", mod_info.id),
		_ => format!("{output_path}/{pack_build}")
	};
	let mut zip = ZipWriter::new(
		File::create(&packfilename).map_err(|error| {
			println!("{}", format!("[ERROR] {}", error).red());
			io::Error::new(error.kind(), error)
		})?
	);

	
	import_folder(mod_info.paths.get("data"), &mut zip, "Datapack Files", "data")?; // Data
	import_folder(mod_info.paths.get("assets"), &mut zip, "Resourcepack Files", "assets")?; // Assets

	if !mod_info.paths["kubeloader"].is_null() { 
		println!("{}","[CAUT] CPACK: Loading KubeLoader integrations at your risk.".bright_red());
		import_folder(mod_info.paths["kubeloader"].get("server-js"), &mut zip, "Server Scripts", "server_scripts")?; // server-js
		import_folder(mod_info.paths["kubeloader"].get("client-js"), &mut zip, "Client Scripts", "client_scripts")?; // client-js
		import_folder(mod_info.paths["kubeloader"].get("startup-js"), &mut zip, "Startup Scripts", "startup_scripts")?; // startup-js
		// KUBELOADER metadatas. (Check https://github.com/WhiseNT/kubeloader_wiki-en/wiki/Content-Packs )

		let mut dependancies = json!([]);
		let mut v=0;
		for dependancy in mod_info.dependency.as_object().unwrap_or(
			json!({}).as_object().unwrap()
		).keys() {
			dependancies[v] = json!({ // TOFIX: PANICK HERE !!!
				"type": mod_info.dependency[dependancy]["forge"]["type"],
				"id": dependancy,
				"versionRange": mod_info.dependency[dependancy]["forge"]["versionRange"]

			});
			v+=1
		};

		zip.start_file("contentpacks.json", SimpleFileOptions::default())?;
		zip.write_all(
			serde_json::to_string(&json!({
				"id": mod_info.id,
				"version": mod_info.version,
				"name": mod_info.display_name,
				"description": mod_info.description,
				"authors": mod_info.authors,
				"license": mod_info.license,
				"dependencies": dependancies
			}))?
			.as_bytes()
		)?;
		mod_info.dependency["kubeloader"]["forge"]["versionRange"] = "".into(); 
	};

	match zip.finish() {
		Ok(_) => {println!("{}", format!("[DONE] Mod created at {packfilename}").green())},
		Err(error) => {println!("{}", format!("[ERROR] {error}").red())}
	};

	Ok(()) // TODO: Just... Do it. (Check https://github.com/WhiseNT/kubeloader_wiki-en/wiki/Content-Packs ))
}