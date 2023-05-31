use anyhow::Result;
use crypto::hashes::{blake2b, Digest};
use iota_streams::app::transport::tangle::client::Client;
use iota_streams::app_channels::api::tangle::{Address, Author, Bytes, ChannelType};
use std::str::FromStr;
use std::time::Duration;
use std::{io, thread};
use std::fs::File;
use std::io::Read;


use rand::{distributions::Uniform, thread_rng, Rng};

#[tokio::main]
async fn main() -> Result<()> {
    let mut input = String::new();

    let node = "https://chrysalis-nodes.iota.org";
    let mut author = Author::new(
        random_seed().as_str(),
        ChannelType::SingleBranch,
        Client::new_from_url(node),
    );

    let ann_link = author.send_announce().await.unwrap();
    println!("Announcement link: {}", ann_link.to_string());

    println!("Ingrese Sub_link:");
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    let sub_link = Address::from_str(input.trim())?;
    author.receive_subscribe(&sub_link).await.unwrap();

    // Send keyload for subscriber
    let keyload_link = author.send_keyload_for_everyone(&ann_link).await.unwrap();

    // // Send signed message from author
    let mut prev_link = keyload_link;
    let mut i = 1;
    loop {
        println!("Enviando mensaje...");
        let texto = "mensaje numero: ".to_owned() + &i.to_string();
        let public_payload = create_payload(&texto);

        //AQUI, LEER .TXT QUE TENDRÁ TEMPERATURAS Y PASAR COMO ARGUNMENTOS
        let mut file = File::open("archivo.txt").expect("Error al abrir el archivo");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Error al leer el archivo");

        let mask_texto = "temperatura: ".to_owned() + content.trim();
        let masked_payload = create_payload(&mask_texto);
        let new_link = author
            .send_signed_packet(&prev_link.0, &public_payload, &masked_payload)
            .await
            .unwrap();
        prev_link = new_link.clone();
        i = i + 1;
        content.clear();
        thread::sleep(Duration::from_secs(10));
    }

    Ok(())
}

pub fn create_payload(payload: &str) -> Bytes {
    Bytes(payload.as_bytes().to_vec())
}

pub fn get_hash(link: &Address) -> String {
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




