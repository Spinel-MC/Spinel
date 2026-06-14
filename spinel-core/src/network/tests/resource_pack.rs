use super::super::resource_pack::{ResourcePackInfo, ResourcePackStatus};
use spinel_network::DataType;
use uuid::Uuid;

#[test]
fn resource_pack_status_ids_match_minestom_client_status_ids() {
    assert_eq!(ResourcePackStatus::SuccessfullyLoaded.id(), 0);
    assert_eq!(ResourcePackStatus::Declined.id(), 1);
    assert_eq!(ResourcePackStatus::FailedDownload.id(), 2);
    assert_eq!(ResourcePackStatus::Accepted.id(), 3);
    assert_eq!(ResourcePackStatus::Downloaded.id(), 4);
    assert_eq!(ResourcePackStatus::InvalidUrl.id(), 5);
    assert_eq!(ResourcePackStatus::FailedReload.id(), 6);
    assert_eq!(ResourcePackStatus::Discarded.id(), 7);
}

#[test]
fn resource_pack_status_codec_rejects_unknown_status_ids() {
    let mut payload = Vec::new();
    spinel_network::VarIntWrapper(8)
        .encode(&mut payload)
        .unwrap();

    let error = ResourcePackStatus::decode(&mut payload.as_slice()).unwrap_err();

    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
}

#[test]
fn resource_pack_info_preserves_request_identity_url_and_hash() {
    let id = Uuid::from_u128(1);
    let resource_pack = ResourcePackInfo::new(id, "url", "hash");

    assert_eq!(resource_pack.id(), id);
    assert_eq!(resource_pack.url(), "url");
    assert_eq!(resource_pack.hash(), "hash");
}
