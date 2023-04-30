use anyhow::Result;
use iota_streams::app_channels::api::tangle::{Author, ChannelType, Address, Bytes};
use iota_streams::app::transport::tangle::client::Client;
use std::str::FromStr;
use std::io;


#[tokio::main]
async fn main() -> Result<()> {
    let mut input = String::new();

    let node = "https://chrysalis-nodes.iota.org";
    let mut author = Author::new("test26", ChannelType::SingleBranch, Client::new_from_url(node));
    
    let ann_link = author.send_announce().await.unwrap();   
    println!("Announcement link: {}", ann_link.to_string());

    println!("Ingrese Sub_link:");
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let sub_link = Address::from_str(input.trim())?;
    author.receive_subscribe(&sub_link).await.unwrap();

    // Send keyload for subscriber
    let keyload_link = author.send_keyload_for_everyone(&ann_link).await.unwrap();

    // // Send signed message from author
    let public_payload = create_payload("esto es un troll");
    let masked_payload = create_payload("estamos a 100 grados, estamos hirviendo");
    let signed_message_link = author.send_signed_packet(
        &keyload_link.0, 
        &public_payload, 
        &masked_payload
    ).await.unwrap();

    Ok(())
}

pub fn create_payload(payload: &str) -> Bytes {
    Bytes(payload.as_bytes().to_vec())
}