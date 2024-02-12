#[derive(Clone, Debug)]
pub struct Config {
    pub db_url: String,
    pub host_ip: String,
    pub url: String,
    pub port: u16,
}

impl Config {
    pub fn init() -> Config {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
        let port = std::env::var("PORT").expect("PORT must be set!");
        let host_ip = std::env::var("HOST").unwrap_or(String::from("127.0.0.1"));
        let url = std::env::var("URL").unwrap_or(String::from("http://localhost"));
        let port_u16 = port.parse::<u16>().unwrap();

        Config {
            db_url,
            host_ip,
            url,
            port: port_u16,
        }
    }
}
