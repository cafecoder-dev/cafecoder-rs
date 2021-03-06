use once_cell::sync::Lazy;

pub static ENV_CONFIG: Lazy<Config> = Lazy::new(load_config);
pub struct Config {
    pub database_url: String,
    pub judge_container_port: String,
    pub docker_address: String,
}

pub fn load_config() -> Config {
    const DEFAULT_JUDGE_CONTAINER_PORT: &str = "8000";
    const DEFAULT_DOCKER_ADDRESS: &str = "http://localhost:2375";
    let database_url = dotenv::var("DATABASE_URL").unwrap();
    let judge_container_port = dotenv::var("JUDGE_CONTAINER_PORT")
        .unwrap_or_else(|_| DEFAULT_JUDGE_CONTAINER_PORT.to_string());
    let docker_address =
        dotenv::var("DOCKER_ADDRESS").unwrap_or_else(|_| DEFAULT_DOCKER_ADDRESS.to_string());
    Config {
        database_url,
        judge_container_port,
        docker_address,
    }
}
