use std::{env, path::PathBuf};

struct App {
    title: String,
}

impl App {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let Some(path) = args.get(1) else {
        return Err("You need to enter the name of the Lua file.".to_string());
    };

    let path_buf = PathBuf::from(path);

    let app = App::new(path);
    Ok(())
}
