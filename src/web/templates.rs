use std::{fs, path::Path};

use minijinja::Environment;

type TemplateResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn load_templates(root: &Path) -> TemplateResult<Environment<'static>> {
    let mut environment = Environment::new();
    add_templates_recursively(&mut environment, root, root)?;
    Ok(environment)
}

fn add_templates_recursively(
    environment: &mut Environment<'static>,
    root: &Path,
    current: &Path,
) -> TemplateResult<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            add_templates_recursively(environment, root, &path)?;
            continue;
        }

        if path.extension().and_then(std::ffi::OsStr::to_str) != Some("html") {
            continue;
        }

        let relative = path
            .strip_prefix(root)?
            .to_string_lossy()
            .replace('\\', "/");
        let source = fs::read_to_string(path)?;
        environment.add_template_owned(relative, source)?;
    }

    Ok(())
}
