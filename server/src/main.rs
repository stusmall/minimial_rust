use clap::Parser;
use server::dao::DaoImpl;
use server::run;

#[derive(Debug, Parser)]
struct Args {
    #[clap(env = "HOSTNAME")]
    hostname: String,
    #[clap(env = "USERNAME")]
    username: String,
    #[clap(env = "PASSWORD")]
    password: String,
    #[clap(env = "DATABASE")]
    database: String,
    #[clap(env = "PORT", default_value_t = 8080)]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let dao = DaoImpl::new(args.hostname, args.database, args.username, args.password).unwrap();

    run(dao, args.port).await.unwrap();
}
