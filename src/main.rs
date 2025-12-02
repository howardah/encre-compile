use clap::{Arg, ArgAction, Command};
use encre_compile::add_encre;

fn main() {
    let matches = Command::new("encre-compile")
        .about("Compiles simple class string into CSS")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("full")
                .short('f')
                .long("full")
                .help("Build full CSS, including basic configuration.")
                .action(ArgAction::SetTrue),
        )
        .trailing_var_arg(true)
        .arg(
            Arg::new("CLASSES")
                .help("Specify CSS classes to compile.")
                .num_args(1..),
        )
        .get_matches();

    let classes: String = matches
        .get_many::<String>("CLASSES")
        .unwrap_or_default()
        .map(|s| s.to_string())
        .reduce(|acc, item| format!("{} {}", acc, item))
        .unwrap_or_default();

    let full = matches.get_flag("full");
    println!("{}", add_encre(vec![classes], full));
}
