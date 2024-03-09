mod docker_api {
    use crate::ResponseApi;
    pub async fn get_last_tags(name: &String) ->  Result<ResponseApi, reqwest::Error> {
        let registry_admin = models::RegistryProfile::new();
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

    pub(crate) mod models {
        use crate::config_image::ImageVersion;

        #[derive(Debug)]
        pub struct RegistryProfile {
            pub username: String,
            pub password: String
        }

        impl RegistryProfile {
            pub fn new() -> RegistryProfile {
                let username = std::env::var("REGISTRY_USER").unwrap_or(String::from("NA"));
                let password = std::env::var("REGISTRY_PASS").unwrap_or(String::from("NA"));
                RegistryProfile {
                    username,
                    password,
                }
            }
        }

        #[derive(Debug)]
        pub struct TagsResponse {
            pub(crate) name: String,
            pub(crate) tag: ImageVersion
        }
    }
}

use docker_api::models::TagsResponse;
use docker_api::get_last_tags;
use crate::config_image::ImageVersion;

pub async fn get_tags(name: String) -> Result<TagsResponse, reqwest::Error> {
    let res = get_last_tags(&name).await?;
    let latest_tag = ImageVersion::new(res.tags.last().unwrap());
    match latest_tag {
        Ok(t) => {
            Ok(TagsResponse { name, tag: t })
        },
        Err(e) => panic!("Error while parsing version {:?}", e)
    }
}