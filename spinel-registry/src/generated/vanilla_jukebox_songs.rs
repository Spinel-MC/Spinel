use crate::jukebox_song::{JukeboxSong, JukeboxSongRegistry};
use crate::types::Identifier;
use spinel_utils::component::text::TextComponent;
use std::borrow::Cow;
pub const MUSIC_DISC_11: &JukeboxSong = &JukeboxSong {
    key: Identifier::vanilla_static("11"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("music_disc.11"),
    },
    description: TextComponent::translatable("jukebox_song.minecraft.11"),
    length_in_seconds: 71f32,
    comparator_output: 11i32,
};
pub const MUSIC_DISC_13: &JukeboxSong = &JukeboxSong {
    key: Identifier::vanilla_static("13"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("music_disc.13"),
    },
    description: TextComponent::translatable("jukebox_song.minecraft.13"),
    length_in_seconds: 178f32,
    comparator_output: 1i32,
};
pub const MUSIC_DISC_5: &JukeboxSong = &JukeboxSong {
    key: Identifier::vanilla_static("5"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("music_disc.5"),
    },
    description: TextComponent::translatable("jukebox_song.minecraft.5"),
    length_in_seconds: 178f32,
    comparator_output: 15i32,
};
pub const BLOCKS: &JukeboxSong = &JukeboxSong {
    key: Identifier::vanilla_static("blocks"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("music_disc.blocks"),
    },
    description: TextComponent::translatable("jukebox_song.minecraft.blocks"),
    length_in_seconds: 345f32,
    comparator_output: 3i32,
};
pub const CAT: &JukeboxSong = &JukeboxSong {
    key: Identifier::vanilla_static("cat"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("music_disc.cat"),
    },
    description: TextComponent::translatable("jukebox_song.minecraft.cat"),
    length_in_seconds: 185f32,
    comparator_output: 2i32,
};
pub const CHIRP: &JukeboxSong = &JukeboxSong {
    key: Identifier::vanilla_static("chirp"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("music_disc.chirp"),
    },
    description: TextComponent::translatable("jukebox_song.minecraft.chirp"),
    length_in_seconds: 185f32,
    comparator_output: 4i32,
};
pub const CREATOR: &JukeboxSong = &JukeboxSong {
    key: Identifier::vanilla_static("creator"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("music_disc.creator"),
    },
    description: TextComponent::translatable("jukebox_song.minecraft.creator"),
    length_in_seconds: 176f32,
    comparator_output: 12i32,
};
pub const CREATOR_MUSIC_BOX: &JukeboxSong = &JukeboxSong {
    key: Identifier::vanilla_static("creator_music_box"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("music_disc.creator_music_box"),
    },
    description: TextComponent::translatable("jukebox_song.minecraft.creator_music_box"),
    length_in_seconds: 73f32,
    comparator_output: 11i32,
};
pub const FAR: &JukeboxSong = &JukeboxSong {
    key: Identifier::vanilla_static("far"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("music_disc.far"),
    },
    description: TextComponent::translatable("jukebox_song.minecraft.far"),
    length_in_seconds: 174f32,
    comparator_output: 5i32,
};
pub const LAVA_CHICKEN: &JukeboxSong = &JukeboxSong {
    key: Identifier::vanilla_static("lava_chicken"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("music_disc.lava_chicken"),
    },
    description: TextComponent::translatable("jukebox_song.minecraft.lava_chicken"),
    length_in_seconds: 134f32,
    comparator_output: 9i32,
};
pub const MALL: &JukeboxSong = &JukeboxSong {
    key: Identifier::vanilla_static("mall"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("music_disc.mall"),
    },
    description: TextComponent::translatable("jukebox_song.minecraft.mall"),
    length_in_seconds: 197f32,
    comparator_output: 6i32,
};
pub const MELLOHI: &JukeboxSong = &JukeboxSong {
    key: Identifier::vanilla_static("mellohi"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("music_disc.mellohi"),
    },
    description: TextComponent::translatable("jukebox_song.minecraft.mellohi"),
    length_in_seconds: 96f32,
    comparator_output: 7i32,
};
pub const OTHERSIDE: &JukeboxSong = &JukeboxSong {
    key: Identifier::vanilla_static("otherside"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("music_disc.otherside"),
    },
    description: TextComponent::translatable("jukebox_song.minecraft.otherside"),
    length_in_seconds: 195f32,
    comparator_output: 14i32,
};
pub const PIGSTEP: &JukeboxSong = &JukeboxSong {
    key: Identifier::vanilla_static("pigstep"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("music_disc.pigstep"),
    },
    description: TextComponent::translatable("jukebox_song.minecraft.pigstep"),
    length_in_seconds: 149f32,
    comparator_output: 13i32,
};
pub const PRECIPICE: &JukeboxSong = &JukeboxSong {
    key: Identifier::vanilla_static("precipice"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("music_disc.precipice"),
    },
    description: TextComponent::translatable("jukebox_song.minecraft.precipice"),
    length_in_seconds: 299f32,
    comparator_output: 13i32,
};
pub const RELIC: &JukeboxSong = &JukeboxSong {
    key: Identifier::vanilla_static("relic"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("music_disc.relic"),
    },
    description: TextComponent::translatable("jukebox_song.minecraft.relic"),
    length_in_seconds: 218f32,
    comparator_output: 14i32,
};
pub const STAL: &JukeboxSong = &JukeboxSong {
    key: Identifier::vanilla_static("stal"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("music_disc.stal"),
    },
    description: TextComponent::translatable("jukebox_song.minecraft.stal"),
    length_in_seconds: 150f32,
    comparator_output: 8i32,
};
pub const STRAD: &JukeboxSong = &JukeboxSong {
    key: Identifier::vanilla_static("strad"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("music_disc.strad"),
    },
    description: TextComponent::translatable("jukebox_song.minecraft.strad"),
    length_in_seconds: 188f32,
    comparator_output: 9i32,
};
pub const TEARS: &JukeboxSong = &JukeboxSong {
    key: Identifier::vanilla_static("tears"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("music_disc.tears"),
    },
    description: TextComponent::translatable("jukebox_song.minecraft.tears"),
    length_in_seconds: 175f32,
    comparator_output: 10i32,
};
pub const WAIT: &JukeboxSong = &JukeboxSong {
    key: Identifier::vanilla_static("wait"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("music_disc.wait"),
    },
    description: TextComponent::translatable("jukebox_song.minecraft.wait"),
    length_in_seconds: 238f32,
    comparator_output: 12i32,
};
pub const WARD: &JukeboxSong = &JukeboxSong {
    key: Identifier::vanilla_static("ward"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("music_disc.ward"),
    },
    description: TextComponent::translatable("jukebox_song.minecraft.ward"),
    length_in_seconds: 251f32,
    comparator_output: 10i32,
};
pub fn register_jukebox_songs(registry: &mut JukeboxSongRegistry) {
    registry.register(MUSIC_DISC_11);
    registry.register(MUSIC_DISC_13);
    registry.register(MUSIC_DISC_5);
    registry.register(BLOCKS);
    registry.register(CAT);
    registry.register(CHIRP);
    registry.register(CREATOR);
    registry.register(CREATOR_MUSIC_BOX);
    registry.register(FAR);
    registry.register(LAVA_CHICKEN);
    registry.register(MALL);
    registry.register(MELLOHI);
    registry.register(OTHERSIDE);
    registry.register(PIGSTEP);
    registry.register(PRECIPICE);
    registry.register(RELIC);
    registry.register(STAL);
    registry.register(STRAD);
    registry.register(TEARS);
    registry.register(WAIT);
    registry.register(WARD);
}
