use nostr::prelude::*;
use nostr_sdk::secp256k1::hashes::{sha256, Hash};
use nostr_sdk::secp256k1::schnorr::Signature;
use nostr_sdk::secp256k1::Message;
use nostr_sdk::secp256k1::Secp256k1;
use nostr_sdk::Client;
use nostr_sdk::NostrSigner;

#[flutter_rust_bridge::frb(sync)]
pub fn keys(hex: String) -> (String, String) {
    let keys = Keys::parse(hex).unwrap();
    (
        keys.secret_key().to_secret_hex(),
        keys.public_key().to_hex(),
    )
}

pub async fn encode_nip17(
    sender_secret: String,
    receiver_public: String,
    message: String,
) -> String {
    let sender_keys = Keys::parse(sender_secret).unwrap();
    let sender_client = Client::new(sender_keys);
    let receiver = nostr_sdk::PublicKey::parse(receiver_public).unwrap();

    let signer: NostrSigner = sender_client.signer().await.unwrap();
    let public_key: PublicKey = signer.public_key().await.unwrap();

    // Create a rumor
    let rumor: UnsignedEvent =
        EventBuilder::private_msg_rumor(receiver, message, None).to_unsigned_event(public_key);

    // Compose seal
    let content: String = signer
        .nip44_encrypt(&receiver, rumor.as_json())
        .await
        .unwrap();

    let seal: EventBuilder = EventBuilder::new(Kind::Seal, content, [])
        .custom_created_at(Timestamp::tweaked(nip59::RANGE_RANDOM_TIMESTAMP_TWEAK));

    let seal: Event = sender_client.sign_event_builder(seal).await.unwrap();

    // Compose gift wrap
    let gift_wrap = EventBuilder::gift_wrap_from_seal(&receiver, &seal, None).unwrap();

    return gift_wrap.as_json();
}

pub async fn decode_nip17(
    receiver_secret_key: String,
    event_json: String,
) -> Option<(String, String)> {
    let receiver_keys = Keys::parse(receiver_secret_key).unwrap();
    let client = Client::new(receiver_keys);
    let event = Event::from_json(event_json).unwrap();

    match client.unwrap_gift_wrap(&event).await {
        Ok(UnwrappedGift { rumor, sender }) => {
            if rumor.kind == Kind::PrivateDirectMessage {
                return Some((sender.to_hex(), rumor.content));
            }
            return None;
        }
        Err(e) => {
            eprintln!("Failed to unwrap gift: {:?}", e);
            return None;
        }
    }
}

pub async fn send_nip17(
    sender_secret_key: String,
    receiver_public_key: String,
    message: String,
    relay: String,
) -> String {
    let sender_keys = Keys::parse(sender_secret_key).unwrap();
    let client = Client::new(sender_keys);
    client.add_relay(relay).await.unwrap();
    client.connect().await;

    let receiver_pubkey = nostr_sdk::PublicKey::parse(receiver_public_key).unwrap();
    let send_private_msg = client
        .send_private_msg(receiver_pubkey, message, None)
        .await;
    return send_private_msg.unwrap().to_hex();
}

#[flutter_rust_bridge::frb(sync)]
pub fn sign(signer_secret_key: Vec<u8>, message: Vec<u8>) -> Vec<u8> {
    let secp = Secp256k1::new();
    let keys = nostr_sdk::secp256k1::Keypair::from_seckey_slice(&secp, &signer_secret_key)
        .expect("32 bytes, within curve order");

    let digest = sha256::Hash::hash(&message);
    let message = Message::from_digest(digest.to_byte_array());

    let sig = secp.sign_schnorr(&message, &keys);
    return sig.serialize().to_vec();
}

#[flutter_rust_bridge::frb(sync)]
pub fn verify(signer_public_key: Vec<u8>, message: Vec<u8>, signature: Vec<u8>) -> bool {
    let public_key =
        nostr_sdk::PublicKey::from_slice(&signer_public_key).expect("32 bytes, within curve order");

    let signature = Signature::from_slice(&signature).unwrap();

    let digest = sha256::Hash::hash(&message);
    let message = Message::from_digest(digest.to_byte_array());

    let secp = Secp256k1::new();
    let sig = secp.verify_schnorr(&signature, &message, &public_key);
    return sig.is_ok();
}
