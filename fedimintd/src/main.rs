pub mod lib;
use std::path::PathBuf;

use clap::Parser;
use fedimint_api::db::Database;
use fedimint_core::modules::ln::LightningModule;
use fedimint_mint_server::MintServerModule;
use fedimint_server::config::ServerConfig;
use fedimint_server::consensus::FedimintConsensus;
use fedimint_server::ui::run_ui;
use fedimint_server::FedimintServer;
use fedimint_wallet::Wallet;
use fedimintd::encrypt::*;
use tokio::spawn;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;

#[derive(Parser)]
pub struct ServerOpts {
    pub cfg_path: PathBuf,
    #[arg(default_value = None)]
    pub password: Option<String>,
    #[arg(default_value = None)]
    pub ui_port: Option<u32>,
    #[cfg(feature = "telemetry")]
    #[clap(long)]
    pub with_telemetry: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    if let Some(ref arg) = args.nth(1) {
        if arg.as_str() == "version-hash" {
            println!("{}", env!("GIT_HASH"));
            return Ok(());
        }
    }
    let opts: ServerOpts = ServerOpts::parse();
    let fmt_layer = tracing_subscriber::fmt::layer();
    let filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let registry = tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer);

    let telemetry_layer = || -> Option<Box<dyn Layer<_> + Send + Sync + 'static>> {
        #[cfg(feature = "telemetry")]
        if opts.with_telemetry {
            let tracer = opentelemetry_jaeger::new_pipeline()
                .with_service_name("fedimint")
                .install_simple()
                .unwrap();

            return Some(tracing_opentelemetry::layer().with_tracer(tracer).boxed());
        }
        None
    };

    if let Some(layer) = telemetry_layer() {
        registry.with(layer).init();
    } else {
        registry.init();
    }

    let (sender, mut receiver) = tokio::sync::mpsc::channel(1);

    if let Some(ui_port) = opts.ui_port {
        // Spawn UI, wait for it to finish
        spawn(run_ui(opts.cfg_path.clone(), sender, ui_port));
        receiver
            .recv()
            .await
            .expect("failed to receive setup message");
    }

    let salt_path = opts.cfg_path.join(SALT_FILE);
    let key = get_key(opts.password, salt_path);
    let (decrypted, _) = encrypted_read(&key, opts.cfg_path.join(CONFIG_FILE));
    let cfg_string = String::from_utf8(decrypted).expect("is not correctly encoded");
    let cfg: ServerConfig = serde_json::from_str(&cfg_string).expect("could not parse config");

    let db: Database = fedimint_rocksdb::RocksDb::open(opts.cfg_path.join(DB_FILE))
        .expect("Error opening DB")
        .into();
    let btc_rpc = fedimint_bitcoind::make_bitcoind_rpc(&cfg.wallet.btc_rpc)?;

    let mint = fedimint_core::modules::mint::Mint::new(cfg.mint.clone(), db.clone());

    let wallet = Wallet::new_with_bitcoind(cfg.wallet.clone(), db.clone(), btc_rpc)
        .await
        .expect("Couldn't create wallet");

    let ln = LightningModule::new(cfg.ln.clone(), db.clone());

    let mut consensus = FedimintConsensus::new(cfg.clone(), mint, wallet, ln, db);

    consensus.register_module(MintServerModule::new().into());

    FedimintServer::run(cfg, consensus).await;

    #[cfg(feature = "telemetry")]
    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
