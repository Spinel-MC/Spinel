use super::super::commands::{ArgumentParserType, CommandNode, CommandsPacket};
use spinel_network::DataType;

#[test]
fn commands_packet_matches_reference_root_and_literal_node_shape() {
    let packet = CommandsPacket {
        nodes: vec![
            CommandNode::root(vec![1]),
            CommandNode::literal("test", vec![2], false),
            CommandNode::literal("spawn", vec![3], false),
            CommandNode::literal("zombie", Vec::new(), true),
        ],
        root_index: 0,
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = CommandsPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(CommandsPacket::get_id(), 0x10);
    assert_eq!(decoded_packet.root_index, 0);
    assert_eq!(decoded_packet.nodes, packet.nodes);
    assert_eq!(packet.nodes[3].flags, 0x05);
}

#[test]
fn commands_packet_matches_reference_argument_node_shape() {
    let packet = CommandsPacket {
        nodes: vec![
            CommandNode::root(vec![1]),
            CommandNode::literal("spawn", vec![2], false),
            CommandNode::argument(
                "entity",
                ArgumentParserType::ResourceLocation,
                Vec::new(),
                true,
                Some("minecraft:summonable_entities".to_string()),
            ),
        ],
        root_index: 0,
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = CommandsPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet.nodes, packet.nodes);
    assert_eq!(packet.nodes[2].flags, 0x16);
    assert_eq!(
        packet.nodes[2].parser,
        Some(ArgumentParserType::ResourceLocation)
    );
}

#[test]
fn commands_packet_decodes_reference_argument_property_bytes() {
    let packet = CommandsPacket {
        nodes: vec![
            CommandNode::root((1..=6).collect()),
            argument_node("word", ArgumentParserType::String, vec![0]),
            argument_node("integer", ArgumentParserType::Integer, integer_properties()),
            argument_node("double", ArgumentParserType::Double, double_properties()),
            argument_node("entity", ArgumentParserType::Entity, vec![3]),
            argument_node(
                "time",
                ArgumentParserType::Time,
                20i32.to_be_bytes().to_vec(),
            ),
            argument_node(
                "resource",
                ArgumentParserType::Resource,
                encoded_string("minecraft:block"),
            ),
        ],
        root_index: 0,
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = CommandsPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet.nodes, packet.nodes);
}

fn argument_node(
    argument_name: &str,
    parser: ArgumentParserType,
    properties: Vec<u8>,
) -> CommandNode {
    let mut command_node = CommandNode::argument(argument_name, parser, Vec::new(), true, None);
    command_node.properties = properties;
    command_node
}

fn integer_properties() -> Vec<u8> {
    let mut properties = vec![3];
    properties.extend(1i32.to_be_bytes());
    properties.extend(64i32.to_be_bytes());
    properties
}

fn double_properties() -> Vec<u8> {
    let mut properties = vec![3];
    properties.extend(1.25f64.to_be_bytes());
    properties.extend(64.5f64.to_be_bytes());
    properties
}

fn encoded_string(value: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    value.to_string().encode(&mut bytes).unwrap();
    bytes
}
