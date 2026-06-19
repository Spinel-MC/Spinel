use spinel_macros::packet;
use spinel_network::DataType;
use spinel_network::types::Array;
use spinel_network::types::var_int::VarIntWrapper;

#[derive(Debug, Clone)]
pub struct Tag {
    pub tag_name: String,
    pub entries: Array<VarIntWrapper>,
}

impl DataType for Tag {
    fn encode<W: ::std::io::Write>(&self, w: &mut W) -> ::std::io::Result<()> {
        <String as DataType>::encode(&self.tag_name, w)?;
        <Array<VarIntWrapper> as DataType>::encode(&self.entries, w)
    }
    fn decode<R: ::std::io::Read>(r: &mut R) -> ::std::io::Result<Self> {
        Ok(Self {
            tag_name: <String as DataType>::decode(r)?,
            entries: <Array<VarIntWrapper> as DataType>::decode(r)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct TagRegistry {
    pub registry_name: String,
    pub tags: Array<Tag>,
}

impl DataType for TagRegistry {
    fn encode<W: ::std::io::Write>(&self, w: &mut W) -> ::std::io::Result<()> {
        <String as DataType>::encode(&self.registry_name, w)?;
        <Array<Tag> as DataType>::encode(&self.tags, w)
    }
    fn decode<R: ::std::io::Read>(r: &mut R) -> ::std::io::Result<Self> {
        Ok(Self {
            registry_name: <String as DataType>::decode(r)?,
            tags: <Array<Tag> as DataType>::decode(r)?,
        })
    }
}

#[packet(id: "update_tags", state: ConnectionState::Play, recipient: Recipient::Client)]
#[derive(Debug, Clone)]
pub struct UpdateTagsPacket {
    pub registries: Array<TagRegistry>,
}

impl UpdateTagsPacket {
    pub fn new(registries: Vec<TagRegistry>) -> Self {
        Self {
            registries: Array(registries),
        }
    }
}
