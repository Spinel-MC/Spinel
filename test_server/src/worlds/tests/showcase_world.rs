use crate::showcase::ShowcaseSigns;
use crate::worlds::ShowcaseWorld;
use spinel::nbt::Nbt;
use spinel::registry::Identifier;
use spinel::registry::block_entity_type::BlockEntityType;
use spinel::server::MinecraftServer;
use spinel::server::world::{Block, ChunkPosition};

#[test]
fn showcase_world_generation_preserves_showcase_sign_blocks() {
    let mut server = MinecraftServer::new();
    ShowcaseWorld::install(&mut server).unwrap();
    let world_id = server
        .world_manager
        .worlds()
        .iter()
        .find(|world| world.name() == &Identifier::minecraft("overworld"))
        .unwrap()
        .uuid;
    {
        let world = server.world_manager.world_mut(world_id).unwrap();

        ShowcaseSigns::positions()
            .into_iter()
            .for_each(|sign_position| {
                assert_eq!(world.loaded_block_at(sign_position), Some(Block::OAK_SIGN));
            });

        world.regenerate_chunk(ChunkPosition::new(0, 0));

        ShowcaseSigns::positions()
            .into_iter()
            .for_each(|sign_position| {
                assert_eq!(world.loaded_block_at(sign_position), Some(Block::OAK_SIGN));
            });

        world.unload_chunk(ChunkPosition::new(0, 0)).unwrap();
        world.load_chunk(ChunkPosition::new(0, 0)).unwrap();

        ShowcaseSigns::positions()
            .into_iter()
            .for_each(|sign_position| {
                assert_eq!(world.loaded_block_at(sign_position), Some(Block::OAK_SIGN));
            });
    }

    let world = server.world_manager.world(world_id).unwrap();
    let chunk_packet = world
        .chunk(ChunkPosition::new(0, 0))
        .unwrap()
        .full_data_packet(server.registries())
        .unwrap();
    let mut network_sign_positions = chunk_packet
        .chunk_data
        .block_entities
        .iter()
        .filter(|block_entity| block_entity.block_entity_type == BlockEntityType::Sign)
        .map(|block_entity| {
            (
                block_entity.packed_xz >> 4,
                block_entity.y,
                block_entity.packed_xz & 15,
            )
        })
        .collect::<Vec<_>>();
    network_sign_positions.sort();

    assert_eq!(
        network_sign_positions,
        vec![(1, 4, 5), (3, 4, 5), (5, 4, 5), (7, 4, 5)]
    );

    let player_sign_block_entity = chunk_packet
        .chunk_data
        .block_entities
        .iter()
        .find(|block_entity| block_entity.packed_xz == 21 && block_entity.y == 4)
        .unwrap();

    assert_eq!(
        sign_text_messages(player_sign_block_entity.nbt.get("front_text").unwrap()),
        vec!["", "", "", ""]
    );
    assert_eq!(
        sign_text_messages(player_sign_block_entity.nbt.get("back_text").unwrap()),
        vec!["Player", "Showcase", "Right click", ""]
    );
}

fn sign_text_messages(text_nbt: &Nbt) -> Vec<String> {
    let Nbt::Compound(text_compound) = text_nbt else {
        panic!("sign text field must be a compound");
    };
    let Some(Nbt::List(messages)) = text_compound.get("messages") else {
        panic!("sign text compound must include messages");
    };

    messages
        .iter()
        .map(|message| match message {
            Nbt::String(message) => message.clone(),
            _ => panic!("sign message must be a text component string"),
        })
        .collect()
}
