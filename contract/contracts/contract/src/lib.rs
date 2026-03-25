#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Vec, Address};

// Struct ứng viên
#[derive(Clone)]
#[contracttype]
pub struct Candidate {
    pub name: Symbol,
    pub votes: u32,
}

#[contract]
pub struct ScholarshipDAO;

#[contractimpl]
impl ScholarshipDAO {

    // Thêm ứng viên
    pub fn add_candidate(env: Env, name: Symbol) {
        let mut candidates: Vec<Candidate> =
            env.storage().instance().get(&Symbol::short("CANDS"))
            .unwrap_or(Vec::new(&env));

        // tránh trùng tên
        for c in candidates.iter() {
            if c.name == name {
                panic!("Candidate already exists");
            }
        }

        candidates.push_back(Candidate {
            name,
            votes: 0,
        });

        env.storage().instance().set(&Symbol::short("CANDS"), &candidates);
    }

    // Vote (mỗi address chỉ vote 1 lần)
    pub fn vote(env: Env, voter: Address, name: Symbol) {
        voter.require_auth();

        let voted_key = (Symbol::short("VOTED"), voter.clone());

        // check đã vote chưa
        if env.storage().instance().has(&voted_key) {
            panic!("Already voted");
        }

        let mut candidates: Vec<Candidate> =
            env.storage().instance().get(&Symbol::short("CANDS"))
            .unwrap_or(Vec::new(&env));

        let mut found = false;

        // tăng vote đúng cách
        for i in 0..candidates.len() {
            let mut c = candidates.get(i).unwrap();
            if c.name == name {
                c.votes += 1;
                candidates.set(i, c);
                found = true;
                break;
            }
        }

        if !found {
            panic!("Candidate not found");
        }

        // đánh dấu đã vote
        env.storage().instance().set(&voted_key, &true);

        env.storage().instance().set(&Symbol::short("CANDS"), &candidates);
    }

    // Xem danh sách
    pub fn get_candidates(env: Env) -> Vec<Candidate> {
        env.storage().instance().get(&Symbol::short("CANDS"))
            .unwrap_or(Vec::new(&env))
    }

    // Lấy winner
    pub fn get_winner(env: Env) -> Symbol {
        let candidates: Vec<Candidate> =
            env.storage().instance().get(&Symbol::short("CANDS"))
            .unwrap_or(Vec::new(&env));

        if candidates.len() == 0 {
            return Symbol::short("NONE");
        }

        let mut max_votes = 0;
        let mut winner = candidates.get(0).unwrap().name;

        for c in candidates.iter() {
            if c.votes > max_votes {
                max_votes = c.votes;
                winner = c.name.clone();
            }
        }

        winner
    }
}