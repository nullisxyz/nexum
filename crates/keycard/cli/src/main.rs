use clap::Parser;
use nexum_apdu_transport_pcsc::{PcscConfig, PcscDeviceManager};
use std::error::Error;
use tracing::info;

mod commands;
mod utils;

use commands::Commands;

#[derive(Parser)]
#[command(version, about = "Nexum Keycard CLI - A tool for managing Keycard")]
struct Cli {
    /// Optional reader name to use (will auto-detect if not specified)
    #[arg(short, long)]
    reader: Option<String>,

    /// Detailed logging for debugging
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Setup logging based on verbosity
    setup_logging(cli.verbose);

    // Create a PC/SC device manager
    let manager = PcscDeviceManager::new()?;

    match &cli.command {
        Commands::List => {
            commands::list_readers(&manager)?;
            return Ok(());
        }
        _ => {
            // For all other commands, find appropriate reader
            let reader_name = match &cli.reader {
                Some(name) => utils::reader::find_reader_by_name(&manager, name)?,
                None => utils::reader::find_reader_with_card(&manager)?,
            };

            info!("Using reader: {}", reader_name);

            // Execute the command using the selected reader
            let config = PcscConfig::default();
            let transport = manager.open_reader_with_config(&reader_name, config)?;

            match &cli.command {
                Commands::List => unreachable!(), // Already handled above
                Commands::Select => commands::select_command(transport)?,
                Commands::Init {
                    pin,
                    puk,
                    pairing_password,
                } => commands::init_command(transport, pin, puk, pairing_password)?,
                Commands::Pair { output } => commands::pair_command(transport, output.as_ref())?,
                Commands::GenerateKey { pairing } => {
                    commands::generate_key_command(transport, pairing)?
                }
                Commands::ExportKey {
                    pairing,
                    derivation,
                    export_option,
                } => commands::export_key_command(transport, pairing, derivation, *export_option)?,
                Commands::Sign {
                    data,
                    derivation,
                    pairing,
                } => commands::sign_command(transport, data, derivation, pairing).await?,
                Commands::ChangeCredential {
                    credential_type,
                    new_value,
                    pairing,
                } => commands::change_credential_command(
                    transport,
                    credential_type,
                    new_value,
                    pairing,
                )?,
                Commands::UnblockPin {
                    puk,
                    new_pin,
                    pairing,
                } => commands::unblock_pin_command(transport, puk, new_pin, pairing)?,
                Commands::SetPinlessPath { path, pairing } => {
                    commands::set_pinless_path_command(transport, path, pairing)?
                }
                Commands::LoadKeyFromSeed {
                    password,
                    language,
                    pairing,
                } => commands::load_seed_command(transport, *password, language, pairing)?,
                Commands::RemoveKey { pairing } => {
                    commands::remove_key_command(transport, pairing)?
                }
                Commands::GetStatus { pairing } => {
                    commands::get_status_command(transport, pairing)?
                }
                Commands::Unpair { pairing } => commands::unpair_command(transport, pairing)?,
                Commands::GenerateMnemonic {
                    words_count,
                    pairing,
                } => commands::generate_mnemonic_command(transport, *words_count, pairing)?,
                Commands::StoreData {
                    record_type,
                    data,
                    pairing,
                } => {
                    commands::store_data_command(transport, *record_type, data.as_bytes(), pairing)?
                }
                Commands::GetData {
                    record_type,
                    pairing,
                } => commands::get_data_command(transport, *record_type, pairing)?,
                Commands::FactoryReset => commands::factory_reset_command(transport)?,
                Commands::Applet(cmd) => commands::applet_command(transport, cmd)?,
            }
        }
    }

    Ok(())
}

fn setup_logging(verbose: bool) {
    let level = if verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_ansi(true)
        .init();
}
