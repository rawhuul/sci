use rand::random;
use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct Utils {}

impl Utils {
    pub async fn send_msg(
        addr: &SocketAddr,
        key: u128,
        msg: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let msg = format!("{:?}:{}", key, msg);

        let mut stream = TcpStream::connect(addr).await?;
        stream.write_all(msg.as_bytes()).await?;
        Ok(())
    }

    pub fn key() -> u128 {
        generate_key()
    }

    pub fn get_full_path(repo: &PathBuf) -> String {
        let absolute_path = env::current_dir()
            .expect("Failed to get current directory")
            .join(&repo);

        match absolute_path.canonicalize() {
            Ok(path) => path.to_string_lossy().replace(r"\\?\", ""),
            Err(_) => format!("{repo:?}"),
        }
    }
}

fn generate_key() -> u128 {
    const SEED: u32 = 0x17012001 ^ 0xDEADBEEF;
    const LFSR_MASK: u32 = 0xB4000001;
    const LFSR_TAPS: u32 = 0x8020001;
    const LFSR_ROUNDS: u32 = 32;

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time went backwards")
        .as_millis();

    let random_number: u32 = random();

    let get_prime_number = |time: u128, random: u32| -> u32 {
        let mut number = (time as u32) ^ random;
        number = number.wrapping_mul(SEED);

        while !(2..number).any(|i| number % i == 0) {
            number = number.wrapping_add(1);
        }

        number
    };

    let lfsr = |seed: u32| -> u32 {
        let mut state = seed;

        for _ in 0..LFSR_ROUNDS {
            let lsb = state & 1;
            state >>= 1;
            if lsb == 1 {
                state ^= LFSR_TAPS;
            }
        }

        state &= LFSR_MASK;

        state
    };

    let prime_number = get_prime_number(current_time, random_number);
    let lfsr_output = lfsr(random_number);

    let result: u128 = (prime_number as u128) << 32 | (lfsr_output as u128);

    result
}
