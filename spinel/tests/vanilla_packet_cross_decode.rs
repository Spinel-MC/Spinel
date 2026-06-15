use spinel::network::PacketFixtureCatalog;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
#[ignore = "requires SpinelExtractor and a Vanilla runtime"]
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
    let verification_output = Command::new(gradle_wrapper)
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
