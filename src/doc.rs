use colored::Colorize;

pub fn doc(topic: &str) {
	println!("MAT Modinator Doccumentation\nCreate a mod using DATA, ASSETS and KubeJS standrad folders.\n");
	match topic.to_ascii_uppercase().as_str() {
		"MAIN"=>{
			println!("{} [OPTIONS]", format!("{} modinator", "Usage:".underline()).bold());
			println!("{} [TOPIC]", "	   modinator --doc".bold());
			println!("{}\n", "	   modinator --doc LIST".bold());
			println!("For CLI usage help, try '{}'.\n", "--help".bold());
		},
		"LIST"=>{
		print!("{}", "List of topics:".underline().bold());
		print!("
  MAIN - Default topic
  LIST - Shows a list of all topics
					")
				}
		_ => println!("Topic does not exists.\nTry '{}' to see topics.", "modinator --doc LIST".bold())
	} // TODO: Add topics.
}