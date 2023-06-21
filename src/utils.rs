use rand::random;
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct Utils {
    addr: SocketAddr,
    key: u128,
}

impl Utils {
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            addr,
            key: generate_key(),
        }
    }

    pub async fn send_msg(&self, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
        let msg = format!("{:?}:{}", self.key, msg);

        let mut stream = TcpStream::connect(self.addr).await?;
        stream.write_all(msg.as_bytes()).await?;
        Ok(())
    }

    pub fn key(&self) -> u128 {
        self.key
    }
}

fn generate_key() -> u128 {
    // Generate a random number
    let random_number: u128 = random();

    // Get the current UNIX timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to retrieve UNIX timestamp")
        .as_secs();

    // Find a large prime number near the timestamp
    let prime_number = find_prime_near_timestamp(timestamp);

    // Perform binary operations with 0x17012001
    (prime_number ^ random_number) & 0x17012001
}

fn find_prime_near_timestamp(timestamp: u64) -> u128 {
    // Start searching for a prime number near the timestamp
    let mut candidate = timestamp as u128;
    while !is_prime(candidate) {
        candidate += 1;
    }

    candidate
}

fn is_prime(n: u128) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}
