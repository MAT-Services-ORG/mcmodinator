use colored::Colorize;

pub fn doc(topic: &str) {
    println!("MAT Modinator
Create a mod using DATA, ASSETS and KubeJS standrad folders.\n");
    if topic.to_ascii_uppercase()=="MAIN" {
        println!("{} [OPTIONS]", format!("{} modinator", "Usage:".underline()).bold());
        println!("{} [TOPIC]\n", "       modinator --doc".bold());
        println!("For CLI usage help, try '{}'.\n", "--help".bold());
    } else {
        println!("Topic does not exists.\nTry '{}' to see topics.", "modinator --doc".bold())
    } // TODO: Add topics.
}