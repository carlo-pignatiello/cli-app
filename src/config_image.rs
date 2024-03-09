#[derive(Debug)]
pub struct ImageVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl ImageVersion {
    pub fn new(args: &String) -> Result<ImageVersion, &str> {
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

#[derive(Debug)]
pub struct ConfigImage {
    pub image_name: String,
    pub version: ImageVersion,
}

impl ConfigImage {
    pub fn new(args: &[String]) -> ConfigImage {
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
