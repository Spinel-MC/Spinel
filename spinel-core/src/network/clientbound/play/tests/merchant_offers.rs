use super::super::merchant_offers::MerchantOffersPacket;
use spinel_network::data_type::DataType;
use spinel_network::types::{
    ExactDataComponentPredicate, ItemCost, MerchantOffer, MerchantOffers, Slot,
};

#[test]
fn merchant_offers_packet_roundtrips() {
    let packet = MerchantOffersPacket {
        container_id: 7,
        offers: MerchantOffers {
            offers: vec![MerchantOffer {
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
            }],
        },
        villager_level: 2,
        villager_experience: 15,
        show_progress: true,
        can_restock: false,
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = MerchantOffersPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet.container_id, packet.container_id);
    assert_eq!(decoded_packet.offers, packet.offers);
    assert_eq!(decoded_packet.villager_level, packet.villager_level);
    assert_eq!(
        decoded_packet.villager_experience,
        packet.villager_experience
    );
    assert_eq!(decoded_packet.show_progress, packet.show_progress);
    assert_eq!(decoded_packet.can_restock, packet.can_restock);
}
