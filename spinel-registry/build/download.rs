use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct VersionTracker {
    minecraft_version: String,
}

pub fn ensure_datapacks_downloaded(minecraft_version: &str) -> io::Result<()> {
    let datapacks_dir = "build_assets/datapacks/default";
    let version_file = format!("{}/spinel.json", datapacks_dir);

    let needs_download = if Path::new(&version_file).exists() {
        let content = fs::read_to_string(&version_file)?;
        match serde_json::from_str::<VersionTracker>(&content) {
            Ok(tracker) => tracker.minecraft_version != minecraft_version,
            Err(_) => true,
        }
    } else {
        true
    };

    if !needs_download {
        println!(
            "cargo:warning=Minecraft assets already downloaded for version {}",
            minecraft_version
        );
        return Ok(());
    }

    println!(
        "cargo:warning=Downloading minecraft-assets for version {}...",
        minecraft_version
    );

    if Path::new(datapacks_dir).exists() {
        fs::remove_dir_all(datapacks_dir)?;
    }

    let url = format!(
        "https://github.com/InventivetalentDev/minecraft-assets/archive/refs/tags/{}.zip",
        minecraft_version
    );

    let response = reqwest::blocking::get(&url)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to download: {}", e)))?;

    if !response.status().is_success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Failed to download minecraft-assets: HTTP {}",
                response.status()
            ),
        ));
    }

    let bytes = response.bytes().map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to read response: {}", e),
        )
    })?;

    let temp_zip = "build_assets/temp_minecraft_assets.zip";
    let mut file = File::create(temp_zip)?;
    file.write_all(&bytes)?;
    drop(file);

    println!("cargo:warning=Extracting minecraft-assets...");

    let file = File::open(temp_zip)?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to open ZIP: {}", e)))?;

    fs::create_dir_all(datapacks_dir)?;

    let data_prefix = format!("minecraft-assets-{}/data/", minecraft_version);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to read ZIP entry: {}", e),
            )
        })?;

        let outpath = match file.enclosed_name() {
            Some(path) => {
                let path_str = path.to_string_lossy();

                if let Some(stripped) = path_str.strip_prefix(&data_prefix) {
                    Path::new(datapacks_dir).join("data").join(stripped)
                } else {
                    continue;
                }
            }
            None => continue,
        };

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    fs::remove_file(temp_zip)?;

    let tracker = VersionTracker {
        minecraft_version: minecraft_version.to_string(),
    };
    let tracker_json = serde_json::to_string_pretty(&tracker)?;
    fs::write(&version_file, tracker_json)?;

    println!(
        "cargo:warning=Successfully downloaded and extracted minecraft-assets for version {}",
        minecraft_version
    );

    Ok(())
}
