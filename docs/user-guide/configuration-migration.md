# Configuration Migration Guide

## Overview

The Plex Media Organizer includes an intelligent configuration migration system that automatically updates your configuration when new defaults are available, while preserving your customizations.

## How It Works

### Automatic Migration
When you run any command, the application automatically:
1. **Detects version differences** between your config and the current version
2. **Creates a backup** of your current configuration
3. **Merges new defaults** with your existing customizations
4. **Updates the version** to match the current release

### Migration Process
```
🔄 Configuration migration detected:
   From version: 0.1.0
   To version: 0.1.1
   📋 Backup created: ~/Library/Application Support/plex-media-organizer/config.toml.backup
   ✅ Configuration migrated successfully
```

## Manual Migration Commands

### Check Migration Status
```bash
plex-media-organizer migrate --dry-run
```

### Force Migration
```bash
plex-media-organizer migrate --force
```

### Restore from Backup
```bash
plex-media-organizer migrate --restore
```

## What Gets Migrated

### ✅ Preserved (Your Customizations)
- Custom technical terms you've added
- Modified quality preferences
- Custom title preservation rules
- User-specific language codes
- Personal API keys and settings

### 🔄 Updated (New Defaults)
- New release groups added to defaults
- New codec/quality terms
- Improved problematic patterns
- Enhanced language detection terms
- Updated content filtering rules

## Example Migration

### Before Migration (Your Config)
```toml
version = "0.1.0"

[organization.technical_terms]
release_groups = ["YIFY", "YTS", "MyCustomGroup"]  # You added "MyCustomGroup"
```

### After Migration (Merged Config)
```toml
version = "0.1.1"

[organization.technical_terms]
release_groups = [
    "YIFY", "YTS", "MyCustomGroup",  # Your custom term preserved
    "3L", "CMCT", "WiKi", "FRDS"     # New defaults added
]
```

## Safety Features

### Automatic Backup
- Creates `.backup` file before any changes
- Backup includes timestamp and version info
- Located in same directory as config file

### Dry Run Mode
- Preview changes without applying them
- Shows what would be migrated
- Safe way to understand changes

### Restore Capability
- Rollback to previous configuration
- Restore from any backup file
- Maintains full control over your settings

## Migration Scenarios

### Scenario 1: Fresh Installation
- No user config exists
- Uses embedded defaults
- No migration needed

### Scenario 2: First Run with Old Config
- Detects version mismatch
- Automatically migrates
- Preserves all customizations

### Scenario 3: Manual Migration
- User runs `migrate` command
- Full control over process
- Can preview and restore

## Platform-Specific Paths

### macOS
```
~/Library/Application Support/plex-media-organizer/
├── config.toml
└── config.toml.backup
```

### Linux
```
~/.local/share/plex-media-organizer/
├── config.toml
└── config.toml.backup
```

### Windows
```
%APPDATA%\plex-media-organizer\
├── config.toml
└── config.toml.backup
```

## Troubleshooting

### Migration Fails
1. Check file permissions
2. Ensure sufficient disk space
3. Verify config file is valid TOML

### Restore Issues
1. Verify backup file exists
2. Check backup file permissions
3. Ensure backup is valid TOML

### Version Conflicts
1. Check current app version
2. Verify config version field
3. Use `--force` if needed

## Best Practices

### Before Major Updates
1. Run `migrate --dry-run` to preview changes
2. Create manual backup if needed
3. Review new defaults

### After Migration
1. Verify your customizations are preserved
2. Test with sample files
3. Review new technical terms

### Regular Maintenance
1. Keep backups organized
2. Review migration logs
3. Clean up old backups periodically

## Advanced Usage

### Custom Migration Scripts
```bash
#!/bin/bash
# Custom migration script
plex-media-organizer migrate --dry-run
if [ $? -eq 0 ]; then
    plex-media-organizer migrate
    echo "Migration completed successfully"
else
    echo "Migration failed or not needed"
fi
```

### Batch Migration
```bash
# Migrate multiple configurations
for config in configs/*.toml; do
    cp "$config" ~/Library/Application\ Support/plex-media-organizer/config.toml
    plex-media-organizer migrate
done
```

## Support

If you encounter issues with configuration migration:

1. **Check the logs** for detailed error messages
2. **Use dry-run mode** to preview changes
3. **Restore from backup** if needed
4. **Report issues** with your config version and app version

The migration system is designed to be safe and transparent, ensuring you never lose your customizations while benefiting from improved defaults.
