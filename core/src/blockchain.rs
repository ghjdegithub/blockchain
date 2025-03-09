use crate::bcdb::BlockChainDb;
use crate::block::Block;
use crate::pow::ProofOfWork;
use crate::transaction::Transaction;
use bigint::U256;
use leveldb::database::Database;
use std::collections::HashMap;
use std::sync::Mutex;
use utils::bkey;
use utils::bkey::BKey;
use utils::serializer::{hash_str, hash_u8, serialize};

const INIT_BITS: u32 = 0x2100FFFF;
const SAVE_DIR: &str = "bc_db";
// 创世区块 pre_hash
const PRE_HASH: &str = "22caaf24ef0aea3522c13d133912d2b722caaf24ef0aea3522c13d133912d2b7";

// 区块链
pub struct BlockChain {
    blocks_db: Box<Database<BKey>>,
    blocks_index: Mutex<HashMap<String, Block>>,
    pub gens_hash: String,
    pub curr_hash: String,
    pub curr_bits: u32,
}

impl BlockChain {
    pub fn new() -> Self {
        let mut db = BlockChainDb::new(SAVE_DIR);
        let genesis = Self::genesis_block();
        Self::write_block(&mut db, &genesis);
        Self::write_tail(&mut db, &genesis);
        println!("New produced block saved!\n");

        let gene_block = genesis.clone();
        let mut block_index = Mutex::new(HashMap::new());
        Self::update_hmap(&mut block_index, gene_block);

        let gens_hash = genesis.hash.clone();
        let curr_hash = genesis.hash.clone();
        BlockChain {
            blocks_db: Box::new(db),
            blocks_index: block_index,
            gens_hash,
            curr_hash,
            curr_bits: INIT_BITS,
        }
    }

    // 生成创世区块
    fn genesis_block() -> Block {
        println!("Start mining ... ");
        let from = "0x0000".to_string();
        let to = "0x0000".to_string();
        let sign = "创世区块".to_string();
        let tx = Transaction::new(from, to, 0, 0, 0, sign);
        let mut block = Block::new(vec![tx], PRE_HASH.to_string(), INIT_BITS);

        let header_ser = ProofOfWork::prepare_data(&mut block, 0);
        block.hash = hash_str(&header_ser);
        println!("Produced a new block!");

        block
    }

    // 添加区块，形成区块链
    pub fn add_block(&mut self, block: Block) {
        Self::write_block(&mut (self.blocks_db), &block);
        Self::write_tail(&mut (self.blocks_db), &block);
        println!("New produced block saved!\n");
        self.curr_hash = block.hash.clone();
        self.curr_bits = block.header.bits.clone();
        Self::update_hmap(&mut self.blocks_index, block);
    }

    fn update_hmap(hmap: &mut Mutex<HashMap<String, Block>>, block: Block) {
        let mut hmap = hmap.lock().unwrap();
        let hash = block.hash.clone();
        hmap.insert(hash, block);
    }

    fn write_block(db: &mut Database<BKey>, block: &Block) {
        // 基于区块的 header 生成 key
        let header_ser = serialize(&block.header);
        let mut hash_u: [u8; 32] = [0; 32];
        hash_u8(&header_ser, &mut hash_u);

        let key = BKey {
            val: U256::from(hash_u),
        };
        let val = serialize(&block);
        BlockChainDb::write_db(db, key, &val);
    }

    // 将区块哈希值作为尾巴写入
    fn write_tail(db: &mut Database<bkey::BKey>, block: &Block) {
        let key = BKey {
            val: U256::from("tail".as_bytes()),
        };
        let val = serialize(&(block.hash));
        BlockChainDb::write_db(db, key, &val);
    }

    // 打印区块信息
    pub fn block_info(&self) {
        let mut hash = self.curr_hash.clone();
        let hmap = self.blocks_index.lock().unwrap();
        let mut blocks: Vec<Block> = Vec::new();

        loop {
            if let Some(b) = hmap.get(&hash) {
                blocks.push(b.clone());
                hash = b.header.pre_hash.clone();
            } else {
                panic!("Error getting block");
            }

            if hash == self.gens_hash {
                if let Some(b) = hmap.get(&hash) {
                    blocks.push(b.clone());
                }
                break;
            }
        }
        blocks.reverse();

        for b in blocks.iter() {
            println!("{:#?}", b);
        }
    }
}
