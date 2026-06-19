use crate::data_type::DataType;
use crate::types::{ExactDataComponentPredicate, ItemCost, MerchantOffer, MerchantOffers, Slot};

#[test]
fn merchant_offer_roundtrips() {
    let merchant_offer = MerchantOffer {
        primary_cost: ItemCost {
            item_id: 1,
            count: 3,
            exact_components: ExactDataComponentPredicate::default(),
        },
        result: Slot {
            count: 1,
            item_id: 2,
            components: Default::default(),
        },
        secondary_cost: Some(ItemCost {
            item_id: 3,
            count: 1,
            exact_components: ExactDataComponentPredicate::default(),
        }),
        is_out_of_stock: false,
        uses: 4,
        max_uses: 12,
        experience: 7,
        special_price: -2,
        price_multiplier: 0.05,
        demand: 9,
    };
    let mut payload = Vec::new();

    merchant_offer.encode(&mut payload).unwrap();
    let decoded_offer = MerchantOffer::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_offer, merchant_offer);
}

#[test]
fn merchant_offers_roundtrip() {
    let merchant_offers = MerchantOffers {
        offers: vec![MerchantOffer {
            primary_cost: ItemCost {
                item_id: 4,
                count: 5,
                exact_components: ExactDataComponentPredicate::default(),
            },
            result: Slot {
                count: 2,
                item_id: 6,
                components: Default::default(),
            },
            secondary_cost: None,
            is_out_of_stock: true,
            uses: 1,
            max_uses: 8,
            experience: 2,
            special_price: 0,
            price_multiplier: 0.2,
            demand: 3,
        }],
    };
    let mut payload = Vec::new();

    merchant_offers.encode(&mut payload).unwrap();
    let decoded_offers = MerchantOffers::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_offers, merchant_offers);
}
