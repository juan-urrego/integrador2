use anyhow::Result;
use iota_streams::app_channels::api::tangle::{Transport, MessageContent, Address, Subscriber};
use iota_streams::app::transport::tangle::client::Client;
use std::str::FromStr;
use std::{io, thread};
use std::time::Duration;
use crypto::hashes::{Digest, blake2b};

use rand::{
    distributions::Uniform,
    Rng,
    thread_rng
};


#[tokio::main]
async fn main() -> Result<()> {
    let mut input = String::new();
    let mut input2 = String::new();

    let node = "https://chrysalis-nodes.iota.org";
    let mut subscriber = Subscriber::new(random_seed().as_str(), Client::new_from_url(node));
    
    println!("Ingrese Ann_link:");
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let ann_link = Address::from_str(input.trim())?;
    subscriber.receive_announcement(&ann_link).await.unwrap();

    // Send subscription message
    let sub_link = subscriber.send_subscribe(&ann_link).await.unwrap();
    println!("Subscription link: {}", sub_link.to_string());

    //Receive signed message
    loop {
        receive_messages_for_subscriber(&mut subscriber, "Subscriber").await;  
        thread::sleep(Duration::from_secs(5));
    }

    Ok(())
}

pub async fn receive_messages_for_subscriber<T: Transport>(subscriber: &mut Subscriber<T>, tag: &str) {
    println!("");
    println!("Receiving messages for {}...", tag);
    loop {
        let messages = subscriber.fetch_next_msgs().await;
        if messages.is_empty() { break; }

        for message in messages {
            println!("Message Index: {}", get_hash(&message.link));
            println!("Message ID: {}", message.link.msgid);
            match message.body {
                MessageContent::Unreadable => {
                    println!("Received an unreadable message: ");
                    // Or you can use a logging crate, like `log::error!` or `slog::error!`
                },
                MessageContent::Announce => {
                    println!("    Announcement");
                },
                MessageContent::Keyload => {
                    println!("    Keyload");
                },
                MessageContent::Sequence => {
                    println!("    Sequence");
                },
                MessageContent::SignedPacket { pk: _, public_payload, masked_payload } => {
                    println!("    Public Message: {}", String::from_utf8(public_payload.0).unwrap());
                    println!("    Masked Message: {}", String::from_utf8(masked_payload.0).unwrap());
                },
                MessageContent::Subscribe => {
                    println!("    Subscription");
                },
                MessageContent::TaggedPacket { public_payload, masked_payload } => {
                    println!("    Public Message: {}", String::from_utf8(public_payload.0).unwrap());
                    println!("    Masked Message: {}", String::from_utf8(masked_payload.0).unwrap());
                },
                MessageContent::Unsubscribe => {
                    println!("    Unsubscribtion");
                },
            }
        }
    }
}

pub fn get_hash(link: &Address) ->  String {
    let total = [link.appinst.as_ref(), link.msgid.as_ref()].concat();
    let hash = blake2b::Blake2b256::digest(&total);
    hex::encode(&hash)
}


pub fn random_seed() -> String {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ9".as_bytes();
    thread_rng()
        .sample_iter(Uniform::new(0, alphabet.len()))
        .take(81)
        .map(|i| alphabet[i] as char)
        .collect()
}