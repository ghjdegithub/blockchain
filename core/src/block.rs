use crate::transaction::Transaction;
use chrono::Utc;
use serde::Serialize;
use utils::serializer::{hash_str, serialize};

// 区块头
#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct BlockHeader {
    pub nonce: u32,
    pub time: i64,
    pub bits: u32,
    pub txs_hash: String,
    pub pre_hash: String,
}

// 区块
#[derive(Serialize, Debug, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub tranxs: Vec<Transaction>, // 改成了具体交易
    pub hash: String,
}

impl Block {
    pub fn new(txs: Vec<Transaction>, pre_hash: String, bits: u32) -> Self {
        let time = Utc::now().timestamp();
        let txs_hash = Self::merkle_hash_str(&txs);

        Block {
            header: BlockHeader {
                nonce: 0,
                time,
                bits,
                txs_hash,
                pre_hash,
            },
            tranxs: txs,
            hash: "".to_string(),
        }
    }

    // 计算梅根哈希
    fn merkle_hash_str(txs: &Vec<Transaction>) -> String {
        if txs.is_empty() {
            return "00000000".to_string();
        }

        // 哈希值收集
        let mut merkle_tree: Vec<String> = Vec::new();
        for tx in txs {
            merkle_tree.push(tx.hash.clone());
        }

        let mut j: u64 = 0;
        let mut size = merkle_tree.len();
        while size > 1 {
            let mut i: u64 = 0;
            let temp = size as u64;

            // 合并计算哈希值
            while i < temp {
                let k = Self::min(i + 1, temp - 1);
                let idx1 = (j + i) as usize;
                let idx2 = (j + k) as usize;
                let hash1 = merkle_tree[idx1].clone();
                let hash2 = merkle_tree[idx2].clone();
                let merge: String = format!("{}-{}", hash1, hash2);
                let merge_ser = serialize(&merge);
                let merge_hash = hash_str(&merge_ser);

                // 合并计算的新哈希值放到 merkle_tree 末尾
                merkle_tree.push(merge_hash);
                i += 2;
            }

            // 跳过已经处理过的哈希
            j += temp;

            // 哈希数减少一半
            size = (size + 1) >> 1;
        }

        if !merkle_tree.is_empty() {
            merkle_tree.pop().unwrap()
        } else {
            "00000000".to_string()
        }
    }

    fn min(num1: u64, num2: u64) -> u64 {
        if num1 >= num2 { num2 } else { num1 }
    }
}
