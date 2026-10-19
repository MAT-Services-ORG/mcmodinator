use colored::Colorize;

pub fn doc(topic: &str) {
	let t = topic.to_ascii_uppercase();
	println!("MAT Modinator Doccumentation
Create a mod using DATA, ASSETS and KubeJS standrad folders.\n");
	if t=="MAIN" {
		println!("{} [OPTIONS]", format!("{} modinator", "Usage:".underline()).bold());
		println!("{} [TOPIC]", "       modinator --doc".bold());
		println!("{}\n", "       modinator --doc LIST".bold());
		println!("For CLI usage help, try '{}'.\n", "--help".bold());
	} 
	else if t=="LIST" {
		print!("{}", "List of topics:".underline().bold());
		print!("
  MAIN - Default topic
  LIST - Shows a list of all topics
		")
	}
	else {
		println!("Topic does not exists.\nTry '{}' to see topics.", "modinator --doc LIST".bold())
	} // TODO: Add topics.
}