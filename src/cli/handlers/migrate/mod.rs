//! Migration command handler for configuration updates

use crate::{
    config::AppConfig,
    output::{print_section_header, print_subsection_header},
};
use anyhow::Result;
use clap::Args;
use std::fs;

#[derive(Args, Debug)]
pub struct MigrateArgs {
    /// Force migration even if versions match
    #[arg(short, long)]
    force: bool,

    /// Show what would be migrated without applying changes
    #[arg(short, long)]
    dry_run: bool,

    /// Restore from backup
    #[arg(long)]
    restore: bool,
}

pub async fn handle_migrate(args: MigrateArgs) -> Result<()> {
    print_section_header("Configuration Migration");

    let config_dir = AppConfig::get_config_dir()?;
    let config_file = config_dir.join("config.toml");
    let backup_file = config_file.with_extension("toml.backup");

    if args.restore {
        if backup_file.exists() {
            fs::copy(&backup_file, &config_file)?;
            println!("✅ Configuration restored from backup");
            println!("   Backup file: {}", backup_file.display());
        } else {
            println!("❌ No backup file found at: {}", backup_file.display());
        }
        return Ok(());
    }

    // Load current configuration
    let current_config = AppConfig::load()?;

    if !args.force && !current_config.needs_migration() {
        println!(
            "✅ Configuration is up to date (version: {})",
            current_config.version
        );
        return Ok(());
    }

    if args.dry_run {
        println!("🔍 Migration preview:");
        let (from_version, to_version) = current_config.get_migration_info();
        println!("   From version: {}", from_version);
        println!("   To version: {}", to_version);
        println!("   Changes would be applied automatically");
        println!("   Backup would be created at: {}", backup_file.display());
        return Ok(());
    }

    // Perform migration
    print_subsection_header("Performing Migration");

    // Create backup
    if config_file.exists() {
        fs::copy(&config_file, &backup_file)?;
        println!("📋 Backup created: {}", backup_file.display());
    }

    // Load defaults and merge
    let default_config = AppConfig::load_defaults_only()?;
    let migrated_config = AppConfig::merge_with_defaults(current_config, default_config);

    // Update version
    let mut final_config = migrated_config;
    final_config.version = AppConfig::get_current_version();

    // Save migrated configuration
    final_config.save()?;

    print_subsection_header("Migration Complete");
    println!("✅ Configuration migrated successfully");
    println!("   From version: {}", final_config.version);
    println!("   To version: {}", AppConfig::get_current_version());
    println!("   Backup available at: {}", backup_file.display());
    println!("   New configuration saved to: {}", config_file.display());

    Ok(())
}
