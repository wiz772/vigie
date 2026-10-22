const DEFAULT_SERVER_HOST: &str = "127.0.0.1";
const DEFAULT_SERVER_PORT: u16 = 3000;

pub struct Config {
    // pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
}

fn read_var(name: &str) -> Result<Option<String>, String> {
    match std::env::var(name) {
        Ok(value) => {
            let value = value.trim();
            if value.is_empty() {
                Err(format!("{name} must not be empty"))
            } else {
                Ok(Some(value.to_string()))
            }
        }
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(std::env::VarError::NotUnicode(_)) => Err(format!("{name} is not valid unicode")),
    }
}

impl Config {
    pub fn load() -> Result<Self, String> {
        // let database_url =
        //     read_var("VIGIE_DATABASE_URL")?.ok_or("VIGIE_DATABASE_URL is missing")?;

        let server_host =
            read_var("VIGIE_HOST")?.unwrap_or_else(|| DEFAULT_SERVER_HOST.to_string());

        let server_port = match read_var("VIGIE_PORT")? {
            Some(value) => {
                let port = value.parse::<u16>().map_err(|_| {
                    "VIGIE_PORT must be a valid port between 1 and 65535".to_string()
                })?;
                if port == 0 {
                    return Err("VIGIE_PORT cannot be 0".to_string());
                }
                port
            }
            None => DEFAULT_SERVER_PORT,
        };

        Ok(Self {
            // database_url,
            server_host,
            server_port,
        })
    }
}
