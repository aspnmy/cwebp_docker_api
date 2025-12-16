use std::process::Command;
use std::path::Path;
use log::debug;

/// 转换选项
#[derive(Debug)]
pub struct ConversionOptions {
    pub lossless: bool,
    pub quality: u8,
    pub near_lossless: u8,
    pub compression_level: u8,
    pub preset: Option<String>,
    pub method: u8,
}

impl Default for ConversionOptions {
    fn default() -> Self {
        Self {
            lossless: false,
            quality: 80,
            near_lossless: 100,
            compression_level: 6,
            preset: None,
            method: 4,
        }
    }
}

/// 将图片转换为webp格式
/// 
/// # 参数
/// * `input_path` - 输入图片路径
/// * `output_path` - 输出webp路径
/// * `options` - 转换选项
/// 
/// # 返回值
/// * `Ok(())` - 转换成功
/// * `Err(std::io::Error)` - 转换失败
pub fn convert_to_webp(
    input_path: &Path,
    output_path: &Path,
    options: &ConversionOptions
) -> std::io::Result<()> {
    // 构建cwebp命令
    let mut cmd = Command::new("cwebp");
    
    // 添加转换选项
    if options.lossless {
        cmd.arg("-lossless");
    }
    
    cmd.arg(format!("-q"))
       .arg(options.quality.to_string());
    
    cmd.arg(format!("-near_lossless"))
       .arg(options.near_lossless.to_string());
    
    cmd.arg(format!("-z"))
       .arg(options.compression_level.to_string());
    
    if let Some(preset) = &options.preset {
        cmd.arg(format!("-preset")).arg(preset);
    }
    
    cmd.arg(format!("-m"))
       .arg(options.method.to_string());
    
    // 添加输入输出文件
    cmd.arg(input_path)
       .arg("-o")
       .arg(output_path);
    
    debug!("Executing command: {:?}", cmd);
    
    // 执行命令
    let output = cmd.output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("cwebp conversion failed: {}", stderr)
        ));
    }
    
    Ok(())
}
