use builder_pattern::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: std::option::Option<String>,
}

fn main() {
    let builder = Command::builder();
    let _ = builder;
}
