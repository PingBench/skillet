use colored_json::{ColorMode, ToColoredJson};
use dotenvy::from_path_iter;
use indexmap::IndexMap;
use inquire::MultiSelect;
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::path::Path;

/// Load variables from a `.env` file, preserving the on‑disk order.
/// If `quiet` is false, pretty‑print the map as JSON.
pub fn parse_env_file(quiet: bool) -> IndexMap<String, String> {
    let mut map = IndexMap::<String, String>::new();
    if let Ok(iter) = from_path_iter(Path::new(".env")) {
        for (key, value) in iter.flatten() {
            map.insert(key, value);
        }
    }

    if !quiet {
        println!(".env config:");
        let json = serde_json::to_string_pretty(&map).unwrap();
        println!("{}", json.to_colored_json(ColorMode::default()).unwrap());
    }

    map
}

/// Collect all shell variables prefixed with `TFAID`.
/// When `quiet` is false, pretty‑print the discovered map.
pub fn parse_tfaid_vars(quiet: bool) -> HashMap<String, String> {
    let result: HashMap<String, String> = env::vars()
        .filter(|(k, _)| k.starts_with("TFAID"))
        .collect();

    if !quiet {
        println!("shell config:");
        let json = serde_json::to_string_pretty(&result).unwrap();
        println!("{}", json.to_colored_json(ColorMode::default()).unwrap());
    }

    result
}

#[derive(Debug, Serialize)]
pub struct Namespaced {
    pub profiles: IndexMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct Reconciled {
    pub env: Namespaced,
    pub shell: Namespaced,
}

/// Merge `.env` vs shell configs following precedence rules.
/// - `.env` wins over shell (case‑insensitive)
/// - keys are case‑insensitive when deduping / sorting
pub fn reconcile_configs(
    env_cfg: &IndexMap<String, String>,
    shell_cfg: &HashMap<String, String>,
    quiet: bool,
) -> Reconciled {
    let env_norm: HashMap<String, (&String, &String)> = env_cfg
        .iter()
        .map(|(k, v)| (k.to_lowercase(), (k, v)))
        .collect();

    let mut env_pairs: Vec<(&String, &String)> = env_cfg.iter().collect();
    env_pairs.sort_by_key(|(k, _)| k.to_lowercase());

    let env_profiles: IndexMap<String, String> = env_pairs
        .into_iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    let mut shell_pairs: Vec<(&String, &String)> = shell_cfg
        .iter()
        .filter(|(k, _)| !env_norm.contains_key(&k.to_lowercase()))
        .collect();

    shell_pairs.sort_by_key(|(k, _)| k.to_lowercase());

    let shell_profiles: IndexMap<String, String> = shell_pairs
        .into_iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    let reconciled = Reconciled {
        env: Namespaced {
            profiles: env_profiles,
        },
        shell: Namespaced {
            profiles: shell_profiles,
        },
    };

    if !quiet {
        println!("reconciled config:");
        let json = serde_json::to_string_pretty(&reconciled).unwrap();
        println!("{}", json.to_colored_json(ColorMode::default()).unwrap());
    }

    reconciled
}

/// Prompt the user with a multi‑select of `env` + `shell` profile values.
/// Returns entries formatted as `"env.<value>"` or `"shell.<value>"`.
pub fn user_select_profiles(
    reconciled: &Reconciled,
    message: &str,
    instruction: &str,
) -> Vec<String> {
    let mut items: Vec<String> = Vec::new();
    let mut defaults: Vec<usize> = Vec::new();

    for (section, profiles) in [
        ("env", &reconciled.env.profiles),
        ("shell", &reconciled.shell.profiles),
    ] {
        for (_, value) in profiles {
            let entry = format!("{section}.{value}");
            if section == "env" {
                defaults.push(items.len());
            }
            items.push(entry);
        }
    }

    if items.is_empty() {
        return Vec::new();
    }

    let mut prompt = MultiSelect::new(message, items);
    prompt = prompt.with_help_message(instruction);
    prompt = prompt.with_default(&defaults);

    prompt.prompt().unwrap_or_default()
}

/// Convenience struct holding all bootstrapped data.
#[allow(dead_code)]
#[derive(Debug)]
pub struct BootInfo {
    pub env_cfg: IndexMap<String, String>,
    pub shell_cfg: HashMap<String, String>,
    pub reconciled: Reconciled,
    pub profiles: Vec<String>,
}

/// Run the full config bootstrap sequence. You can call this in any sub‑command.
/// Returns a `BootInfo` bundle you can stash in your context or use directly.
pub fn bootstrap(quiet: bool) -> BootInfo {
    let env_cfg = parse_env_file(quiet);
    let shell_cfg = parse_tfaid_vars(quiet);
    let reconciled = reconcile_configs(&env_cfg, &shell_cfg, quiet);

    let profiles = user_select_profiles(
        &reconciled,
        "Select profiles for providers:",
        "Use SPACE to toggle",
    );

    // TODO: call aws::authenticate_profiles(&profiles, quiet);
    // TODO: terraform_auth::add_provider_blocks(...);

    BootInfo {
        env_cfg,
        shell_cfg,
        reconciled,
        profiles,
    }
}
