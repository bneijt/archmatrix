use simple_error::SimpleError;

use octocrab;

fn latest_version_from(versions: &Vec<String>) -> Option<String> {
    let mut semivers: Vec<Vec<u32>> = versions
        .into_iter()
        .map(|version| {
            version
                .split('.')
                .map(|s| s.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    semivers.sort();
    semivers.last().map(|version_vec| {
        version_vec
            .iter()
            .map(|u| u.to_string())
            .collect::<Vec<String>>()
            .join(".")
    })
}

pub async fn load_latest_with_prefix(prefix: &String) -> Result<String, Box<SimpleError>> {
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
    let mut versions: Vec<Vec<String>> = content_items
        .items
        .iter()
        .map(|content| content.name.clone())
        .filter(|version| version.starts_with(prefix))
        .filter(|version| !version.contains('-'))
        .map(|version| version.split('.').map(String::from).collect())
        .collect();
    // println!("{:?}", content_items.items);
    println!("{:?}", versions);
    versions.sort(); //|a, b| a.split(".").map(|z|z.parse::<i32>().unwrap()) );
    let version: String = versions
        .get(0)
        .map(|version_vec| version_vec.join("."))
        .ok_or_else(|| {
            Box::new(SimpleError::new(format!(
                "Failed to find version for {prefix}"
            )))
        })?
        .clone();
    Ok(version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latest_version_from_should_know_numbers() {
        assert_eq!(
            Some(String::from("1.33.0")),
            latest_version_from(&vec![String::from("1.30.0"), String::from("1.33.0"), String::from("1.20.0")])
        );
    }
}
