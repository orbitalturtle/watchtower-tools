use bitcoin::secp256k1::SecretKey;
use plugin::net::http;
use std::env;
use std::str::FromStr;
use teos_common::net::NetAddr;
use teos_common::cryptography;
use teos_common::test_utils::{generate_random_appointment, get_random_user_id};
use tokio::{select, time, time::Duration};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    simple_logger::SimpleLogger::new().env().init().unwrap();

     // Check if at least one argument is provided.
     if args.len() > 1 {
        match args[1].as_str() {
            "register" => {
                if args.len() == 4 {
                    let tower_id = args[2].clone();
                    let address = args[3].clone();
                    register(tower_id, address).await
                } else {
                    println!("ERROR: Usage for register: register <tower_id> <address>")
                }
            },
            "spam_appointments" => {
                if args.len() == 4 {
                    let address = args[2].clone();
                    let sk = SecretKey::from_str(&args[3].clone()).unwrap();
                    spam_appointments(address, sk).await;
                } else {
                    println!("ERROR: Usage for spam_appointments: spam_appointments <address> <sk>")
                }
            },
            _ => println!("ERROR: Unknown command. Please use 'register' or 'spam_appointments'."),
        }
    } else {
        println!("ERROR: No command provided. Please specify 'register' or 'spam_appointments'.");
    }
}

// Registers a user with a tower.
async fn register(_tower_id: String, addr: String) {
    println!("Registering...");

    let tower_id = get_random_user_id();
    let tower_net_addr = NetAddr::new(format!("{addr}"));

    let user_id = get_random_user_id();
    
    match http::register(tower_id, user_id, &tower_net_addr, &None).await {
            Ok(_) => println!("SUCCESSFULLY registered with tower"),
            Err(e) => println!("ERROR registering with tower {e:?}"),
    };
}

// Placeholder function for the "spam_appointments" command.
async fn spam_appointments(addr: String, sk: SecretKey) {
    println!("Adding an appointment...");

    let tower_id = get_random_user_id();
    let tower_net_addr = NetAddr::new(format!("{addr}"));

    let appointment = generate_random_appointment(None);
    let signature = cryptography::sign(
        &appointment.to_vec(),
        &sk,
    )
    .unwrap();

    let mut interval = time::interval(Duration::from_secs(1));
    loop {
        select!(
            _ = interval.tick() => {
                match http::add_appointment(tower_id, &tower_net_addr, &None, &appointment, &signature).await {
                    Ok(_) => println!("SUCCESS adding appointment"),
                    Err(e) => println!("ERROR adding appointment: {e:?}"),
                };
            }
        )
    }
}