use crate::*;

pub fn doc(topic: &str) {
	println!("MAT Modinator Doccumentation\nCreate a mod using DATA, ASSETS and KubeJS standrad folders.\n");
	// Topics are in JSON.
	let topics = json!({
		"MAIN": [
			"Default topic",
			format!("{} [OPTIONS]\n{} [TOPIC]\n{}\n\nFor CLI usage help, try '{}'.\n", 
				format!("{} modinator", "Usage:".underline()).bold(),
				"	   modinator --doc".bold(),
				"	   modinator --doc LIST".bold(),
				"--help".bold()
			)
		],
		"LIST": [
			"Shows a list of all topics",
			format!("{}", "List of topics:".underline().bold())
		]
	}); // TODO: Add topics.

	if topics[topic].is_null() {
		println!("Topic does not exists.\nTry '{}' to see topics.", "modinator --doc LIST".bold())
	} else {
		println!("{}", topics[topic][1].as_str().unwrap_or(""));
		if topic=="LIST" {
			for (name, content) in topics.as_object().unwrap() {
				println!("  {name} — {}", content[0].as_str().unwrap_or(""));
			}
		};
	}
}