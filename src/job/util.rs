use std::collections::HashMap;

// use anyhow::anyhow;

pub fn ensure_project(
    project: &String,
    token: &String,
    store: &HashMap<String, String>,
) -> anyhow::Result<()> {
    if store.get(project).is_some_and(|t| t.eq(token)) {
        return Ok(());
    }
    Ok(())
    // Err(anyhow!("Invalid token"))
}
