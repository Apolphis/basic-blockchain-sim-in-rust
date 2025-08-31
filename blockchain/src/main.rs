use sha2::{Digest, Sha256};
use std::fmt::{self, format, write};
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time::Duration;

const DIFFICULTY: usize = 2;

struct Block {
    index: u32,
    previous_hash:  String,
    timestamp: u64,
    data: String,
    nonce: u64,
    hash: String,
}

impl Block {
    fn new(index: u32, previous_hash: String, data: String) -> Block {
        let timestamp = SystemTime::now().duration_since
        (UNIX_EPOCH).expect( "Time went backwards").as_secs();
        Block {
            index,
            previous_hash,
            timestamp,
            data,
            nonce:0 ,
            hash: String::new(),
        }
    }

    fn calculate_hash(&self) -> String {
    let data = format!(
        "{}{}{}{}{}",
        self.index,
        &self.previous_hash,
        self.timestamp,
        &self.data,
        self.nonce
    );

    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();

    let hash_str = format!("{:x}", result);
    hash_str
    }

    fn mine_block_with_visual_effects(&mut self) {
        let mut interations = 0;
        loop {
            self.hash = self.calculate_hash();
            interations += 1;

            if !self.hash.is_empty() && &self.hash[..DIFFICULTY] == "00" .repeat(DIFFICULTY) {
                println!("\u{26CF} Block mined: {}", self.index);
                break;
            }

            if interations > 100 {
                print!("\u{23F2} Mining in progress...");
                thread::sleep(Duration::from_millis(3000));

                println!("Calculated hash: {}", self.hash);
                break;
            }

            self.nonce += 1;
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datetime = chrono::NaiveDateTime::from_timestamp(self.timestamp as i64, 0);

        write!(
            f,
            "Block {}: {} at {}",
            self.index, self.data, datetime
        )
    }
}

struct Blockchain {
    chain: Vec<Block>
}

impl Blockchain {
    fn new() -> Blockchain {
        let genesis_block = Block::new(0,
        String::new(), String::from("Genesis Block"));

        Blockchain { chain: vec![genesis_block],
         }
    }

    fn add_block(&mut self, mut new_block: Block) {
        let previos_hash = self.chain.last().unwrap().hash.clone();
        new_block.previous_hash = previos_hash;
        new_block.mine_block_with_visual_effects();

        self.chain.push(new_block);
    }

    fn get_total_blocks(&self) -> usize {
        self.chain.len()
    }
}

fn main() {
    println!("\u{1F680} Welcome to the Blockchain Simulator! \u{1F680}");

    println!("\u{1F477} Please enter your miner name: ");

    let mut miner_name = String::new();

    std::io::stdin()
    .read_line(&mut miner_name)
    .expect("Failed to read input");

    miner_name = miner_name.trim().to_string();

    let trader_names = vec!["Tim", "Bob", "Phil", "Zorro", "Faulty Towers", "Judean Peoples front","Peoples front of Judea"];

    let mut simcoin = Blockchain::new();

    println!("\n \u{26CF} Let's start mining and simulating some transactions! \n");

    let mut sender = miner_name.clone();

    for i in 0..trader_names.len() {
        println!("\u{1F9F1}  Mining Block {}...\u{26CF}", i+1);
        let recipient = if i < trader_names.len() - 1 {
            trader_names[i+1].to_string()
        } else {
            miner_name.clone()
        };

        let transaction = format!("{} sent to {}", sender, recipient);

        let new_block = Block::new((i + 1) as u32, String::new(), transaction.clone());

        simcoin.add_block(new_block);

        println!("\u{1F4E7} Transaction: {}", transaction);

        sender = recipient;

        println!();
    }

    let total_blocks = simcoin.get_total_blocks();

    println!("\u{2611} Total blocks added to the blockchain: {}", total_blocks);

    let simcoin_per_block: usize = 137;

    let simcoin_traded = total_blocks * simcoin_per_block;

    println!("\u{1F4B0} Total Simcoin traded: {} Simcoin", simcoin_traded);

    let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();

    let end_datetime = chrono::NaiveDateTime::from_timestamp(end_timestamp as i64, 0);

    println!("\u{23F0} Simulation ended at: {}", end_datetime);

    println!("\u{1F389} Congrats! Mining operation completed successfully!");
}


