use crate::instrument::{Instrument, InstrumentRegistry};
use crate::types::Identifier;
use spinel_utils::component::text::TextComponent;
use std::borrow::Cow;
pub const ADMIRE_GOAT_HORN: &Instrument = &Instrument {
    key: Identifier::vanilla_static("admire_goat_horn"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("item.goat_horn.sound.4"),
    },
    use_duration: 7f32,
    range: 256f32,
    description: TextComponent::translatable("instrument.minecraft.admire_goat_horn"),
};
pub const CALL_GOAT_HORN: &Instrument = &Instrument {
    key: Identifier::vanilla_static("call_goat_horn"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("item.goat_horn.sound.5"),
    },
    use_duration: 7f32,
    range: 256f32,
    description: TextComponent::translatable("instrument.minecraft.call_goat_horn"),
};
pub const DREAM_GOAT_HORN: &Instrument = &Instrument {
    key: Identifier::vanilla_static("dream_goat_horn"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("item.goat_horn.sound.7"),
    },
    use_duration: 7f32,
    range: 256f32,
    description: TextComponent::translatable("instrument.minecraft.dream_goat_horn"),
};
pub const FEEL_GOAT_HORN: &Instrument = &Instrument {
    key: Identifier::vanilla_static("feel_goat_horn"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("item.goat_horn.sound.3"),
    },
    use_duration: 7f32,
    range: 256f32,
    description: TextComponent::translatable("instrument.minecraft.feel_goat_horn"),
};
pub const PONDER_GOAT_HORN: &Instrument = &Instrument {
    key: Identifier::vanilla_static("ponder_goat_horn"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("item.goat_horn.sound.0"),
    },
    use_duration: 7f32,
    range: 256f32,
    description: TextComponent::translatable("instrument.minecraft.ponder_goat_horn"),
};
pub const SEEK_GOAT_HORN: &Instrument = &Instrument {
    key: Identifier::vanilla_static("seek_goat_horn"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("item.goat_horn.sound.2"),
    },
    use_duration: 7f32,
    range: 256f32,
    description: TextComponent::translatable("instrument.minecraft.seek_goat_horn"),
};
pub const SING_GOAT_HORN: &Instrument = &Instrument {
    key: Identifier::vanilla_static("sing_goat_horn"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("item.goat_horn.sound.1"),
    },
    use_duration: 7f32,
    range: 256f32,
    description: TextComponent::translatable("instrument.minecraft.sing_goat_horn"),
};
pub const YEARN_GOAT_HORN: &Instrument = &Instrument {
    key: Identifier::vanilla_static("yearn_goat_horn"),
    sound_event: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("item.goat_horn.sound.6"),
    },
    use_duration: 7f32,
    range: 256f32,
    description: TextComponent::translatable("instrument.minecraft.yearn_goat_horn"),
};
pub fn register_instruments(registry: &mut InstrumentRegistry) {
    registry.register(ADMIRE_GOAT_HORN);
    registry.register(CALL_GOAT_HORN);
    registry.register(DREAM_GOAT_HORN);
    registry.register(FEEL_GOAT_HORN);
    registry.register(PONDER_GOAT_HORN);
    registry.register(SEEK_GOAT_HORN);
    registry.register(SING_GOAT_HORN);
    registry.register(YEARN_GOAT_HORN);
}
