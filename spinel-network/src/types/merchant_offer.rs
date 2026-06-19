use crate::data_type::DataType;
use crate::types::{ExactDataComponentPredicate, Slot, var_int::VarIntWrapper};
use spinel_registry::{ItemStack, Material};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemCost {
    pub item_id: i32,
    pub count: i32,
    pub exact_components: ExactDataComponentPredicate,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MerchantOffer {
    pub primary_cost: ItemCost,
    pub result: Slot,
    pub secondary_cost: Option<ItemCost>,
    pub is_out_of_stock: bool,
    pub uses: i32,
    pub max_uses: i32,
    pub experience: i32,
    pub special_price: i32,
    pub price_multiplier: f32,
    pub demand: i32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MerchantOffers {
    pub offers: Vec<MerchantOffer>,
}

impl ItemCost {
    pub fn as_item_stack(&self) -> Option<ItemStack> {
        let material = Material::from_id(self.item_id)?;
        Some(ItemStack::of(material).with_amount(self.count))
    }
}

impl DataType for ItemCost {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.item_id).encode(writer)?;
        VarIntWrapper(self.count).encode(writer)?;
        self.exact_components.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            item_id: VarIntWrapper::decode(reader)?.0,
            count: VarIntWrapper::decode(reader)?.0,
            exact_components: ExactDataComponentPredicate::decode(reader)?,
        })
    }
}

impl DataType for MerchantOffer {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.primary_cost.encode(writer)?;
        self.result.encode(writer)?;
        self.secondary_cost.encode(writer)?;
        self.is_out_of_stock.encode(writer)?;
        self.uses.encode(writer)?;
        self.max_uses.encode(writer)?;
        self.experience.encode(writer)?;
        self.special_price.encode(writer)?;
        self.price_multiplier.encode(writer)?;
        self.demand.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            primary_cost: ItemCost::decode(reader)?,
            result: Slot::decode(reader)?,
            secondary_cost: Option::<ItemCost>::decode(reader)?,
            is_out_of_stock: bool::decode(reader)?,
            uses: i32::decode(reader)?,
            max_uses: i32::decode(reader)?,
            experience: i32::decode(reader)?,
            special_price: i32::decode(reader)?,
            price_multiplier: f32::decode(reader)?,
            demand: i32::decode(reader)?,
        })
    }
}

impl DataType for MerchantOffers {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.offers.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            offers: Vec::<MerchantOffer>::decode(reader)?,
        })
    }
}
