use simple_error::SimpleError;

use octocrab;

pub async fn latest_with_prefix(prefix: &String) -> Result<String, Box<SimpleError>> {
    // TODO determine latest pyenv version online
    // by parsing https://api.github.com/repos/pyenv/pyenv/git/trees/master?recursive=true
    // and extracting the correct versions from https://github.com/pyenv/pyenv/tree/master/plugins/python-build/share/python-build
    // use https://api.github.com/repos/pyenv/pyenv/contents/plugins/python-build/share/python-build
    let content_items = octocrab::instance()
        .repos("pyenv", "pyenv")
        .get_content()
        .path("plugins/python-build/share/python-build")
        .r#ref("master")
        .send()
        .await
        .map_err(|_| Box::new(SimpleError::new(format!("Failed to contact github"))))?;
    let mut versions: Vec<String> = content_items
        .items
        .iter()
        .map(|content| content.name.clone())
        .filter(|version| version.starts_with(prefix))
        .filter(|version| !version.contains('-'))
        .collect();
    // println!("{:?}", content_items.items);
    println!("{:?}", versions);
    versions.sort();//|a, b| a.split(".").map(|z|z.parse::<i32>().unwrap()) );
    let version: String = versions
        .get(0)
        .ok_or_else(|| {
            Box::new(SimpleError::new(format!(
                "Failed to find version for {prefix}"
            )))
        })?
        .clone();
    Ok(version)
}
