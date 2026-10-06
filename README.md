# MAT Modinator
## Description
The Modinator is a tool created to create Minecraft Mods using data and ressource packs, alied with [KubeLoader](https://github.com/WhiseNT/kubeloader) technologies.
It's a CLI tool that uses common mods architectures to create "universal" mods.
The original project was developed and archived by Tornato, at the name of [DatapackModinator](https://github.com/T0RNATO/datapackmodinator), originally writen in Python. This project is actually written in Rust and are in actual development on some new features, improvements and compatibility avancements.

Due to the fact than this project stills in development, We advise you to download the [original project releases](https://github.com/T0RNATO/datapackmodinator/releases/latest) to have a working modinator while this one is in development. 


## Configure
> For informations about the compatibility mode, check the [DatapackModinator](https://github.com/T0RNATO/datapackmodinator) project.

> [!WARNING]
> The new version is WIP, please, use the compatibility mode for now.

On the new version, configuring the mod is directly via the pack.json file: 
> [!IMPORTANT]
> Comments aren't supported, please, remove them on the last version.
```json
{ // WARNING: The comments will be considered as errors by serde_json. Do not use them in the final pack.mcmeta.
	"pack": {
		"pack_format": 121,
		"description": "My custom pack",
		"supported_formats": {
			"min_inclusive": 4,
			"max_inclusive": 121
		}
	},
	"modinator": {
		"id": "mymod", // Will also be the name of the output file.
		"display_name": "My Mod",
		"mod_version": "1.0.0",
		"authors": ["Myself"],
		"license": "MIT",
		"forge_version": "47"
	},
	"paths": { // Values can be set to None for none.
		"icon": "./icon.png",
		"assets": "./assets/",
		"data": "./data/",
		"kubeloader": { // Will require contentpacks.json.
			"server-js": "./server_scripts/",
			"client-js": "./client_scripts/",
			"startup-js": "./startup_scripts/"
		}
	},
	"output": {
		"mod": 1, // Set to 1 to create a mod named "{modinator.id}-mod.jar". Set to a string for a custom filename.
		"contentpack": 0 // Set to 1 to create a mod named "{modinator.id}-cp.zip". Set to a string for a custom filename.
	}
}
``` 
