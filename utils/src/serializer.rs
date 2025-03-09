use crypto::digest::Digest;
use crypto::sha3::Sha3;
use serde::Serialize;

// 序列化数据
pub fn serialize<T: ?Sized>(value: &T) -> Vec<u8>
where
    T: Serialize,
{
    bincode::serialize(value).unwrap()
}

// 计算 value 哈希值并以 String 形式返回
pub fn hash_str(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

// 计算 value 哈希值并传递给 out 参数
pub fn hash_u8(value: &[u8], mut out: &mut [u8]) {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result(&mut out);
}
