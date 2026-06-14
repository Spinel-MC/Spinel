use crate::world::{Block, BlockComparison, BlockHandler, BlockHandlerHandle, BlockState};
use spinel_nbt::{NbtCompound, Tag, TagReadable};
use spinel_registry::{BlockStateProperty, Identifier};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlockInstance {
    block_state: BlockState,
    nbt: Option<NbtCompound>,
    handler: Option<BlockHandlerHandle>,
}

impl BlockInstance {
    pub const fn new(block_state: BlockState) -> Self {
        Self {
            block_state,
            nbt: None,
            handler: None,
        }
    }

    pub const fn block(&self) -> Block {
        self.block_state.block()
    }

    pub const fn block_state(&self) -> BlockState {
        self.block_state
    }

    pub fn with_block_state(mut self, block_state: BlockState) -> Self {
        self.block_state = block_state;
        self
    }

    pub fn with_property(self, property: &str, value: &str) -> Option<Self> {
        Some(Self {
            block_state: self.block_state.with_property(property, value)?,
            nbt: self.nbt,
            handler: self.handler,
        })
    }

    pub fn with_properties<K, V>(self, properties: impl IntoIterator<Item = (K, V)>) -> Option<Self>
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        properties
            .into_iter()
            .try_fold(self, |block, (property, value)| {
                block.with_property(property.as_ref(), value.as_ref())
            })
    }

    pub fn with_tag<T>(mut self, tag: &Tag<T>, value: Option<T>) -> Self {
        let mut nbt = self.nbt.unwrap_or_default();
        tag.write(&mut nbt, value);
        self.nbt = (!nbt.is_empty()).then_some(nbt);
        self
    }

    pub fn with_nbt(mut self, nbt: Option<NbtCompound>) -> Self {
        self.nbt = nbt;
        self
    }

    pub fn with_handler(mut self, handler: Option<BlockHandlerHandle>) -> Self {
        self.handler = handler;
        self
    }

    pub fn with_new_handler(self, handler: impl BlockHandler + 'static) -> Self {
        self.with_handler(Some(BlockHandlerHandle::new(handler)))
    }

    pub fn nbt(&self) -> Option<&NbtCompound> {
        self.nbt.as_ref()
    }

    pub fn nbt_or_empty(&self) -> NbtCompound {
        self.nbt.clone().unwrap_or_default()
    }

    pub fn has_nbt(&self) -> bool {
        self.nbt.is_some()
    }

    pub fn handler(&self) -> Option<&BlockHandlerHandle> {
        self.handler.as_ref()
    }

    pub fn properties(&self) -> &'static [BlockStateProperty] {
        self.block_state.properties()
    }

    pub fn state(&self) -> String {
        let mut state = format!("minecraft:{}", self.block().path());
        if self.properties().is_empty() {
            return state;
        }
        state.push('[');
        self.properties()
            .iter()
            .enumerate()
            .for_each(|(property_index, property)| {
                if property_index > 0 {
                    state.push(',');
                }
                state.push_str(property.name);
                state.push('=');
                state.push_str(property.value);
            });
        state.push(']');
        state
    }

    pub fn property(&self, property: &str) -> Option<&'static str> {
        self.block_state.property(property)
    }

    pub fn possible_states(&self) -> Vec<Self> {
        (0..BlockState::COUNT)
            .filter_map(|state_id| BlockState::from_state_id(state_id as i32))
            .filter(|block_state| block_state.block() == self.block())
            .map(Self::from)
            .collect()
    }

    pub fn default_state(&self) -> Self {
        Self::from(self.block())
    }

    pub fn key(&self) -> Identifier {
        Identifier::minecraft(self.block().path())
    }

    pub const fn id(&self) -> i32 {
        self.block().id()
    }

    pub const fn state_id(&self) -> i32 {
        self.block_state.state_id()
    }

    pub const fn is_air(&self) -> bool {
        self.block().is_air()
    }

    pub const fn is_solid(&self) -> bool {
        self.block().is_solid()
    }

    pub const fn is_liquid(&self) -> bool {
        self.block().is_liquid()
    }

    pub fn compare(&self, other: &Self, comparison: BlockComparison) -> bool {
        match comparison {
            BlockComparison::Identity => std::ptr::eq(self, other),
            BlockComparison::BlockType => self.id() == other.id(),
            BlockComparison::BlockState => self.state_id() == other.state_id(),
        }
    }

    pub fn is_same_block_type(&self, other: &Self) -> bool {
        self.compare(other, BlockComparison::BlockType)
    }

    pub fn from_key(key: &str) -> Option<Self> {
        Block::from_key(key).map(Self::from)
    }

    pub fn from_state(state: &str) -> Option<Self> {
        if state.is_empty() {
            return None;
        }
        let Some(property_start) = state.find('[') else {
            return Self::from_key(state);
        };
        if property_start == 0 || !state.ends_with(']') {
            return None;
        }
        let block = Self::from_key(&state[..property_start])?;
        let property_query = &state[property_start + 1..state.len() - 1];
        if property_query.is_empty() {
            return Some(block);
        }
        property_query.split(',').try_fold(block, |block, entry| {
            let (property, value) = entry.split_once('=')?;
            block.with_property(property, value)
        })
    }

    pub fn from_id(id: i32) -> Option<Self> {
        Block::from_id(id).map(Self::from)
    }

    pub fn from_state_id(state_id: i32) -> Option<Self> {
        BlockState::from_state_id(state_id).map(Self::from)
    }
}

impl TagReadable for BlockInstance {
    fn get_tag<T>(&self, tag: &Tag<T>) -> Option<T> {
        match self.nbt.as_ref() {
            Some(nbt) => tag.read(nbt),
            None => tag.read(&NbtCompound::new()),
        }
    }
}

impl From<Block> for BlockInstance {
    fn from(block: Block) -> Self {
        Self::new(block.default_state())
    }
}

impl From<BlockState> for BlockInstance {
    fn from(block_state: BlockState) -> Self {
        Self::new(block_state)
    }
}
