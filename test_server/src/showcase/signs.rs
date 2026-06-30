use spinel::{
    nbt::{Nbt, NbtCompound},
    server::world::{Block, BlockInstanceExt, BlockPosition, Chunk, ChunkPosition, World},
};
use std::io;

#[derive(Clone, Copy)]
pub enum ShowcaseSignCommand {
    Player,
    Entity,
    Inventory,
    World,
}

pub struct ShowcaseSigns;

impl ShowcaseSigns {
    pub fn install(world: &mut World) -> io::Result<()> {
        Self::entries()
            .into_iter()
            .try_for_each(|entry| Self::install_entry(world, entry))
    }

    pub fn positions() -> [BlockPosition; 4] {
        Self::entries().map(|entry| entry.position)
    }

    pub fn install_on_chunk(chunk: &mut Chunk) {
        let chunk_x = chunk.x();
        let chunk_z = chunk.z();

        Self::entries()
            .into_iter()
            .filter(|entry| {
                entry.position.x.div_euclid(16) == chunk_x
                    && entry.position.z.div_euclid(16) == chunk_z
            })
            .for_each(|entry| {
                chunk.set_block_instance(
                    entry.position,
                    Block::OAK_SIGN.with_nbt(Self::sign_nbt(entry)),
                );
            });
    }

    pub fn command_at_position(position: BlockPosition) -> Option<ShowcaseSignCommand> {
        Self::entries()
            .into_iter()
            .find(|entry| entry.position == position)
            .map(|entry| entry.command)
    }

    fn install_entry(world: &mut World, entry: ShowcaseSignEntry) -> io::Result<()> {
        let chunk_position = ChunkPosition::new(
            entry.position.x.div_euclid(16),
            entry.position.z.div_euclid(16),
        );
        let _ = world.load_chunk(chunk_position)?;
        world.set_block_instance(
            entry.position,
            Block::OAK_SIGN.with_nbt(Self::sign_nbt(entry)),
        )?;
        Ok(())
    }

    fn entries() -> [ShowcaseSignEntry; 4] {
        [
            ShowcaseSignEntry::new(
                BlockPosition::new(1, 4, 5),
                "Player",
                ShowcaseSignCommand::Player,
            ),
            ShowcaseSignEntry::new(
                BlockPosition::new(3, 4, 5),
                "Entity",
                ShowcaseSignCommand::Entity,
            ),
            ShowcaseSignEntry::new(
                BlockPosition::new(5, 4, 5),
                "Inventory",
                ShowcaseSignCommand::Inventory,
            ),
            ShowcaseSignEntry::new(
                BlockPosition::new(7, 4, 5),
                "World",
                ShowcaseSignCommand::World,
            ),
        ]
    }

    fn sign_nbt(entry: ShowcaseSignEntry) -> NbtCompound {
        NbtCompound::new()
            .put("front_text", Self::blank_text_compound())
            .put("back_text", Self::label_text_compound(entry.label))
            .put("is_waxed", true)
    }
    fn label_text_compound(label: &str) -> NbtCompound {
        Self::text_compound(Nbt::list([label, "Showcase", "Right click", ""]), true)
    }

    fn blank_text_compound() -> NbtCompound {
        Self::text_compound(Nbt::list(["", "", "", ""]), false)
    }

    fn text_compound(messages: Nbt, has_glowing_text: bool) -> NbtCompound {
        NbtCompound::new()
            .put("color", "black")
            .put("has_glowing_text", has_glowing_text)
            .put("messages", messages)
    }
}

#[derive(Clone, Copy)]
struct ShowcaseSignEntry {
    position: BlockPosition,
    label: &'static str,
    command: ShowcaseSignCommand,
}

impl ShowcaseSignEntry {
    const fn new(
        position: BlockPosition,
        label: &'static str,
        command: ShowcaseSignCommand,
    ) -> Self {
        Self {
            position,
            label,
            command,
        }
    }
}
