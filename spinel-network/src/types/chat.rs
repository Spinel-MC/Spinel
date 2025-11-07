use spinel_utils::component::text::TextComponent;

#[derive(Debug, Clone)]
pub struct ChatType {
    pub type_id: i32,
    pub name: TextComponent,
    pub target: Option<TextComponent>,
}
