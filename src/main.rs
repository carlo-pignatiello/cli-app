use std::env;
use std::fmt::format;
use reqwest::Url;
use serde::Deserialize;

fn main() {
    let args: Vec<String> = env::args().collect();
    let conf = ConfigImage::new(&args);
    println!(
        "You want to process {:?} with version {}.{}.{}",
        conf.image_name, conf.version.major, conf.version.minor, conf.version.patch,
    );
}
#[derive(Debug)]
struct ImageVersion {
    major: u8,
    minor: u8,
    patch: u8,
}
#[derive(Debug)]
struct ConfigImage {
    image_name: String,
    version: ImageVersion,
}

impl ImageVersion {
    fn new(args: &String) -> Result<ImageVersion, &str> {
        if args.chars().count() != 5 {
            return Err("You need to specify your version in this format: major.minor.patch");
        }
        let tags = ImageVersion::parse_tags(args);
        Ok(ImageVersion {
            major: tags[0],
            minor: tags[1],
            patch: tags[2],
        })
    }

    fn parse_tags(args: &String) -> Vec<u8> {
        let tags = args.split(".");
        let tags_as_int = tags
            .filter_map(|s| s.parse::<u8>().ok())
            .collect::<Vec<_>>();
        tags_as_int
    }
}

impl ConfigImage {
    fn new(args: &[String]) -> ConfigImage {
        let image_name = args[1].clone();
        let images_version = args[2].clone();
        let tags = ImageVersion::new(&images_version);
        ConfigImage {
            image_name,
            version: match tags {
                Ok(n) => n,
                Err(err) => panic!("Error: {:?}", err),
            },
        }
    }
}
// This `derive` requires the `serde` dependency.
#[derive(Deserialize)]
struct ImageTagsResponse {
    name: String,
    tags: Vec<String>
}
/*async fn get_last_image_tags(name: &str) -> Result<ImageVersion, &str> {
    let mut uri: String = String::from("http://localhost:5000/v2");
    let path = format!("{}/tags/list", name);
    uri.push_str(&*path);
    println!("{}", uri)
    let body = reqwest::get(uri)
        .await?
        .json::<ImageTagsResponse>()
        .await?;
    let tags_as_str = body.tags.last().unwrap();
    let tags = ImageVersion::new(&tags_as_str);
    tags
}*/