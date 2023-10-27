use colored::Colorize;
use macros_rs::ternary;
use question::{Answer, Question};
use vendor::project::init::create_project;

pub fn get_version(short: bool) -> String {
    return match short {
        true => format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
        false => format!("{} ({} {})", env!("CARGO_PKG_VERSION"), env!("GIT_HASH"), env!("BUILD_DATE")),
    };
}

pub fn create_project_yml() {
    let exists: bool = std::path::Path::new("package.yml").exists();
    if !exists {
        create_project();
    } else {
        let answer = Question::new("overwrite project.yml?").show_defaults().confirm();
        ternary!(answer == Answer::YES, create_project(), println!("{}", "Aborting...".white()))
    }
}
