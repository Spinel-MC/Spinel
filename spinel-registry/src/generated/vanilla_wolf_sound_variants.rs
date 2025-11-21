use crate::types::Identifier;
use crate::wolf_sound_variant::{WolfSoundVariant, WolfSoundVariantRegistry};
use std::borrow::Cow;
pub const ANGRY: &WolfSoundVariant = &WolfSoundVariant {
    key: Identifier::vanilla_static("angry"),
    ambient_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_angry.ambient"),
    },
    death_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_angry.death"),
    },
    growl_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_angry.growl"),
    },
    hurt_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_angry.hurt"),
    },
    pant_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_angry.pant"),
    },
    whine_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_angry.whine"),
    },
};
pub const BIG: &WolfSoundVariant = &WolfSoundVariant {
    key: Identifier::vanilla_static("big"),
    ambient_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_big.ambient"),
    },
    death_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_big.death"),
    },
    growl_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_big.growl"),
    },
    hurt_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_big.hurt"),
    },
    pant_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_big.pant"),
    },
    whine_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_big.whine"),
    },
};
pub const CLASSIC: &WolfSoundVariant = &WolfSoundVariant {
    key: Identifier::vanilla_static("classic"),
    ambient_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf.ambient"),
    },
    death_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf.death"),
    },
    growl_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf.growl"),
    },
    hurt_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf.hurt"),
    },
    pant_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf.pant"),
    },
    whine_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf.whine"),
    },
};
pub const CUTE: &WolfSoundVariant = &WolfSoundVariant {
    key: Identifier::vanilla_static("cute"),
    ambient_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_cute.ambient"),
    },
    death_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_cute.death"),
    },
    growl_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_cute.growl"),
    },
    hurt_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_cute.hurt"),
    },
    pant_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_cute.pant"),
    },
    whine_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_cute.whine"),
    },
};
pub const GRUMPY: &WolfSoundVariant = &WolfSoundVariant {
    key: Identifier::vanilla_static("grumpy"),
    ambient_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_grumpy.ambient"),
    },
    death_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_grumpy.death"),
    },
    growl_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_grumpy.growl"),
    },
    hurt_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_grumpy.hurt"),
    },
    pant_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_grumpy.pant"),
    },
    whine_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_grumpy.whine"),
    },
};
pub const PUGLIN: &WolfSoundVariant = &WolfSoundVariant {
    key: Identifier::vanilla_static("puglin"),
    ambient_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_puglin.ambient"),
    },
    death_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_puglin.death"),
    },
    growl_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_puglin.growl"),
    },
    hurt_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_puglin.hurt"),
    },
    pant_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_puglin.pant"),
    },
    whine_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_puglin.whine"),
    },
};
pub const SAD: &WolfSoundVariant = &WolfSoundVariant {
    key: Identifier::vanilla_static("sad"),
    ambient_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_sad.ambient"),
    },
    death_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_sad.death"),
    },
    growl_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_sad.growl"),
    },
    hurt_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_sad.hurt"),
    },
    pant_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_sad.pant"),
    },
    whine_sound: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity.wolf_sad.whine"),
    },
};
pub fn register_wolf_sound_variants(registry: &mut WolfSoundVariantRegistry) {
    registry.register(ANGRY);
    registry.register(BIG);
    registry.register(CLASSIC);
    registry.register(CUTE);
    registry.register(GRUMPY);
    registry.register(PUGLIN);
    registry.register(SAD);
}
