use std::env;
mod config_image;
mod docker_api;

use config_image::ConfigImage;
use config_image::ImageVersion;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();
    let latest_image_conf = docker_api::get_tags(args[1].clone()).await?;
    let user_image_conf = ConfigImage::new(&args);
    println!(
        "You want to push {:?} with version {}.{}.{}",
        args[1],
        user_image_conf.version.major,
        user_image_conf.version.minor,
        user_image_conf.version.patch,
    );
    let registry_gt = registry_image_gte(&user_image_conf.version, &latest_image_conf.tag);
    match registry_gt {
        true => println!(
            "I'm sad but a newer or equal version in on the registry ({:#?})",
            latest_image_conf.tag
        ),
        false => println!(
            "Yes, your image version ({:#?}) is newer!",
            user_image_conf.version
        ),
    }
    Ok(())
}

fn registry_image_gte(input_image: &ImageVersion, registry_image: &ImageVersion) -> bool {
    if registry_image.major > input_image.major {
        return true;
    }
    if registry_image.major == input_image.major && registry_image.minor > input_image.minor {
        return true;
    }
    if registry_image.major == input_image.major
        && registry_image.minor == input_image.minor
        && registry_image.patch > input_image.patch
    {
        return true;
    }
    if registry_image.major == input_image.major
        && registry_image.minor == input_image.minor
        && registry_image.patch == input_image.patch
    {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::config_image::ImageVersion;
    use crate::registry_image_gte;
    #[test]
    fn test_one() {
        let input_string = String::from("1.0.0");
        let registry_string = String::from("1.1.0");
        let input_image = ImageVersion::new(&input_string);
        let registry_image = ImageVersion::new(&registry_string);
        assert!(registry_image_gte(
            &input_image.unwrap(),
            &registry_image.unwrap()
        ));
    }
    #[test]
    fn test_equal_version() {
        let input_string = String::from("1.0.0");
        let registry_string = String::from("1.0.0");
        let input_image = ImageVersion::new(&input_string);
        let registry_image = ImageVersion::new(&registry_string);
        assert!(registry_image_gte(
            &input_image.unwrap(),
            &registry_image.unwrap()
        ));
    }

    #[test]
    fn test_two() {
        let input_string = String::from("1.4.0");
        let registry_string = String::from("1.1.0");
        let input_image = ImageVersion::new(&input_string);
        let registry_image = ImageVersion::new(&registry_string);
        assert!(!registry_image_gte(
            &input_image.unwrap(),
            &registry_image.unwrap()
        ));
    }

    #[test]
    fn test_three() {
        let input_string = String::from("1.1.1");
        let registry_string = String::from("1.1.0");
        let input_image = ImageVersion::new(&input_string);
        let registry_image = ImageVersion::new(&registry_string);
        assert!(!registry_image_gte(
            &input_image.unwrap(),
            &registry_image.unwrap()
        ),);
    }

    #[test]
    fn test_four() {
        let input_string = String::from("1.1.1");
        let registry_string = String::from("1.1.3");
        let input_image = ImageVersion::new(&input_string);
        let registry_image = ImageVersion::new(&registry_string);
        assert!(!registry_image_gte(
            &input_image.unwrap(),
            &registry_image.unwrap()
        ));
    }

    #[test]
    fn test_five() {
        let input_string = String::from("1.2.1");
        let registry_string = String::from("2.1.3");
        let input_image = ImageVersion::new(&input_string);
        let registry_image = ImageVersion::new(&registry_string);
        assert!(registry_image_gte(
            &input_image.unwrap(),
            &registry_image.unwrap()
        ));
    }

    #[test]
    fn test_six() {
        let input_string = String::from("2.1.1");
        let registry_string = String::from("2.1.0");
        let input_image = ImageVersion::new(&input_string);
        let registry_image = ImageVersion::new(&registry_string);
        assert!(registry_image_gte(
            &input_image.unwrap(),
            &registry_image.unwrap()
        ));
    }

    #[test]
    fn test_seven() {
        let input_string = String::from("1.0.0");
        let registry_string = String::from("1.0.1");
        let input_image = ImageVersion::new(&input_string);
        let registry_image = ImageVersion::new(&registry_string);
        assert!(registry_image_gte(
            &input_image.unwrap(),
            &registry_image.unwrap()
        ));
    }

    #[test]
    fn test_eight() {
        let input_string = String::from("1.2.2");
        let registry_string = String::from("1.2.1");
        let input_image = ImageVersion::new(&input_string);
        let registry_image = ImageVersion::new(&registry_string);
        assert!(!registry_image_gte(
            &input_image.unwrap(),
            &registry_image.unwrap()
        ));
    }

    #[test]
    fn test_nine() {
        let input_string = String::from("1.2.3");
        let registry_string = String::from("1.2.1");
        let input_image = ImageVersion::new(&input_string);
        let registry_image = ImageVersion::new(&registry_string);
        assert!(!registry_image_gte(
            &input_image.unwrap(),
            &registry_image.unwrap()
        ));
    }
}
