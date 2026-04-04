use std::{fs, path::Path, process::Command};

mod banner_patterns;
mod biomes;
mod block_tags;
mod blocks;
mod cat_variants;
mod chat_types;
mod chicken_variants;
mod cow_variants;
mod damage_types;
mod dialogs;
mod dimension_types;
mod download;
mod frog_variants;
mod instruments;
mod item_tags;
mod items;
mod jukebox_songs;
mod packets;
mod painting_variants;
mod pig_variants;
mod trim_materials;
mod trim_patterns;
mod types;
mod wolf_sound_variants;
mod wolf_variants;

const FMT: bool = true;
const OUT_DIR: &str = "src/generated";
const DATAPACKS_DIR: &str = "build_assets/datapacks/default";

pub fn main() {
    let target_version = spinel_utils::constants::MINECRAFT_VERSION;
    let version_file = format!("{}/spinel.json", DATAPACKS_DIR);

    // Tell Cargo to rerun if the version constant or the asset manifest changes
    println!("cargo:rerun-if-changed=build/build.rs");
    println!("cargo:rerun-if-changed={}", version_file);

    // --- FAST PATH CHECK ---
    let mut needs_build = true;
    if Path::new(&version_file).exists() && Path::new(OUT_DIR).exists() {
        if let Ok(content) = fs::read_to_string(&version_file) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                if json["minecraft_version"] == target_version {
                    // Assets match version, now check if we have generated files
                    if fs::read_dir(OUT_DIR)
                        .map(|mut d| d.next().is_some())
                        .unwrap_or(false)
                    {
                        needs_build = false;
                    }
                }
            }
        }
    }

    if !needs_build {
        return;
    }

    // --- FULL BUILD START ---
    download::ensure_datapacks_downloaded(target_version)
        .expect("Failed to download minecraft-assets");

    if !Path::new(OUT_DIR).exists() {
        fs::create_dir_all(OUT_DIR).unwrap();
    }

    let vanilla_builds = [
        (blocks::build(), "blocks"),
        (banner_patterns::build(), "banner_patterns"),
        (biomes::build(), "biomes"),
        (block_tags::build(), "block_tags"),
        (cat_variants::build(), "cat_variants"),
        (chat_types::build(), "chat_types"),
        (chicken_variants::build(), "chicken_variants"),
        (cow_variants::build(), "cow_variants"),
        (damage_types::build(), "damage_types"),
        (dialogs::build(), "dialogs"),
        (dimension_types::build(), "dimension_types"),
        (frog_variants::build(), "frog_variants"),
        (instruments::build(), "instruments"),
        (items::build(), "items"),
        (item_tags::build(), "item_tags"),
        (jukebox_songs::build(), "jukebox_songs"),
        (painting_variants::build(), "painting_variants"),
        (pig_variants::build(), "pig_variants"),
        (trim_materials::build(), "trim_materials"),
        (trim_patterns::build(), "trim_patterns"),
        (wolf_sound_variants::build(), "wolf_sound_variants"),
        (wolf_variants::build(), "wolf_variants"),
        (packets::build(), "packets"),
    ];

    for (content, file_name) in vanilla_builds {
        let path = format!("{OUT_DIR}/vanilla_{file_name}.rs");
        fs::write(&path, content.to_string()).unwrap();
    }

    if FMT {
        if let Ok(entries) = fs::read_dir(OUT_DIR) {
            for entry in entries.flatten() {
                let _ = Command::new("rustfmt").arg(entry.path()).output();
            }
        }
    }
}
