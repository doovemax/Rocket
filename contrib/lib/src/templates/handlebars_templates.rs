use std::path::Path;

use serde::Serialize;
use crate::templates::Engine;

pub use crate::templates::handlebars::Handlebars;

impl Engine for Handlebars<'static> {
    const EXT: &'static str = "hbs";

    fn init<'a>(templates: impl Iterator<Item = (&'a str, &'a Path)>) -> Option<Self> {
        let mut hb = Handlebars::new();
        let mut ok = true;
        for (name, path) in templates {
            if let Err(e) = hb.register_template_file(name, path) {
                error!("Handlebars template '{}' failed to register.", name);
                error_!("{}", e);
                info_!("Template path: '{}'.", path.to_string_lossy());
                ok = false;
            }
        }

        ok.then(|| hb)
    }

    fn render<C: Serialize>(&self, name: &str, context: C) -> Option<String> {
        if self.get_template(name).is_none() {
            error_!("Handlebars template '{}' does not exist.", name);
            return None;
        }

        match Handlebars::render(self, name, &context) {
            Ok(string) => Some(string),
            Err(e) => {
                error_!("Error rendering Handlebars template '{}': {}", name, e);
                None
            }
        }
    }
}
