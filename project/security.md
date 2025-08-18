# Security & Privacy

## API Key Management

### API Configuration
```toml
[apis]
# Users provide their own API keys for external services
tmdb_api_key = "your_tmdb_key_here"
tvdb_api_key = "your_tvdb_key_here"
musicbrainz_user_agent = "PlexMediaOrganizer/1.0"
anidb_username = "your_anidb_username_here"
anidb_password = "your_anidb_password_here"

[apis.rate_limits]
# Default limits based on free tier APIs
tmdb_requests_per_day = 1000
tvdb_requests_per_day = 1000
musicbrainz_requests_per_second = 1
anidb_requests_per_second = 0.5

[apis.setup]
# Instructions for users to obtain API keys
tmdb_setup_url = "https://www.themoviedb.org/settings/api"
tvdb_setup_url = "https://thetvdb.com/api-information"
musicbrainz_setup_url = "https://musicbrainz.org/doc/MusicBrainz_API"
anidb_setup_url = "https://anidb.net/perl-bin/animedb.pl?show=login"
```

### 🔐 **Secure Methods for API Key Management**

#### **1. GitHub Secrets (Recommended for CI/CD)**
For automated testing and CI/CD pipelines:

**Setup:**
1. Go to your GitHub repository
2. Navigate to **Settings** → **Secrets and variables** → **Actions**
3. Click **"New repository secret"**
4. Add your secrets:
   - **Name**: `TMDB_API_KEY`
   - **Value**: Your actual TMDB API key

**Security Features:**
- ✅ **Encrypted at rest** in GitHub's secure storage
- ✅ **Never logged** in CI output
- ✅ **Access controlled** - only repository admins can manage
- ✅ **Automatically masked** in logs

**Usage in CI/CD:**
```yaml
env:
  TMDB_API_KEY: ${{ secrets.TMDB_API_KEY }}
```

#### **2. Local Environment Variables (Development)**
For local development and testing:

**Setup:**
1. Copy the example file:
   ```bash
   cp env.example .env
   ```

2. Edit `.env` with your actual API keys:
   ```bash
   # TMDB API Key (get from https://www.themoviedb.org/settings/api)
   TMDB_API_KEY=your_actual_tmdb_api_key_here
   ```

3. The `.env` file is automatically ignored by git

**Security Features:**
- ✅ **Local only** - never uploaded to git
- ✅ **Easy to manage** - simple key-value pairs
- ✅ **Development friendly** - works with any IDE

#### **3. Configuration File (Alternative)**
For persistent configuration:

**Setup:**
1. Create a configuration file (e.g., `config.toml`):
   ```toml
   [apis]
   tmdb_api_key = "your_actual_tmdb_api_key_here"
   ```

2. Add to `.gitignore`:
   ```
   config.toml
   ```

**Security Features:**
- ✅ **Structured** - TOML format for complex config
- ✅ **Versionable** - can commit example configs
- ✅ **Flexible** - supports multiple environments

### 🛡️ **API Key Security Best Practices**

#### **1. Key Rotation**
- **Rotate keys regularly** (every 90 days)
- **Use different keys** for development and production
- **Monitor usage** for unusual activity

#### **2. Access Control**
- **Limit key permissions** to minimum required
- **Use read-only keys** when possible
- **Monitor API usage** and set rate limits

#### **3. Environment Separation**
- **Development**: Use test API keys or mock services
- **Staging**: Use limited production keys
- **Production**: Use full production keys

### 🔍 **Getting API Keys**

#### **TMDB API Key**
1. Go to [themoviedb.org](https://www.themoviedb.org/)
2. Create an account or sign in
3. Go to **Settings** → **API**
4. Request an API key
5. Choose **Developer** option for personal use

#### **TVDB API Key** (Future)
1. Go to [thetvdb.com](https://thetvdb.com/)
2. Create an account
3. Go to **Account** → **API Keys**
4. Generate a new API key

#### **MusicBrainz User Agent** (Future)
1. Go to [musicbrainz.org](https://musicbrainz.org/)
2. Create an account
3. Use format: `YourAppName/1.0 (your_email@example.com)`

### 🚨 **Security Checklist**

Before committing code:

- [ ] **No API keys in source code**
- [ ] **No API keys in configuration files**
- [ ] **No API keys in environment files**
- [ ] **No API keys in documentation**
- [ ] **No API keys in test files**
- [ ] **No API keys in logs or output**

### 🔧 **Testing Without API Keys**

The application is designed to work without API keys:

```bash
# Run tests without API keys
cargo test

# Run specific tests that don't require API
cargo test --test dynamic_real_world_test

# Run with API keys (if available)
TMDB_API_KEY=your_key cargo test
```

### 📞 **Security Issues**

If you accidentally commit an API key:

1. **Immediately rotate the key** in the service provider
2. **Remove the key** from git history
3. **Check for unauthorized usage**
4. **Report to security team** if applicable

### 🔒 **Additional Security Measures**

#### **Rate Limiting**
The application includes built-in rate limiting:
- TMDB: 1000 requests per day
- TVDB: 1000 requests per day
- MusicBrainz: 1 request per second

#### **Error Handling**
- **Graceful degradation** when APIs are unavailable
- **No sensitive data** in error messages
- **Secure logging** without API keys

#### **Network Security**
- **HTTPS only** for API communications
- **Certificate validation** for all requests
- **Timeout handling** for network issues

---

**Remember**: Security is everyone's responsibility. When in doubt, err on the side of caution and never expose sensitive credentials.
