use dirs::home_dir;
use ini::Ini;
use std::path::PathBuf;

/// Read the AWS CLI config file (`~/.aws/config`) and return the region
/// configured for the given profile name.
///
/// AWS config sections look like:
///   [default]
///   region = us-west-2
///
///   [profile my-profile]
///   region = us-east-1
///
/// # Arguments
/// * `profile_name` - Name of the AWS CLI profile (`"default"` or `"my-profile"`)
///
/// # Returns
/// * `Some(region)` if configured, otherwise `None`
pub fn get_profile_region(profile_name: &str) -> Option<String> {
    let config_path: PathBuf = home_dir()?.join(".aws").join("config");
    if !config_path.exists() {
        return None;
    }

    let ini = Ini::load_from_file(config_path).ok()?;

    // Candidate sections: [default] and [profile my-profile]
    let sections = if profile_name == "default" {
        vec!["default".to_string()]
    } else {
        vec![
            format!("profile {}", profile_name),
            profile_name.to_string(),
        ]
    };

    for section in sections {
        if let Some(props) = ini.section(Some(section.as_str())) {
            if let Some(region) = props.get("region") {
                return Some(region.to_string());
            }
        }
    }

    None
}
