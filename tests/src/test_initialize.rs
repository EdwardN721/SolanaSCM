use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey, signature::read_keypair_file,
        signer::Signer,
    },
    Client, Cluster,
};
use solana_scm::accounts; 
use solana_scm::instruction;
use std::str::FromStr;
use std::collections::HashMap;


/*
#[test]
fn test_create_registry() {
    // Dirección del programa en Solana
    let program_id = Pubkey::from_str("A5i8uPKdCycDG3nbGCCAUiLzHEc4ddpfeYGQhPEWuaTJ").unwrap();

    // Leer el archivo de la clave privada del pagador
    let anchor_wallet = std::env::var("ANCHOR_WALLET").expect("ANCHOR_WALLET env var no encontrada");
    let payer = read_keypair_file(&anchor_wallet).expect("Error al leer el keypair");

    // Crear el cliente de Solana (en Localnet)
    let client = Client::new_with_options(Cluster::Devnet, &payer, CommitmentConfig::confirmed());

    // Acceder al programa en la red de Solana y desempaquetar el `Result`
    let program = client.program(program_id).expect("Error al acceder al programa");

    // Generar una clave pública única para el registro
    let registry_pubkey = Pubkey::new_unique();

    // Ejecutar la transacción para crear un registro
    let tx = program
        .request()
        .accounts(accounts::CreateRegistry {
            registry: registry_pubkey,
            user: payer.pubkey(),
            system_program: system_program::ID,
        })
        .args(instruction::CreateRegistry { 
            name: "Registro 1".to_string()  // Nombre del registro
        })
        .send()
        .expect("Error al enviar la transacción para crear el registro");

    println!("Transacción de creación de registro: {}", tx);

    // Aquí podrías incluir alguna verificación, por ejemplo, obtener la cuenta `registry`
    let registry_account: solana_scm::Registry = program
        .account(registry_pubkey)
        .expect("Error al obtener la cuenta de registro");

    assert_eq!(registry_account.device_count, 0);
    println!("Se creó el registro con éxito: {:?}", registry_account);
}
*/

#[test]
fn test_add_device() {
    // Dirección del programa en Solana
    let program_id = Pubkey::from_str("A5i8uPKdCycDG3nbGCCAUiLzHEc4ddpfeYGQhPEWuaTJ").unwrap();

    // Leer el archivo de la clave privada del pagador
    let anchor_wallet = std::env::var("ANCHOR_WALLET").expect("ANCHOR_WALLET env var no encontrada");
    let payer = read_keypair_file(&anchor_wallet).expect("Error al leer el keypair");

    // Crear el cliente de Solana (en Devnet)
    let client = Client::new_with_options(Cluster::Devnet, &payer, CommitmentConfig::confirmed());

    // Acceder al programa en la red de Solana y desempaquetar el `Result`
    let program = client.program(program_id).expect("Error al acceder al programa");

    // Generar claves públicas únicas para el registro y el dispositivo
    let registry_pubkey = Pubkey::new_unique();
    let device_pubkey = Pubkey::new_unique();

    // Ejecutar la transacción para crear un registro
    program
        .request()
        .accounts(accounts::CreateRegistry {
            registry: registry_pubkey,  
            user: payer.pubkey(),        
            system_program: system_program::ID, 
        })
        .args(instruction::CreateRegistry { 
            name: "Registro 1".to_string()
        })
        .send()
        .expect("Error al enviar la transacción para crear el registro");

    // Ejecutar la transacción para agregar un dispositivo
    let tx = program
        .request()
        .accounts(accounts::AddDevice {
            device: device_pubkey,
            registry: registry_pubkey, 
            user: payer.pubkey(), 
            system_program: system_program::ID,
        })
        .args(instruction::AddDevice {
            name: "Sensor Temperatura".to_string(),
            description: "Dispositivo de sensor de temperatura".to_string(),
        })
        .send()
        .expect("Error al enviar la transacción para agregar el dispositivo");

    println!("Transacción de adición de dispositivo: {}", tx);

    // Verificar que el dispositivo se haya agregado al registro
    let registry_account: solana_scm::Registry = program
        .account(registry_pubkey)
        .expect("Error al obtener la cuenta de registro");

    assert_eq!(registry_account.device_count, 1);
    assert_eq!(registry_account.device_ids.len(), 1);
    assert_eq!(registry_account.device_ids[0], device_pubkey);
    println!("El dispositivo se agregó correctamente al registro: {:?}", registry_account);
}
