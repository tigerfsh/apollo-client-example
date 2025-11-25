use apollo_client::conf::{ApolloConfClientBuilder, requests::CachedFetchRequest};
use ini::Properties;
use std::error::Error;
use url::Url;

mod utils;
use utils::{init_env_from_dotenv, get_env_var};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_env_from_dotenv()?;

    // 建议把环境变量的名字用 const 定义出来
    const APOLLO_URL_ENV: &str = "APOLLO_URL";
    const APOLLO_APP_ID_ENV: &str = "APOLLO_APP_ID";
    const APOLLO_NAMESPACE_NAME_ENV: &str = "APOLLO_NAMESPACE_NAME";
    const APOLLO_ACCESS_KEY_ENV: &str = "APOLLO_ACCESS_KEY";
    const DB_IP_KEY_ENV: &str = "DB_IP_KEY";
    const DB_PORT_KEY_ENV: &str = "DB_PORT_KEY";

    // 获取环境变量，并处理错误（带日志）
    let apollo_url = get_env_var(APOLLO_URL_ENV)?;
    let apollo_app_id = get_env_var(APOLLO_APP_ID_ENV)?;
    let apollo_namespace_name = get_env_var(APOLLO_NAMESPACE_NAME_ENV)?;
    let apollo_access_key = get_env_var(APOLLO_ACCESS_KEY_ENV)?;
    let db_ip_key = get_env_var(DB_IP_KEY_ENV)?;
    let db_port_key = get_env_var(DB_PORT_KEY_ENV)?;

    // Create configuration client.
    let client =
        ApolloConfClientBuilder::new_via_config_service(Url::parse(&apollo_url)?)?.build()?;

    // Request apollo cached configuration api.
    let configuration: Properties = match client
        .cached_fetch(CachedFetchRequest {
            app_id: apollo_app_id.to_string(),
            namespace_name: apollo_namespace_name.to_string(),
            access_key: Some(apollo_access_key.to_string()),
            ..Default::default()
        })
        .await
    {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to fetch configuration from Apollo server: {}", e);
            eprintln!("This might be due to network connectivity issues or incorrect server URL");
            return Err(e.into());
        }
    };

    // 打印全部配置内容方便调试
    // for (k, v) in configuration.iter() {
    //     println!("Config: {} = {}", k, v);
    // }

    let host = configuration.get(&db_ip_key);
    let port = configuration.get(&db_port_key);
    match (host, port) {
        (Some(host_val), Some(port_val)) => {
            println!("{}:{}", host_val, port_val);
        }
        (None, _) => log::error!(
            "Database host key '{}' not found in configuration",
            db_ip_key
        ),
        (_, None) => log::error!(
            "Database port key '{}' not found in configuration",
            db_port_key
        ),
    }

    Ok(())
}