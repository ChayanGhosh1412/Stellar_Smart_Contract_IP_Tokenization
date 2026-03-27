#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short, Map, String};

#[contract]
pub struct IPTokenizationContract;

#[contractimpl]
impl IPTokenizationContract {

    // Store IP metadata: id -> owner
    pub fn register_ip(env: Env, ip_id: Symbol, owner: String) {
        let mut ip_map: Map<Symbol, String> = env.storage().instance().get(&symbol_short!("IPS")).unwrap_or(Map::new(&env));
        ip_map.set(ip_id.clone(), owner);
        env.storage().instance().set(&symbol_short!("IPS"), &ip_map);
    }

    // Get owner of IP
    pub fn get_owner(env: Env, ip_id: Symbol) -> String {
        let ip_map: Map<Symbol, String> = env.storage().instance().get(&symbol_short!("IPS")).unwrap();
        ip_map.get(ip_id).unwrap()
    }

    // Transfer ownership
    pub fn transfer_ip(env: Env, ip_id: Symbol, new_owner: String) {
        let mut ip_map: Map<Symbol, String> = env.storage().instance().get(&symbol_short!("IPS")).unwrap();
        ip_map.set(ip_id, new_owner);
        env.storage().instance().set(&symbol_short!("IPS"), &ip_map);
    }
}