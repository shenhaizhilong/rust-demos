use std::collections::HashMap;
use serde_json::{Error, json, Value};
use serde::Deserialize;

// https://rust-lang-nursery.github.io/rust-cookbook/encoding/complex.html

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

#[derive(Deserialize)]
struct Config {
    package: Package,
    dependencies: HashMap<String, String>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
}

fn main() -> Result<(), Error> {
    let json = r#"{
        "userid": 10,
        "verified": true,
        "access_privileges": ["user", "admin"]
    }"#;

    let parsed: Value = serde_json::from_str(json)?;
    let expected = json!({
        "userid": 10,
        "verified": true,
        "access_privileges": ["user", "admin"]
    });

    assert_eq!(parsed, expected);

    // The type of `j` is `&str`
    let j = "
        {
            \"fingerprint\": \"0xF9BA143B95FF6D82\",
            \"location\": \"Menlo Park, CA\"
        }";

    let u: User = serde_json::from_str(j).unwrap();
    println!("{:#?}", u);
    Ok(())
}

#[cfg(test)]
mod test {
    use toml::de::Error;
    use toml::Value;
    use crate::Config;

    #[test]
    fn test_toml() {
        let toml_content = r#"
          [package]
          name = "your_package"
          version = "0.1.0"
          authors = ["You! <you@example.org>"]

          [dependencies]
          serde = "1.0"
          "#;
        let package_info: Value = toml::from_str(toml_content).unwrap();
        assert_eq!(package_info["dependencies"]["serde"].as_str(), Some("1.0"));
        assert_eq!(package_info["package"]["name"].as_str(),
                   Some("your_package"));
    }

    #[test]
    fn test2() {
        let toml_content = r#"
          [package]
          name = "your_package"
          version = "0.1.0"
          authors = ["You! <you@example.org>"]

          [dependencies]
          serde = "1.0"
          "#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.package.name, "your_package");
        assert_eq!(config.package.version, "0.1.0");
        assert_eq!(config.package.authors, vec!["You! <you@example.org>"]);
        assert_eq!(config.dependencies["serde"], "1.0");
    }
}
