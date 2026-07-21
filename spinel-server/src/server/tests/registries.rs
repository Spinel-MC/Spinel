use crate::server::MinecraftServer;
use spinel_network::types::Identifier;
use spinel_registry::dimension_type::DimensionType;

#[test]
fn minecraft_server_register_dimension_type_adds_custom_dimension_to_registry() {
    let mut server = MinecraftServer::new();
    let dimension_type = DimensionType::builder()
        .vertical_bounds(-32, 256, 128)
        .build();
    let dimension_type_key = server
        .register_dimension_type(Identifier::minecraft("custom_height"), dimension_type)
        .unwrap();
    let registered_dimension_type = server
        .registries()
        .dimension_type()
        .get(&dimension_type_key)
        .unwrap();

    assert_eq!(registered_dimension_type.min_y, -32);
    assert_eq!(registered_dimension_type.height, 256);
    assert_eq!(registered_dimension_type.logical_height, 128);
}
