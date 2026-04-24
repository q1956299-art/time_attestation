#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map, String, Vec};

#[contract]
pub struct TimeAttestation;

#[contractimpl]
impl TimeAttestation {
    /// Initialize the contract
    pub fn init(env: Env) {
        if env.storage().instance().get::<_, bool>(&"initialized").is_some() {
            panic!("Already initialized");
        }
        env.storage().instance().set(&"initialized", &true);
    }

    /// Log time worked. Stores hours, description, and current ledger timestamp.
    pub fn log_time(env: Env, user: Address, hours: u32, description: String) {
        user.require_auth();

        let mut time_logs: Map<Address, Vec<(u32, String, u64, bool)>> = env
            .storage()
            .instance()
            .get(&"time_logs")
            .unwrap_or(Map::new(&env));

        let timestamp = env.ledger().timestamp();

        let user_logs = time_logs.get(user.clone()).unwrap_or(Vec::new(&env));
        let mut new_logs = Vec::new(&env);
        for i in 0..user_logs.len() {
            new_logs.push_back(user_logs.get(i).unwrap());
        }
        new_logs.push_back((hours, description, timestamp, false));

        time_logs.set(user, new_logs);
        env.storage().instance().set(&"time_logs", &time_logs);
    }

    /// Get all time logs for a user. Returns Vec of (hours, description, timestamp, attested).
    pub fn get_time_logs(env: Env, user: Address) -> Vec<(u32, String, u64, bool)> {
        let time_logs: Map<Address, Vec<(u32, String, u64, bool)>> = env
            .storage()
            .instance()
            .get(&"time_logs")
            .unwrap_or(Map::new(&env));
        time_logs.get(user).unwrap_or(Vec::new(&env))
    }

    /// Get total hours logged for a user (sum of all hours, including attested and unattested).
    pub fn get_total_hours(env: Env, user: Address) -> u32 {
        let time_logs: Map<Address, Vec<(u32, String, u64, bool)>> = env
            .storage()
            .instance()
            .get(&"time_logs")
            .unwrap_or(Map::new(&env));

        let user_logs = time_logs.get(user).unwrap_or(Vec::new(&env));
        let mut total: u32 = 0;
        for i in 0..user_logs.len() {
            let (hours, _, _, _) = user_logs.get(i).unwrap();
            total += hours;
        }
        total
    }

    /// Admin attest a specific time log. Marks the log at log_index as attested.
    pub fn attest_log(env: Env, user: Address, log_index: u32) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&"admin")
            .unwrap_or_else(|| panic!("Admin not set"));
        admin.require_auth();

        let mut time_logs: Map<Address, Vec<(u32, String, u64, bool)>> = env
            .storage()
            .instance()
            .get(&"time_logs")
            .unwrap_or(Map::new(&env));

        let mut user_logs = time_logs.get(user.clone()).unwrap_or(Vec::new(&env));
        let index = log_index as u32;
        if index >= user_logs.len() {
            panic!("Log index out of bounds");
        }

        let mut new_logs = Vec::new(&env);
        for i in 0..user_logs.len() {
            let (hours, desc, ts, attested) = user_logs.get(i).unwrap();
            if i == index {
                new_logs.push_back((hours, desc, ts, true));
            } else {
                new_logs.push_back((hours, desc, ts, attested));
            }
        }

        time_logs.set(user, new_logs);
        env.storage().instance().set(&"time_logs", &time_logs);
    }

    /// Set the admin address (only callable once).
    pub fn set_admin(env: Env, admin: Address) {
        if env.storage().instance().get::<_, bool>(&"admin_set").is_some() {
            panic!("Admin already set");
        }
        admin.require_auth();
        env.storage().instance().set(&"admin", &admin);
        env.storage().instance().set(&"admin_set", &true);
    }
}
