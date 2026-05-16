use proc_macro2::TokenStream;
use std::{fs, io, path::Path, process::Command};

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
mod world_blocks;

const SHOULD_FORMAT_OUTPUT: bool = true;
const GENERATED_OUTPUT_DIRECTORY: &str = "src/generated";
const VANILLA_DATAPACK_DIRECTORY: &str = "build_assets/datapacks/default";

pub fn main() {
    RegistryBuildScript::new()
        .run()
        .unwrap_or_else(|error| panic!("registry build failed: {}", error));
}

struct RegistryBuildScript {
    target_version: &'static str,
}

impl RegistryBuildScript {
    fn new() -> Self {
        Self {
            target_version: spinel_utils::constants::MINECRAFT_VERSION,
        }
    }

    fn run(self) -> io::Result<()> {
        self.emit_rerun_instructions();
        self.refresh_world_blocks_module()?;

        if self.generated_assets_are_current()? {
            return Ok(());
        }

        download::ensure_datapacks_downloaded(self.target_version).map_err(io::Error::other)?;
        self.ensure_output_directory()?;
        self.write_generated_modules()?;

        if SHOULD_FORMAT_OUTPUT {
            self.format_generated_modules()?;
        }

        Ok(())
    }

    fn emit_rerun_instructions(&self) {
        println!("cargo:rerun-if-changed=build/build.rs");
        println!("cargo:rerun-if-changed=build/world_blocks.rs");
        println!("cargo:rerun-if-changed=build/world_block_matches.rs");
        println!("cargo:rerun-if-changed={}", self.version_file_path());
    }

    fn generated_assets_are_current(&self) -> io::Result<bool> {
        if !Path::new(&self.version_file_path()).exists()
            || !Path::new(GENERATED_OUTPUT_DIRECTORY).exists()
        {
            return Ok(false);
        }

        let version_manifest = fs::read_to_string(self.version_file_path())?;
        let version_json = serde_json::from_str::<serde_json::Value>(&version_manifest)
            .map_err(io::Error::other)?;

        if version_json["minecraft_version"] != self.target_version {
            return Ok(false);
        }

        if fs::read_dir(GENERATED_OUTPUT_DIRECTORY)?.next().is_none() {
            return Ok(false);
        }

        Ok(Path::new(&format!(
            "{GENERATED_OUTPUT_DIRECTORY}/vanilla_world_blocks.rs"
        ))
        .exists())
    }

    fn ensure_output_directory(&self) -> io::Result<()> {
        fs::create_dir_all(GENERATED_OUTPUT_DIRECTORY)
    }

    fn refresh_world_blocks_module(&self) -> io::Result<()> {
        if !Path::new(GENERATED_OUTPUT_DIRECTORY).exists()
            || !Path::new(world_blocks::BLOCK_EXTRACTION_PATH).exists()
        {
            return Ok(());
        }

        let output_path = format!("{GENERATED_OUTPUT_DIRECTORY}/vanilla_world_blocks.rs");
        let should_refresh = match fs::metadata(&output_path) {
            Ok(metadata) => {
                let generated_blocks = metadata.modified()?;
                generated_blocks < fs::metadata(world_blocks::BLOCK_EXTRACTION_PATH)?.modified()?
                    || generated_blocks < fs::metadata("build/world_blocks.rs")?.modified()?
                    || generated_blocks
                        < fs::metadata("build/world_block_matches.rs")?.modified()?
            }
            Err(_) => true,
        };

        if !should_refresh {
            return Ok(());
        }

        fs::write(&output_path, world_blocks::build().to_string())?;
        let _ = Command::new("rustfmt").arg(output_path).output();
        Ok(())
    }

    fn write_generated_modules(&self) -> io::Result<()> {
        for (file_contents, file_name) in self.generated_modules() {
            let output_path = format!("{GENERATED_OUTPUT_DIRECTORY}/vanilla_{file_name}.rs");
            fs::write(output_path, file_contents.to_string())?;
        }

        Ok(())
    }

    fn format_generated_modules(&self) -> io::Result<()> {
        for directory_entry in fs::read_dir(GENERATED_OUTPUT_DIRECTORY)? {
            let directory_entry = directory_entry?;
            let _ = Command::new("rustfmt").arg(directory_entry.path()).output();
        }

        Ok(())
    }

    fn generated_modules(&self) -> [(TokenStream, &'static str); 24] {
        [
            (blocks::build(), "blocks"),
            (world_blocks::build(), "world_blocks"),
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
            (packets::PacketModuleBuilder::build(), "packets"),
        ]
    }

    fn version_file_path(&self) -> String {
        format!("{}/spinel.json", VANILLA_DATAPACK_DIRECTORY)
    }
}
