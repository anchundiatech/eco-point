use crate::{EcoError, EcoPuntos, EcoPuntosClient};
use soroban_sdk::{testutils::Address as _, Address, Env};

fn setup() -> (Env, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let participant = Address::generate(&env);
    let contract_id = env.register(EcoPuntos, ());
    (env, contract_id, admin, participant)
}

#[test]
fn initializes_and_stores_admin() {
    let (env, contract_id, admin, _participant) = setup();
    let client = EcoPuntosClient::new(&env, &contract_id);
    client.initialize(&admin);
    assert_eq!(client.get_admin(), admin);
}

#[test]
fn records_delivery_and_awards_points() {
    let (env, contract_id, admin, participant) = setup();
    let client = EcoPuntosClient::new(&env, &contract_id);
    client.initialize(&admin);

    let awarded = client.record_delivery(&participant, &5);
    assert_eq!(awarded, 50);
    assert_eq!(client.get_points(&participant), 50);
    assert_eq!(client.get_total_kg(&participant), 5);
    assert_eq!(client.get_delivery_count(&participant), 1);

    let delivery = client.get_delivery(&participant, &0).unwrap();
    assert_eq!(delivery.kilograms, 5);
    assert_eq!(delivery.points, 50);
}

#[test]
fn accumulates_multiple_deliveries() {
    let (env, contract_id, admin, participant) = setup();
    let client = EcoPuntosClient::new(&env, &contract_id);
    client.initialize(&admin);

    assert_eq!(client.record_delivery(&participant, &2), 20);
    assert_eq!(client.record_delivery(&participant, &3), 30);
    assert_eq!(client.get_points(&participant), 50);
    assert_eq!(client.get_total_kg(&participant), 5);
    assert_eq!(client.get_delivery_count(&participant), 2);
}

#[test]
fn rejects_zero_kilograms() {
    let (env, contract_id, admin, participant) = setup();
    let client = EcoPuntosClient::new(&env, &contract_id);
    client.initialize(&admin);

    assert_eq!(
        client.try_record_delivery(&participant, &0),
        Err(Ok(EcoError::InvalidAmount))
    );
}

#[test]
fn rejects_double_initialization() {
    let (env, contract_id, admin, _participant) = setup();
    let client = EcoPuntosClient::new(&env, &contract_id);
    client.initialize(&admin);

    assert_eq!(
        client.try_initialize(&admin),
        Err(Ok(EcoError::AlreadyInitialized))
    );
}

#[test]
fn rejects_recording_before_initialization() {
    let (env, contract_id, _admin, participant) = setup();
    let client = EcoPuntosClient::new(&env, &contract_id);

    assert_eq!(
        client.try_record_delivery(&participant, &1),
        Err(Ok(EcoError::NotInitialized))
    );
}
