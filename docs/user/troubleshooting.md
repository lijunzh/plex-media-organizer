# Troubleshooting Guide

## Common Issues and Solutions

### Movies Being Skipped

**Issue**: Movies are being skipped during organization.

**Cause**: This is **expected behavior**. The organizer uses a conservative approach.

**Solutions**:
1. **Review the skipped list**: Check why each movie was skipped
2. **Use lower confidence**: `--min-confidence 0.5 --preview`
3. **Check TMDB**: Verify the movie exists in TMDB
4. **Clean filenames**: Remove technical terms manually

**Example**:
```bash
# Check what's being skipped
plex-media-organizer organize /path --preview --verbose

# Use more permissive matching
plex-media-organizer organize /path --min-confidence 0.5 --preview
```

### TMDB API Errors

**Issue**: "TMDB API error" or "Invalid API key"

**Solutions**:
1. **Verify API key**: Check at https://www.themoviedb.org/settings/api
2. **Re-run setup**: `plex-media-organizer setup`
3. **Check internet**: Ensure you can access TMDB
4. **Check rate limits**: TMDB has rate limits

**Example**:
```bash
# Reconfigure API key
plex-media-organizer setup

# Test API connection
plex-media-organizer test /path/to/movie.mkv --verbose
```

### Permission Errors

**Issue**: "Permission denied" when accessing files or directories

**Solutions**:
1. **Check file permissions**: `ls -la /path/to/movies`
2. **Check directory permissions**: Ensure write access
3. **Run as appropriate user**: Don't use sudo unnecessarily
4. **Check network drive permissions**: For SMB/NFS shares

**Example**:
```bash
# Check permissions
ls -la /path/to/movies

# Fix permissions if needed
chmod 755 /path/to/movies
chmod 644 /path/to/movies/*.mkv
```

### Database Errors

**Issue**: "Database error" or "Cannot open database"

**Solutions**:
1. **Check database path**: Verify the path exists and is writable
2. **Check disk space**: Ensure sufficient space
3. **Reset database**: Remove and recreate if corrupted
4. **Check permissions**: Ensure write access to database directory

**Example**:
```bash
# Check database path
plex-media-organizer config --show

# Reset database (WARNING: loses all data)
rm ~/.local/share/plex-media-organizer/movies.db
plex-media-organizer setup
```

### Network Drive Issues

**Issue**: Slow performance or timeouts on network drives

**Solutions**:
1. **Check network connection**: Test connectivity
2. **Use appropriate timeouts**: Increase timeout settings
3. **Process in smaller batches**: Don't process entire drive at once
4. **Check SMB/NFS settings**: Optimize network protocol settings

**Example**:
```bash
# Test network connectivity
ping your-nas-ip

# Process smaller directory first
plex-media-organizer organize /path/to/small-directory --preview
```

### Memory Issues

**Issue**: "Out of memory" or very slow performance

**Solutions**:
1. **Process smaller batches**: Don't process entire collection at once
2. **Close other applications**: Free up system memory
3. **Use preview mode**: Preview before organizing
4. **Check system resources**: Monitor memory usage

**Example**:
```bash
# Process in smaller batches
plex-media-organizer organize /path/to/batch1 --preview
plex-media-organizer organize /path/to/batch2 --preview
```

## Error Messages Explained

### "No TMDB match found"
- **Meaning**: Movie not found in TMDB database
- **Action**: Check if movie exists in TMDB, verify title spelling

### "Low confidence"
- **Meaning**: TMDB match found but confidence below threshold
- **Action**: Use `--min-confidence 0.5` for more permissive matching

### "Technical terms in title"
- **Meaning**: Filename contains technical terms that couldn't be filtered
- **Action**: Clean filename manually or add terms to configuration

### "Year mismatch"
- **Meaning**: Year in filename doesn't match TMDB year
- **Action**: Check TMDB for correct year, use lower confidence threshold

### "File not found"
- **Meaning**: File was moved or deleted during processing
- **Action**: Check file exists, ensure no other processes are modifying files

## Performance Issues

### Slow Processing
**Causes**:
- Large number of files
- Network drive latency
- Insufficient system resources
- TMDB API rate limits

**Solutions**:
1. **Process in batches**: Smaller directories
2. **Use SSD storage**: Faster I/O
3. **Optimize network**: Better network connection
4. **Close other apps**: Free up resources

### High Memory Usage
**Causes**:
- Large file lists
- Caching too much data
- Memory leaks

**Solutions**:
1. **Process smaller batches**
2. **Restart application**
3. **Check for memory leaks**
4. **Monitor system resources**

## Debugging

### Enable Verbose Output
```bash
# Get detailed output
plex-media-organizer organize /path --verbose

# Test with verbose output
plex-media-organizer test /path/to/movie.mkv --verbose
```

### Check Configuration
```bash
# Show current configuration
plex-media-organizer config --show

# Validate configuration
plex-media-organizer config --validate
```

### Check Logs
```bash
# Enable debug logging (if available)
RUST_LOG=debug plex-media-organizer organize /path

# Check system logs
journalctl -u plex-media-organizer  # Linux
log show --predicate 'process == "plex-media-organizer"'  # macOS
```

## Getting Help

### Before Asking for Help
1. **Check this guide**: Look for similar issues
2. **Try verbose mode**: Get detailed error information
3. **Test with single file**: Isolate the issue
4. **Check configuration**: Verify settings are correct

### When Reporting Issues
Include:
- **Error message**: Exact error text
- **Command used**: Full command line
- **File example**: Sample filename causing issue
- **System info**: OS, Rust version, etc.
- **Verbose output**: Full verbose output

### Example Issue Report
```
Error: "No TMDB match found" for file "The.Matrix.1999.mkv"
Command: plex-media-organizer organize /path --verbose
System: macOS 12.0, Rust 1.70
Verbose output: [paste full output here]
```

## Prevention

### Best Practices
1. **Always use preview mode**: `--preview --verbose`
2. **Backup important data**: Use `--backup` option
3. **Test with small batches**: Start small, scale up
4. **Monitor system resources**: Check memory and disk space
5. **Keep configuration clean**: Don't modify config unnecessarily

### Regular Maintenance
1. **Update regularly**: Keep up with new versions
2. **Clean up backups**: Remove old backup files
3. **Monitor disk space**: Ensure sufficient space
4. **Check API limits**: Monitor TMDB API usage
