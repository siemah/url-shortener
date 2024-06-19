use libsql::{Builder, Database};

pub struct Config {
    pub port: String,
    pub db: Database,
}

impl Config {
    pub async fn new_connection() -> Result<Self, String> {
        dotenvy::dotenv()
            .expect("Please enter a port and database credentials");
        // todo: extract env vars such as db, port, etc
        let db_file = std::env::var("DB_FILE").expect("Please enter a db url");
        let db_url = std::env::var("DB_URL").expect("Please enter a db url");
        let db_token = std::env::var("DB_TOKEN").unwrap_or_else(|_| {
            println!("Please enter a db token");
            "".to_string()
        });
        let port = std::env::var("PORT").expect("Please enter a port");
        // todo: connect to a database
        let db = Builder::new_remote_replica(db_file, db_url, db_token)
            .read_your_writes(true)
            .build()
            .await
            .expect("Failed to connect to database");

        Result::Ok(Config {
            port,
            db,
        })
    }
}