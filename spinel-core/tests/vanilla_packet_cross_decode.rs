#[path = "support/packet_fixtures.rs"]
mod packet_fixtures;

use packet_fixtures::PacketFixtureCatalog;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn vanilla_decodes_every_generated_spinel_packet_fixture() {
    let extractor_directory = extractor_directory();
    let fixture_path = env::temp_dir().join("spinel-packet-fixtures.json");
    let fixture_catalog = PacketFixtureCatalog::generate();
    let generation_failures = fixture_catalog.generation_failures();

    assert!(
        generation_failures.is_empty(),
        "Spinel could not generate fixtures for: {}",
        generation_failures.join("\n")
    );

    fixture_catalog.write(&fixture_path).unwrap();
    let gradle_wrapper = extractor_directory.join("gradlew.bat");
    let mut gradle_command = Command::new(gradle_wrapper);
    if let Some(java_home) = gradle_java_home() {
        gradle_command.env("JAVA_HOME", java_home);
    }
    let verification_output = gradle_command
        .arg("packetVerificationTest")
        .arg("--console=plain")
        .arg(format!(
            "-PspinelPacketFixtures={}",
            fixture_path.to_string_lossy()
        ))
        .current_dir(&extractor_directory)
        .output()
        .unwrap();

    if !verification_output.status.success() {
        let verifier_output = format!(
            "{}\n{}",
            String::from_utf8_lossy(&verification_output.stdout),
            String::from_utf8_lossy(&verification_output.stderr)
        );
        let concise_failure = verifier_output
            .lines()
            .skip_while(|line| {
                !line.contains("Vanilla rejected Spinel packet fixtures:")
                    && !line
                        .contains("Vanilla could not verify packets without dynamic registries:")
            })
            .take_while(|line| !line.trim_start().starts_with("at "))
            .map(str::trim_start)
            .map(|line| {
                line.strip_prefix("org.opentest4j.AssertionFailedError: ")
                    .unwrap_or(line)
            })
            .collect::<Vec<_>>()
            .join("\n");
        panic!(
            "{}",
            if concise_failure.is_empty() {
                verifier_output
            } else {
                concise_failure
            }
        );
    }
}

fn gradle_java_home() -> Option<PathBuf> {
    if let Some(configured_java_home) = env::var_os("SPINEL_JAVA_HOME") {
        return Some(PathBuf::from(configured_java_home));
    }
    if !cfg!(windows) {
        return env::var_os("JAVA_HOME").map(PathBuf::from);
    }
    let jetbrains_root = PathBuf::from(env::var_os("ProgramFiles")?).join("JetBrains");
    let mut java_homes = fs::read_dir(jetbrains_root)
        .ok()?
        .flatten()
        .map(|entry| entry.path().join("jbr"))
        .filter(|java_home| java_home.join("bin").join("java.exe").is_file())
        .collect::<Vec<_>>();
    java_homes.sort();
    java_homes.pop()
}

fn extractor_directory() -> PathBuf {
    env_file_value("SPINEL_EXTRACTOR_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| panic!("add SPINEL_EXTRACTOR_DIR=<path> to Spinel/.env"))
}

fn env_file_value(key: &str) -> Option<String> {
    let env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../.env");
    fs::read_to_string(env_path)
        .ok()?
        .lines()
        .filter_map(|line| line.split_once('='))
        .find(|(candidate_key, _)| candidate_key.trim() == key)
        .map(|(_, value)| value.trim().trim_matches('"').to_owned())
}
