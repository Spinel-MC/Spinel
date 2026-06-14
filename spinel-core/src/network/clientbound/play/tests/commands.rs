use super::super::commands::{ArgumentParserType, CommandNode, CommandsPacket};
use spinel_network::DataType;

#[test]
fn commands_packet_matches_minestom_root_and_literal_node_shape() {
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
fn commands_packet_matches_minestom_argument_node_shape() {
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
