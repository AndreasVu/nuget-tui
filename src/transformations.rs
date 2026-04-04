use crate::app::App;

impl App {
    pub fn get_package_names(&self) -> Vec<String> {
        self.packages
            .iter()
            .map(|p| p.title.clone())
            .collect::<Vec<_>>()
    }
}
