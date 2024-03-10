use crate::config_image::ImageVersion;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseApi {
    name: String,
    tags: Vec<String>,
}

pub async fn request_tags(name: &String) -> Result<ResponseApi, reqwest::Error> {
    let registry_admin = RegistryProfile::new();
    let url = format!("http://localhost:5000/v2/{}/tags/list", name);
    let tags = reqwest::Client::new()
        .get(url)
        .basic_auth(registry_admin.username, Some(registry_admin.password))
        .send()
        .await?;
    if tags.status() != 200 {
        panic!("Maybe you've entered a wrong image name")
    }
    let response = tags.json().await?;
    Ok(response)
}

#[derive(Debug)]
pub struct RegistryProfile {
    pub username: String,
    pub password: String,
}

impl RegistryProfile {
    pub fn new() -> RegistryProfile {
        let username = std::env::var("REGISTRY_USER").unwrap_or(String::from("NA"));
        let password = std::env::var("REGISTRY_PASS").unwrap_or(String::from("NA"));
        RegistryProfile { username, password }
    }
}

#[derive(Debug)]
pub struct TagsResponse {
    pub name: String,
    pub tag: ImageVersion,
}

impl TagsResponse {
    fn new(name: String, tag: ImageVersion) -> Self {
        Self { name, tag }
    }
}

pub async fn get_tags(name: String) -> Result<TagsResponse, reqwest::Error> {
    let res = request_tags(&name).await?;
    let latest_tag = ImageVersion::new(res.tags.last().unwrap());
    match latest_tag {
        Ok(t) => Ok(TagsResponse::new(name, t)),
        Err(e) => panic!("Error while parsing version {:?}", e),
    }
}
