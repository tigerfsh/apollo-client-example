use std::error::Error;

/// 从 `.env` 文件读取信息并初始化环境变量
pub fn init_env_from_dotenv() -> Result<(), Box<dyn Error>> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::env;

    let file = File::open(".env")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        // 跳过注释和空行
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        // 按第一个'='分割为 key 和 value
        if let Some(idx) = line.find('=') {
            let key = line[..idx].trim();
            let val = line[idx + 1..].trim();
            // 仅当当前环境变量不存在时才设置
            if env::var(key).is_err() {
                unsafe {
                    env::set_var(key, val);
                }
            }
        }
    }
    Ok(())
}

pub fn get_env_var(key: &str) -> Result<String, Box<dyn Error>> {
    std::env::var(key)
        .map_err(|e| format!("Environment variable `{}` not set or invalid: {}", key, e).into())
}