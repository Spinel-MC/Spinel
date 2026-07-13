use crate::server::{Auth, OnlineAuth};

#[test]
fn online_auth_contains_server_key_pair() {
    let online_auth =
        OnlineAuth::new().expect("online authentication key pair should be generated");
    let auth = Auth::Online(online_auth);
    let Auth::Online(online_auth) = auth else {
        panic!("online auth should retain online state");
    };

    assert!(!online_auth.get_public_key_der().is_empty());
}
