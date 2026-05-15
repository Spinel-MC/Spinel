pub fn to_snake_case(input: &str) -> String {
    let mut snake_case_name = String::new();

    for (character_index, character) in input.chars().enumerate() {
        if character.is_uppercase() {
            if character_index > 0 {
                snake_case_name.push('_');
            }

            snake_case_name.extend(character.to_lowercase());
            continue;
        }

        snake_case_name.push(character);
    }

    snake_case_name
}
