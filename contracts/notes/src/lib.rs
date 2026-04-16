#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, 
    token, Address, Env, String, Symbol, Vec
};

// Struktur data untuk menyimpan riwayat tip
#[contracttype]
#[derive(Clone, Debug)]
pub struct TipEntry {
    pub donor: Address,
    pub amount: i128,
    pub message: String,
}

// Key storage untuk data permanen
const TIP_HISTORY: Symbol = symbol_short!("HISTORY");
const TOTAL_COLLECTED: Symbol = symbol_short!("TOTAL");

#[contract]
pub struct TipJarContract;

#[contractimpl]
impl TipJarContract {
    /// Fungsi untuk mengirim tip
    /// - `donor`: Alamat pengirim (terkoneksi ke Freighter)
    /// - `token_address`: Alamat kontrak token (misal: Alamat Native XLM)
    /// - `amount`: Jumlah token (dalam unit stroops)
    /// - `message`: Pesan singkat untuk penerima
    pub fn send_tip(env: Env, donor: Address, token_address: Address, amount: i128, message: String) {
        // 1. Pastikan donor adalah pemilik transaksi ini (verifikasi Freighter)
        donor.require_auth();

        // 2. Inisialisasi client token menggunakan alamat token yang dikirim dari frontend
        let token_client = token::Client::new(&env, &token_address);
        
        // Pindahkan dana dari donor ke alamat kontrak ini
        token_client.transfer(&donor, &env.current_contract_address(), &amount);

        // 3. Update riwayat tip
        let mut history: Vec<TipEntry> = env.storage().instance().get(&TIP_HISTORY).unwrap_or(Vec::new(&env));
        let new_entry = TipEntry {
            donor: donor.clone(),
            amount: amount,
            message: message,
        };
        history.push_back(new_entry);
        env.storage().instance().set(&TIP_HISTORY, &history);

        // 4. Update total saldo terkumpul
        let current_total: i128 = env.storage().instance().get(&TOTAL_COLLECTED).unwrap_or(0);
        env.storage().instance().set(&TOTAL_COLLECTED, &(current_total + amount));
    }

    /// Fungsi untuk melihat semua orang yang sudah kasih tip (Leaderboard)
    pub fn get_history(env: Env) -> Vec<TipEntry> {
        env.storage().instance().get(&TIP_HISTORY).unwrap_or(Vec::new(&env))
    }

    /// Fungsi untuk melihat total dana terkumpul
    pub fn get_total(env: Env) -> i128 {
        env.storage().instance().get(&TOTAL_COLLECTED).unwrap_or(0)
    }
}

mod test;