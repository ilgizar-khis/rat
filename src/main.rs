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

fn main() {
    let app = App::new("hello world");
}
