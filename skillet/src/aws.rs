use aws_config::{BehaviorVersion, defaults, profile::ProfileFileCredentialsProvider};
use aws_sdk_sts::Client as StsClient;
use std::collections::HashMap;
use std::process::Command;

/// Check whether an AWS CLI SSO profile still has a valid token by calling STS GetCallerIdentity.
///
/// Returns `true` if token is valid, `false` if invalid/expired.
pub async fn is_sso_profile_token_valid(profile_name: &str) -> bool {
    let provider = ProfileFileCredentialsProvider::builder()
        .profile_name(profile_name)
        .build();

    let config = defaults(BehaviorVersion::latest())
        .credentials_provider(provider)
        .load()
        .await;

    let client = StsClient::new(&config);

    match client.get_caller_identity().send().await {
        Ok(_) => true,
        Err(err) => {
            eprintln!("GetCallerIdentity failed: {:?}", err);
            false
        }
    }
}

/// Batch-check validity of profiles.
/// Returns map: profile string → true|false
pub async fn check_profiles_token_validity(profiles: &[String]) -> HashMap<String, bool> {
    let mut map = HashMap::new();
    for entry in profiles {
        let profile_name = entry
            .split_once('.')
            .map(|(_, p)| p)
            .unwrap_or(entry.as_str());
        let valid = is_sso_profile_token_valid(profile_name).await;
        map.insert(entry.clone(), valid);
    }
    map
}

/// Run `aws sso login` for a single profile.
pub fn sso_login_profile(profile_name: &str) -> bool {
    let status = Command::new("aws")
        .args(["sso", "login", "--profile", profile_name])
        .status();

    matches!(status, Ok(s) if s.success())
}

/// For any invalid profiles, attempt login and re-check validity.
pub async fn login_profiles(profiles: &[String]) -> HashMap<String, bool> {
    let mut validity = check_profiles_token_validity(profiles).await;
    let mut results = HashMap::new();

    for entry in profiles {
        if *validity.get(entry).unwrap_or(&false) {
            continue; // already valid
        }

        let profile_name = entry
            .split_once('.')
            .map(|(_, p)| p)
            .unwrap_or(entry.as_str());

        let login_ok = sso_login_profile(profile_name);

        if !login_ok {
            results.insert(entry.clone(), false);
            continue;
        }

        // re-check all after login
        validity = check_profiles_token_validity(profiles).await;
        results.insert(entry.clone(), *validity.get(entry).unwrap_or(&false));
    }

    results
}

/// Entry point to check and refresh profiles, printing summary unless quiet.
pub async fn authenticate_profiles(profiles: &[String], quiet: bool) -> HashMap<String, bool> {
    let initial = check_profiles_token_validity(profiles).await;

    let to_login: Vec<_> = initial
        .iter()
        .filter_map(|(entry, ok)| if !ok { Some(entry.clone()) } else { None })
        .collect();

    login_profiles(&to_login).await;

    let final_validity = check_profiles_token_validity(profiles).await;

    if !quiet {
        println!("AWS SSO authentication summary:");
        for entry in profiles {
            let ok = final_validity.get(entry).copied().unwrap_or(false);
            let status = if ok { "✅ valid" } else { "❌ invalid" };
            let note = if to_login.contains(entry) {
                if ok {
                    " (login succeeded)"
                } else {
                    " (login failed)"
                }
            } else {
                ""
            };
            println!("  • {}: {}{}", entry, status, note);
        }
    }

    final_validity
}
