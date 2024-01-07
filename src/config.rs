#[derive(Clone, Debug)]
pub struct Config {
    pub db_url: String,
    pub port: u16,
}

impl Config {
    pub fn init() -> Config {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
        let port = std::env::var("PORT").expect("PORT must be set!");
        let port_u16 = port.parse::<u16>().unwrap();

        Config {
            db_url,
            port: port_u16,
        }
    }
}
