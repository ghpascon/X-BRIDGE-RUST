use minijinja::Environment;

/// Load all HTML templates embedded at compile time.
pub fn load_templates() -> Result<Environment<'static>, Box<dyn std::error::Error + Send + Sync>> {
    let mut env = Environment::new();

    env.add_template_owned(
        "base.html",
        include_str!("../../templates/base.html").to_owned(),
    )?;

    // includes
    env.add_template_owned(
        "includes/header.html",
        include_str!("../../templates/includes/header.html").to_owned(),
    )?;
    env.add_template_owned(
        "includes/footer.html",
        include_str!("../../templates/includes/footer.html").to_owned(),
    )?;
    env.add_template_owned(
        "includes/alerts.html",
        include_str!("../../templates/includes/alerts.html").to_owned(),
    )?;

    // pages
    env.add_template_owned(
        "/index/main.html",
        include_str!("../../templates/pages/index/main.html").to_owned(),
    )?;

    Ok(env)
}
