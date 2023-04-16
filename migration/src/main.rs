use clap::Parser;
use postgres::Config;

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
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        let mut pg_config = Config::new();
        pg_config.user(&args.username);
        pg_config.password(&args.password);
        pg_config.dbname(&args.database);
        pg_config.host(&args.hostname);
        pg_config
    }
}


mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

fn main() {
    println!("Starting migration");
    let args = Args::parse();
    let pg_config: Config = args.into();
    let mut conn = pg_config.connect(postgres::NoTls).unwrap();
    embedded::migrations::runner().run(&mut conn).unwrap();
    println!("Migration is complete");
}

