#![cfg(test)]
extern crate std;

use super::*;
use crate::events::Tipped;
use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, Events as _},
    Event, IntoVal, Symbol,
};

fn setup() -> (Env, Address, TipJarClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(TipJar, ());
    let client = TipJarClient::new(&env, &contract_id);
    (env, contract_id, client)
}

#[test]
fn all_fixed_deposit_and_tip_happy_path() {
    let (env, _, client) = setup();
    let ama = Address::generate(&env);
    let kofi = Address::generate(&env);
    client.deposit(&ama, &100);
    client.tip(&ama, &kofi, &30);
    assert_eq!(client.balance(&ama), 70);
    assert_eq!(client.balance(&kofi), 30);
}

#[test]
fn bug13_tip_needs_the_tippers_signature() {
    let (env, contract_id, client) = setup();
    let ama = Address::generate(&env);
    let kofi = Address::generate(&env);
    client.deposit(&ama, &100);
    client.tip(&ama, &kofi, &30);
    assert_eq!(
        env.auths(),
        std::vec![(
            ama.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    contract_id.clone(),
                    Symbol::new(&env, "tip"),
                    (ama.clone(), kofi.clone(), 30_i128).into_val(&env),
                )),
                sub_invocations: std::vec![],
            }
        )]
    );
}

#[test]
fn bug14_balance_shows_what_was_deposited() {
    let (env, _, client) = setup();
    let ama = Address::generate(&env);
    assert_eq!(client.balance(&ama), 0);
    client.deposit(&ama, &100);
    assert_eq!(client.balance(&ama), 100);
    client.deposit(&ama, &50);
    assert_eq!(client.balance(&ama), 150);
}

#[test]
fn bug15_cannot_tip_more_than_you_have() {
    let (env, _, client) = setup();
    let ama = Address::generate(&env);
    let kofi = Address::generate(&env);
    client.deposit(&ama, &100);
    let result = client.try_tip(&ama, &kofi, &500);
    assert_eq!(result, Err(Ok(Error::InsufficientBalance)));
}

#[test]
fn bug16_tip_event_names_both_people() {
    let (env, contract_id, client) = setup();
    let ama = Address::generate(&env);
    let kofi = Address::generate(&env);
    client.deposit(&ama, &100);
    client.tip(&ama, &kofi, &30);
    let expected = Tipped {
        from: ama.clone(),
        to: kofi.clone(),
        amount: 30,
    };
    assert_eq!(
        env.events().all(),
        [expected.to_xdr(&env, &contract_id)]
    );
}
