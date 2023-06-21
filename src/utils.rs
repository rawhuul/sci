use md5::{compute, Digest};
use rand::Rng;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct Utils {
    addr: SocketAddr,
    key: Digest,
}

impl Utils {
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            addr,
            key: generate_header_key(100000, 1000000),
        }
    }

    pub async fn send_msg(&self, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
        let msg = format!("{:?}:{}", self.key, msg);

        let mut stream = TcpStream::connect(self.addr).await?;
        stream.write_all(msg.as_bytes()).await?;
        Ok(())
    }

    pub fn key(&self) -> Digest {
        self.key
    }
}

pub fn generate_header_key(min: usize, max: usize) -> Digest {
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(min..max);
    let key = random ^ 0x17012001;

    compute(format!("{key}"))
}
