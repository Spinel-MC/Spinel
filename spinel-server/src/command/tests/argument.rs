use crate::command::{ArgumentError, ArgumentType, CommandSender};

#[test]
fn integer_argument_ports_numeric_branches_and_property_bytes() {
    let mut argument = ArgumentType::integer("level");
    argument.between(0, 4);
    assert_eq!(argument.parse_input(&CommandSender::Server, "0"), Ok(0));
    assert_eq!(argument.parse_input(&CommandSender::Server, "0b100"), Ok(4));
    assert_eq!(argument.parse_input(&CommandSender::Server, "0x3"), Ok(3));
    assert_eq!(argument.parse_input(&CommandSender::Server, "2e0"), Ok(2));
    assert_eq!(
        argument
            .parse_input(&CommandSender::Server, "-1")
            .unwrap_err()
            .get_error_code(),
        2
    );
    assert_eq!(
        argument
            .parse_input(&CommandSender::Server, "5")
            .unwrap_err()
            .get_error_code(),
        3
    );
    assert_eq!(
        argument
            .parse_input(&CommandSender::Server, "2147483648")
            .unwrap_err()
            .get_error_code(),
        1
    );
    assert_eq!(
        argument.get_node_properties(),
        Some(vec![3, 0, 0, 0, 0, 0, 0, 0, 4])
    );
}

#[test]
fn mapped_and_filtered_arguments_preserve_fallible_contracts() {
    let mut argument = ArgumentType::integer("level");
    argument.set_default_value(3);
    let mapped = argument.map(|value| {
        if value == 3 {
            Err(ArgumentError::new("map", "3", 555))
        } else {
            Ok(value.to_string())
        }
    });
    assert_eq!(
        mapped
            .parse_input(&CommandSender::Server, "3")
            .unwrap_err()
            .get_error_code(),
        555
    );
    assert_eq!(
        mapped
            .get_default_value(&CommandSender::Server)
            .unwrap_err()
            .get_error_code(),
        555
    );
    let mut filtered_source = ArgumentType::integer("level");
    filtered_source.set_default_value(-1);
    let filtered = filtered_source.filter(|value| *value >= 0);
    assert_eq!(
        filtered.get_default_value(&CommandSender::Server),
        Ok(Some(-1))
    );
    assert_eq!(
        filtered
            .parse_input(&CommandSender::Server, "-1")
            .unwrap_err()
            .get_error_code(),
        556
    );
}

#[test]
fn integer_argument_encodes_every_bound_property_state() {
    let no_bounds = ArgumentType::integer("level");
    assert_eq!(no_bounds.get_node_properties(), Some(vec![0]));

    let mut minimum_only = ArgumentType::integer("level");
    minimum_only.min(-2);
    assert_eq!(
        minimum_only.get_node_properties(),
        Some(vec![1, 255, 255, 255, 254])
    );

    let mut maximum_only = ArgumentType::integer("level");
    maximum_only.max(7);
    assert_eq!(
        maximum_only.get_node_properties(),
        Some(vec![2, 0, 0, 0, 7])
    );

    let mut both_bounds = ArgumentType::integer("level");
    both_bounds.between(-2, 7);
    assert_eq!(
        both_bounds.get_node_properties(),
        Some(vec![3, 255, 255, 255, 254, 0, 0, 0, 7])
    );
}
