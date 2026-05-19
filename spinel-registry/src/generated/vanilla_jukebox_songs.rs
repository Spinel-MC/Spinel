use crate::{DynamicRegistry, RegistryKey};
use crate::jukebox_song::JukeboxSong;
impl JukeboxSong {
    pub const _11: RegistryKey<Self> = RegistryKey::vanilla_static("11");
    pub const _13: RegistryKey<Self> = RegistryKey::vanilla_static("13");
    pub const _5: RegistryKey<Self> = RegistryKey::vanilla_static("5");
    pub const BLOCKS: RegistryKey<Self> = RegistryKey::vanilla_static("blocks");
    pub const CAT: RegistryKey<Self> = RegistryKey::vanilla_static("cat");
    pub const CHIRP: RegistryKey<Self> = RegistryKey::vanilla_static("chirp");
    pub const CREATOR: RegistryKey<Self> = RegistryKey::vanilla_static("creator");
    pub const CREATOR_MUSIC_BOX: RegistryKey<Self> = RegistryKey::vanilla_static("creator_music_box");
    pub const FAR: RegistryKey<Self> = RegistryKey::vanilla_static("far");
    pub const LAVA_CHICKEN: RegistryKey<Self> = RegistryKey::vanilla_static("lava_chicken");
    pub const MALL: RegistryKey<Self> = RegistryKey::vanilla_static("mall");
    pub const MELLOHI: RegistryKey<Self> = RegistryKey::vanilla_static("mellohi");
    pub const OTHERSIDE: RegistryKey<Self> = RegistryKey::vanilla_static("otherside");
    pub const PIGSTEP: RegistryKey<Self> = RegistryKey::vanilla_static("pigstep");
    pub const PRECIPICE: RegistryKey<Self> = RegistryKey::vanilla_static("precipice");
    pub const RELIC: RegistryKey<Self> = RegistryKey::vanilla_static("relic");
    pub const STAL: RegistryKey<Self> = RegistryKey::vanilla_static("stal");
    pub const STRAD: RegistryKey<Self> = RegistryKey::vanilla_static("strad");
    pub const TEARS: RegistryKey<Self> = RegistryKey::vanilla_static("tears");
    pub const WAIT: RegistryKey<Self> = RegistryKey::vanilla_static("wait");
    pub const WARD: RegistryKey<Self> = RegistryKey::vanilla_static("ward");
}
pub fn register_jukebox_songs(registry: &mut DynamicRegistry<JukeboxSong>) {
    let _ = registry.register_vanilla(JukeboxSong::_11, JukeboxSong::default());
    let _ = registry.register_vanilla(JukeboxSong::_13, JukeboxSong::default());
    let _ = registry.register_vanilla(JukeboxSong::_5, JukeboxSong::default());
    let _ = registry.register_vanilla(JukeboxSong::BLOCKS, JukeboxSong::default());
    let _ = registry.register_vanilla(JukeboxSong::CAT, JukeboxSong::default());
    let _ = registry.register_vanilla(JukeboxSong::CHIRP, JukeboxSong::default());
    let _ = registry.register_vanilla(JukeboxSong::CREATOR, JukeboxSong::default());
    let _ = registry.register_vanilla(JukeboxSong::CREATOR_MUSIC_BOX, JukeboxSong::default());
    let _ = registry.register_vanilla(JukeboxSong::FAR, JukeboxSong::default());
    let _ = registry.register_vanilla(JukeboxSong::LAVA_CHICKEN, JukeboxSong::default());
    let _ = registry.register_vanilla(JukeboxSong::MALL, JukeboxSong::default());
    let _ = registry.register_vanilla(JukeboxSong::MELLOHI, JukeboxSong::default());
    let _ = registry.register_vanilla(JukeboxSong::OTHERSIDE, JukeboxSong::default());
    let _ = registry.register_vanilla(JukeboxSong::PIGSTEP, JukeboxSong::default());
    let _ = registry.register_vanilla(JukeboxSong::PRECIPICE, JukeboxSong::default());
    let _ = registry.register_vanilla(JukeboxSong::RELIC, JukeboxSong::default());
    let _ = registry.register_vanilla(JukeboxSong::STAL, JukeboxSong::default());
    let _ = registry.register_vanilla(JukeboxSong::STRAD, JukeboxSong::default());
    let _ = registry.register_vanilla(JukeboxSong::TEARS, JukeboxSong::default());
    let _ = registry.register_vanilla(JukeboxSong::WAIT, JukeboxSong::default());
    let _ = registry.register_vanilla(JukeboxSong::WARD, JukeboxSong::default());
}
