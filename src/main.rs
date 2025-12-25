use clap::Parser;
use rayon::prelude::*;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sha3::Keccak256;
use std::env;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use rand::rngs::ThreadRng;
use rand::RngCore;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 末尾相同字符的最小数量
    #[arg(short, long, default_value_t = get_min_same_chars_default())]
    min_same_chars: usize,

    /// CPU核心数
    #[arg(short, long, default_value_t = num_cpus::get())]
    cores: usize,

    /// 输出文件名
    #[arg(short, long, default_value = "output")]
    output: String,

    /// 生成地址数量限制（0表示无限制）
    #[arg(short, long, default_value_t = 0)]
    limit: u64,

    /// 批处理大小（更大的批次可能更快）
    #[arg(short, long, default_value_t = 50000)]
    batch_size: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct TronAddress {
    address: String,
    private_key: String,
    same_chars_count: usize,
}

fn main() {
    let args = Args::parse();
    
    println!("🚀 Tron地址生成器启动 (极速模式)");
    println!("📊 配置信息:");
    println!("   - 最小相同字符数: {}", args.min_same_chars);
    println!("   - CPU核心数: {}", args.cores);
    println!("   - 批处理大小: {}", args.batch_size);
    println!("   - 输出文件: {}", args.output);
    if args.limit > 0 {
        println!("   - 生成限制: {} 个地址", args.limit);
    } else {
        println!("   - 生成限制: 无限制");
    }
    println!();

    // 设置线程池
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.cores)
        .build_global()
        .unwrap();

    let counter = Arc::new(AtomicU64::new(0));
    let found_counter = Arc::new(AtomicU64::new(0));
    let start_time = Instant::now();

    // 创建带缓冲的输出文件
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&args.output)
        .expect("无法创建输出文件");
    
    let writer = Arc::new(Mutex::new(BufWriter::with_capacity(64 * 1024, file)));

    loop {
        let results: Vec<Option<TronAddress>> = (0..args.batch_size)
            .into_par_iter()
            .map(|_| {
                let current_count = counter.fetch_add(1, Ordering::Relaxed);
                
                // 每50万次显示进度
                if current_count % 500000 == 0 && current_count > 0 {
                    let elapsed = start_time.elapsed();
                    let rate = current_count as f64 / elapsed.as_secs_f64();
                    let found = found_counter.load(Ordering::Relaxed);
                    println!(
                        "⚡ 已尝试: {:>12} | 已找到: {:>6} | 速度: {:>8.0} addr/s | 用时: {:>6.1}s",
                        current_count, found, rate, elapsed.as_secs_f64()
                    );
                }

                generate_tron_address_optimized(args.min_same_chars)
            })
            .collect();

        // 批量处理结果
        let mut batch_results = Vec::new();
        for result in results {
            if let Some(addr) = result {
                found_counter.fetch_add(1, Ordering::Relaxed);
                batch_results.push(addr);
            }
        }

        // 批量写入文件
        if !batch_results.is_empty() {
            let mut writer_guard = writer.lock().unwrap();
            for addr in &batch_results {
                writeln!(writer_guard, "地址: {}", addr.address).expect("写入文件失败");
                writeln!(writer_guard, "私钥: {}", addr.private_key).expect("写入文件失败");
                writeln!(writer_guard, "相同字符数: {}", addr.same_chars_count).expect("写入文件失败");
                writeln!(writer_guard, "---").expect("写入文件失败");
            }
            writer_guard.flush().expect("刷新文件失败");
            drop(writer_guard);

            // 显示找到的地址
            for addr in batch_results {
                println!(
                    "🎯 找到地址: {} ({}个相同字符: '{}')",
                    addr.address,
                    addr.same_chars_count,
                    get_repeated_char(&addr.address)
                );

                // 检查是否达到限制
                if args.limit > 0 && found_counter.load(Ordering::Relaxed) >= args.limit {
                    println!("\n✅ 已达到生成限制，程序结束");
                    return;
                }
            }
        }
    }
}

// 优化版本的地址生成函数
fn generate_tron_address_optimized(min_same_chars: usize) -> Option<TronAddress> {
    thread_local! {
        static SECP: Secp256k1<secp256k1::All> = Secp256k1::new();
        static RNG: std::cell::RefCell<ThreadRng> = std::cell::RefCell::new(rand::thread_rng());
    }
    
    SECP.with(|secp| {
        RNG.with(|rng_cell| {
            let mut rng = rng_cell.borrow_mut();
            
            // 直接生成32字节随机数作为私钥
            let mut private_key_bytes = [0u8; 32];
            rng.fill_bytes(&mut private_key_bytes);
            
            // 确保私钥有效（小于secp256k1的阶）
            if let Ok(private_key) = SecretKey::from_slice(&private_key_bytes) {
                let public_key = PublicKey::from_secret_key(secp, &private_key);
                
                // 获取未压缩的公钥字节
                let public_key_bytes = public_key.serialize_uncompressed();
                
                // 计算Keccak256哈希（去掉第一个字节0x04）
                let mut hasher = Keccak256::new();
                hasher.update(&public_key_bytes[1..]);
                let hash = hasher.finalize();
                
                // 取后20字节作为地址
                let mut address_bytes = [0u8; 21];
                address_bytes[0] = 0x41; // Tron主网前缀
                address_bytes[1..].copy_from_slice(&hash[12..]);
                
                // 计算校验和
                let checksum = double_sha256_optimized(&address_bytes);
                
                // 组合地址和校验和
                let mut full_address = [0u8; 25];
                full_address[..21].copy_from_slice(&address_bytes);
                full_address[21..].copy_from_slice(&checksum[..4]);
                
                // Base58编码
                let address = bs58::encode(full_address).into_string();
                
                // 优化的末尾相同字符检查
                let same_chars_count = count_trailing_same_chars_optimized(&address);
                
                if same_chars_count >= min_same_chars {
                    return Some(TronAddress {
                        address,
                        private_key: hex::encode(private_key_bytes),
                        same_chars_count,
                    });
                }
            }
            None
        })
    })
}

// 优化的双SHA256计算
fn double_sha256_optimized(data: &[u8]) -> [u8; 32] {
    thread_local! {
        static HASHER1: std::cell::RefCell<Sha256> = std::cell::RefCell::new(Sha256::new());
        static HASHER2: std::cell::RefCell<Sha256> = std::cell::RefCell::new(Sha256::new());
    }
    
    HASHER1.with(|h1| {
        HASHER2.with(|h2| {
            let mut hasher1 = h1.borrow_mut();
            let mut hasher2 = h2.borrow_mut();
            
            hasher1.reset();
            hasher1.update(data);
            let first_hash = hasher1.finalize_reset();
            
            hasher2.reset();
            hasher2.update(&first_hash);
            hasher2.finalize_reset().into()
        })
    })
}

// 优化的末尾相同字符计数
fn count_trailing_same_chars_optimized(s: &str) -> usize {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return 0;
    }
    
    let last_char = bytes[bytes.len() - 1];
    let mut count = 0;
    
    for &byte in bytes.iter().rev() {
        if byte == last_char {
            count += 1;
        } else {
            break;
        }
    }
    
    count
}

fn get_min_same_chars_default() -> usize {
    env::var("MIN_SAME_CHARS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(6)
}

fn get_repeated_char(s: &str) -> char {
    s.chars().last().unwrap_or('?')
}
