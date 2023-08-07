#[derive(clap::Parser)]
pub struct Arguments {
    #[clap(long, env, default_value = "0.0.0.0")]
    pub ip: String,

    #[clap(long, env, default_value = "8080")]
    pub port: String,

    #[clap(
        long,
        env,
        default_value = "postgres://postgres:password@localhost:5432/mat"
    )]
    pub db_url: String,
}

impl std::fmt::Display for Arguments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "IP: {}", self.ip)?;
        writeln!(f, "Port: {}", self.port)?;
        writeln!(f, "Database url: {}", self.db_url)?;

        Ok(())
    }
}
