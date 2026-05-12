use minijinja::Environment;

/// Load all HTML templates embedded at compile time.
pub fn load_templates() -> Result<Environment<'static>, Box<dyn std::error::Error + Send + Sync>> {
    let mut env = Environment::new();

    env.add_template_owned(
        "base.html",
        include_str!("../../templates/base.html").to_owned(),
    )?;
    env.add_template_owned(
        "home.html",
        include_str!("../../templates/home.html").to_owned(),
    )?;
    env.add_template_owned(
        "includes/header.html",
        include_str!("../../templates/includes/header.html").to_owned(),
    )?;
    env.add_template_owned(
        "includes/footer.html",
        include_str!("../../templates/includes/footer.html").to_owned(),
    )?;

    Ok(env)
}
