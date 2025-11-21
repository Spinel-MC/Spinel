use std::fs;

use heck::ToShoutySnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use serde::Deserialize;
use crate::types::Identifier;

#[derive(Deserialize, Debug)]
pub struct DimensionTypeJson {
    #[serde(default)]
    fixed_time: Option<i64>,
    has_skylight: bool,
    has_ceiling: bool,
    ultrawarm: bool,
    natural: bool,
    coordinate_scale: f64,
    bed_works: bool,
    respawn_anchor_works: bool,
    min_y: i32,
    height: i32,
    logical_height: i32,
    infiniburn: String,
    #[serde(default = "default_effects")]
    effects: Identifier,
    ambient_light: f32,
    #[serde(default)]
    cloud_height: Option<i32>,
    piglin_safe: bool,
    has_raids: bool,
    monster_spawn_light_level: MonsterSpawnLightLevelJson,
    monster_spawn_block_light_limit: i32,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum MonsterSpawnLightLevelJson {
    Simple(i32),
    Complex {
        #[serde(rename = "type")]
        distribution_type: String,
        min_inclusive: i32,
        max_inclusive: i32,
    },
}

fn default_effects() -> Identifier {
    Identifier {
        namespace: "minecraft".into(),
        path: "overworld".into(),
    }
}

fn generate_identifier(resource: &Identifier) -> TokenStream {
    let namespace = resource.namespace.as_ref();
    let path = resource.path.as_ref();
    quote! { Identifier { namespace: Cow::Borrowed(#namespace), path: Cow::Borrowed(#path) } }
}

fn generate_option<T, F>(opt: &Option<T>, f: F) -> TokenStream
where
    F: FnOnce(&T) -> TokenStream,
{
    match opt {
        Some(val) => {
            let inner = f(val);
            quote! { Some(#inner) }
        }
        None => quote! { None },
    }
}

fn generate_monster_spawn_light_level(level: &MonsterSpawnLightLevelJson) -> TokenStream {
    match level {
        MonsterSpawnLightLevelJson::Simple(value) => {
            quote! { MonsterSpawnLightLevel::Simple(#value) }
        }
        MonsterSpawnLightLevelJson::Complex {
            distribution_type,
            min_inclusive,
            max_inclusive,
        } => {
            let dist_type = distribution_type.as_str();
            quote! {
                MonsterSpawnLightLevel::Complex {
                    distribution_type: #dist_type,
                    min_inclusive: #min_inclusive,
                    max_inclusive: #max_inclusive,
                }
            }
        }
    }
}

pub(crate) fn build() -> TokenStream {
    println!(
        "cargo:rerun-if-changed=build_assets/datapacks/default/data/minecraft/dimension_type/"
    );

    let dimension_type_dir =
        "build_assets/datapacks/default/data/minecraft/dimension_type";
    let mut dimension_types = Vec::new();

    // Read all dimension type JSON files
    for entry in fs::read_dir(dimension_type_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let dimension_type_name = path.file_stem().unwrap().to_str().unwrap().to_string();            
            // Skip metadata files from minecraft-assets (e.g., _all.json, _list.json)
            if dimension_type_name.starts_with('_') {
                continue;
            }
            
            let content = fs::read_to_string(&path).unwrap();
            let dimension_type: DimensionTypeJson = serde_json::from_str(&content)
                .unwrap_or_else(|e| panic!("Failed to parse {}: {}", dimension_type_name, e));

            dimension_types.push((dimension_type_name, dimension_type));
        }
    }

    // Sort dimension types by name for consistent generation
    dimension_types.sort_by(|a, b| a.0.cmp(&b.0));

    let mut stream = TokenStream::new();

    stream.extend(quote! {
        use crate::dimension_type::{
            DimensionType, DimensionTypeRegistry, MonsterSpawnLightLevel,
        };
        use crate::types::Identifier;
        use std::borrow::Cow;
    });

    // Generate static dimension type definitions
    for (dimension_type_name, dimension_type) in &dimension_types {
        let dimension_type_ident = Ident::new(
            &dimension_type_name.to_shouty_snake_case(),
            Span::call_site(),
        );
        let dimension_type_name_str = dimension_type_name.clone();

        let key = quote! { Identifier::vanilla_static(#dimension_type_name_str) };
        let fixed_time = generate_option(&dimension_type.fixed_time, |t| quote! { #t });
        let has_skylight = dimension_type.has_skylight;
        let has_ceiling = dimension_type.has_ceiling;
        let ultrawarm = dimension_type.ultrawarm;
        let natural = dimension_type.natural;
        let coordinate_scale = dimension_type.coordinate_scale;
        let bed_works = dimension_type.bed_works;
        let respawn_anchor_works = dimension_type.respawn_anchor_works;
        let min_y = dimension_type.min_y;
        let height = dimension_type.height;
        let logical_height = dimension_type.logical_height;
        let infiniburn = dimension_type.infiniburn.as_str();
        let effects = generate_identifier(&dimension_type.effects);
        let ambient_light = dimension_type.ambient_light;
        let cloud_height = generate_option(&dimension_type.cloud_height, |h| quote! { #h });
        let piglin_safe = dimension_type.piglin_safe;
        let has_raids = dimension_type.has_raids;
        let monster_spawn_light_level =
            generate_monster_spawn_light_level(&dimension_type.monster_spawn_light_level);
        let monster_spawn_block_light_limit = dimension_type.monster_spawn_block_light_limit;

        stream.extend(quote! {
            pub const #dimension_type_ident: &DimensionType = &DimensionType {
                key: #key,
                fixed_time: #fixed_time,
                has_skylight: #has_skylight,
                has_ceiling: #has_ceiling,
                ultrawarm: #ultrawarm,
                natural: #natural,
                coordinate_scale: #coordinate_scale,
                bed_works: #bed_works,
                respawn_anchor_works: #respawn_anchor_works,
                min_y: #min_y,
                height: #height,
                logical_height: #logical_height,
                infiniburn: #infiniburn,
                effects: #effects,
                ambient_light: #ambient_light,
                cloud_height: #cloud_height,
                piglin_safe: #piglin_safe,
                has_raids: #has_raids,
                monster_spawn_light_level: #monster_spawn_light_level,
                monster_spawn_block_light_limit: #monster_spawn_block_light_limit,
            };
        });
    }

    // Generate registration function
    let mut register_stream = TokenStream::new();
    for (dimension_type_name, _) in &dimension_types {
        let dimension_type_ident = Ident::new(
            &dimension_type_name.to_shouty_snake_case(),
            Span::call_site(),
        );
        register_stream.extend(quote! {
            registry.register(#dimension_type_ident);
        });
    }

    stream.extend(quote! {
        pub fn register_dimension_types(registry: &mut DimensionTypeRegistry) {
            #register_stream
        }
    });

    stream
}
