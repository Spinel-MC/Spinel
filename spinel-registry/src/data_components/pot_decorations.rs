use crate::Material;
use crate::data_components::DataComponentValue;
use spinel_nbt::Nbt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PotDecorations {
    back: Material,
    left: Material,
    right: Material,
    front: Material,
}

impl PotDecorations {
    #[must_use]
    pub fn new(back: Material, left: Material, right: Material, front: Material) -> Self {
        Self {
            back,
            left,
            right,
            front,
        }
    }

    #[must_use]
    pub fn from_list(materials: Vec<Material>) -> Self {
        Self {
            back: materials.first().cloned().unwrap_or(Material::BRICK),
            left: materials.get(1).cloned().unwrap_or(Material::BRICK),
            right: materials.get(2).cloned().unwrap_or(Material::BRICK),
            front: materials.get(3).cloned().unwrap_or(Material::BRICK),
        }
    }

    #[must_use]
    pub fn back(&self) -> &Material {
        &self.back
    }

    #[must_use]
    pub fn left(&self) -> &Material {
        &self.left
    }

    #[must_use]
    pub fn right(&self) -> &Material {
        &self.right
    }

    #[must_use]
    pub fn front(&self) -> &Material {
        &self.front
    }

    #[must_use]
    pub fn as_list(&self) -> Vec<Material> {
        vec![
            self.back.clone(),
            self.left.clone(),
            self.right.clone(),
            self.front.clone(),
        ]
    }
}

impl DataComponentValue for PotDecorations {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::List(
            self.as_list()
                .into_iter()
                .map(|material| Nbt::String(material.key().to_string()))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let Nbt::List(materials) = component_nbt else {
            return None;
        };
        let materials = materials
            .iter()
            .map(|material| match material {
                Nbt::String(material_key) => Material::from_key(material_key),
                _ => None,
            })
            .collect::<Option<Vec<_>>>()?;
        Some(Self::from_list(materials))
    }
}
