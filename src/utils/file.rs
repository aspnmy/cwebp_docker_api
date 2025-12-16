use std::path::Path;
use chrono::{Utc, Duration};
use std::fs;
use log::{info, debug};
use rand::Rng;

/// 获取文件扩展名
#[allow(dead_code)]
pub fn get_file_extension(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(|ext| ext.to_str())
}

/// 生成唯一ID
#[allow(dead_code)]
pub fn generate_unique_id() -> String {
    let now = Utc::now();
    let timestamp = now.timestamp_millis();
    let mut rng = rand::thread_rng();
    let random_part: u32 = rng.r#gen();
    format!("{}_{:x}", timestamp, random_part)
}

/// 清理过期文件
/// 
/// # 参数
/// * `dir` - 要清理的目录
/// * `hours` - 文件保留的小时数，超过这个时间的文件将被删除
/// 
/// # 返回值
/// * `()` - 无返回值
pub fn cleanup_files(dir: &Path, hours: i32) {
    info!("Cleaning up files in {:?} that are older than {} hours", dir, hours);
    
    // 获取当前时间
    let now = Utc::now();
    // 计算过期时间
    let expiry_time = now - Duration::hours(hours as i64);
    
    // 遍历目录
    if let Ok(entries) = fs::read_dir(dir) {
        let mut deleted_count = 0;
        
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                
                // 只处理文件
                if path.is_file() {
                    // 获取文件的修改时间
                    if let Ok(metadata) = fs::metadata(&path) {
                        if let Ok(modified) = metadata.modified() {
                            let modified_utc = chrono::DateTime::<Utc>::from(modified);
                            
                            // 如果文件超过过期时间，删除它
                            if modified_utc < expiry_time {
                                if let Ok(_) = fs::remove_file(&path) {
                                    debug!("Deleted expired file: {:?}", path);
                                    deleted_count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        info!("Cleanup completed: {} files deleted from {:?}", deleted_count, dir);
    }
}
