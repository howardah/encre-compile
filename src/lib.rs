use encre_css::Config;

pub fn add_encre(classes: Vec<String>, full: bool) -> String {
    let classes: String = classes
        .iter()
        .map(|s| s.to_string())
        .reduce(|acc, item| format!("{} {}", acc, item))
        .unwrap_or_default();

    let config = Config::default();
    let generated_config = encre_css::generate([r#"<div class=""></div>"#], &config);
    let generated = encre_css::generate(
        [format!("<div class=\"{}\"></div>", classes).as_str()],
        &config,
    );

    match full {
        true => {
            format!("{}", generated.trim())
        }
        false => {
            let diff = generated.replace(generated_config.as_str(), "");
            format!("{}", diff.trim())
        }
    }
}
