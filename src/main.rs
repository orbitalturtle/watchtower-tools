use plugin::net::http;
use std::env;
use teos_common::net::NetAddr;
use teos_common::test_utils::get_random_user_id;

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
            "add_appointment" => add_appointment(),
            _ => println!("ERROR: Unknown command. Please use 'register' or 'add_appointment'."),
        }
    } else {
        println!("ERROR: No command provided. Please specify 'register' or 'add_appointment'.");
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

// Placeholder function for the "add_appointment" command.
fn add_appointment() {
    println!("Adding an appointment...");
    // Implement the add_appointment logic here
}