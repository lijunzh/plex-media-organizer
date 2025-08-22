# ADR-003: Documentation Format Choice

**Date**: 2025-08-17  
**Status**: Accepted  
**Context**: Need to choose optimal format for user-facing documentation  
**Decision**: Use Markdown files with future migration path to mdBook  
**Consequences**: Simple setup now, upgrade path available later

## Context

The project needs comprehensive user-facing documentation to serve three distinct user types:
- **Light users**: Want to get started ASAP with minimal friction
- **Power users**: Need complete feature understanding with examples
- **Developers**: Need quick code architecture understanding and change locations

We evaluated multiple documentation formats and tools to determine the best approach for both current needs and future scalability.

## Decision

**Use Markdown files for initial documentation with planned migration to mdBook for enhanced user experience**

### **Phase 1: Markdown Files (Current - Iteration 1)**
- Continue with standard Markdown files in `docs/` directory
- Focus on content quality over presentation
- Maintain simplicity for contributors and development velocity

### **Phase 2: mdBook Migration (Iteration 2+)**
- Migrate to mdBook when documentation grows beyond 20+ pages
- Triggered by user feedback about navigation or search needs
- Provides significant UX improvements while maintaining Markdown source

### **Phase 3: Advanced Features (As Needed)**
- Add interactive examples, runnable code snippets
- Consider more advanced platforms if community grows significantly

## Options Considered

### **1. Markdown Files (Selected for Phase 1)**

**✅ Pros:**
- **Zero setup**: Works immediately in GitHub, VS Code, etc.
- **Version controlled**: Changes tracked with code
- **Developer friendly**: Easy for contributors to update
- **Universal**: Renders everywhere (GitHub, editors, static sites)
- **Simple**: No build process or dependencies
- **Searchable**: Text-based, grep-friendly

**❌ Cons:**
- **Basic styling**: Limited visual design options
- **No interactivity**: Can't embed demos, forms, or dynamic content
- **Navigation**: No built-in table of contents or search
- **Organization**: Hard to manage large doc sets
- **User experience**: Not optimized for non-technical users

### **2. mdBook (Selected for Phase 2)**

**✅ Pros:**
- **Rust ecosystem**: Perfect fit for Rust project
- **Beautiful UI**: Modern, professional appearance
- **Built-in search**: Fast, client-side search functionality
- **Navigation**: Automatic table of contents and page navigation
- **Code highlighting**: Excellent syntax highlighting for examples
- **Markdown-based**: Easy migration from existing content
- **Runnable examples**: Can embed interactive code examples
- **Single binary**: Simple `cargo install mdbook` setup

**❌ Cons:**
- **Build step**: Requires compilation to static site
- **Additional dependency**: Need mdBook installed for development
- **Learning curve**: Contributors need to understand mdBook structure

### **3. Other Options Considered**

#### **GitBook**
**Rejected**: Commercial service, vendor lock-in, costs money for private repos

#### **Docusaurus (Meta/Facebook)**  
**Rejected**: JavaScript/Node.js dependency, complex setup, overkill for project size

#### **VitePress (Vue Ecosystem)**
**Rejected**: Vue.js dependency, additional complexity, not aligned with Rust stack

#### **Sphinx + Read the Docs**
**Rejected**: Python dependency, complex configuration, dated UI

#### **GitHub Pages + Jekyll**
**Rejected**: Ruby dependency, limited customization, slower builds

## Implementation Plan

### **Phase 1: Markdown Enhancement (Current)**

**Completed:**
- ✅ Streamlined main README.md to focus on first impression
- ✅ Created comprehensive docs/ structure:
  - `quick-start.md` - 5-minute setup guide for light users
  - `installation.md` - Complete installation methods
  - `user-guide.md` - Comprehensive usage tutorial for power users
  - `configuration.md` - Full configuration reference

**In Progress:**
- [ ] `examples/` directory with practical examples
- [ ] `development/` directory with code-focused documentation
- [ ] `troubleshooting.md` with common issues and solutions

**Structure:**
```
docs/
├── README.md              # Navigation hub
├── quick-start.md         # Light users: 5-minute start
├── installation.md        # All installation methods
├── user-guide.md          # Power users: complete guide
├── configuration.md       # Full configuration reference
├── troubleshooting.md     # Common issues
├── examples/              # Real-world examples
│   ├── basic/
│   ├── advanced/
│   └── integrations/
└── development/           # Developer-focused docs
    ├── code-architecture.md
    ├── api-reference.md
    └── adding-features.md
```

### **Phase 2: mdBook Migration (Future)**

**Migration Triggers:**
- Documentation grows beyond 20+ pages
- Users request better navigation or search
- Need for interactive examples arises
- Community feedback indicates UX issues

**Migration Process:**
1. **Content preparation**: Ensure all Markdown is mdBook-compatible
2. **Structure conversion**: Create `SUMMARY.md` navigation file
3. **Build setup**: Add mdBook to CI/CD pipeline
4. **Deployment**: Deploy to GitHub Pages or similar
5. **Redirect**: Update links and add redirects from old structure

**Future mdBook Structure:**
```
book/
├── src/
│   ├── SUMMARY.md          # Navigation structure
│   ├── introduction.md     # Project overview
│   ├── quick-start.md      # Getting started
│   ├── user-guide/         # User documentation
│   ├── examples/           # Practical examples
│   ├── development/        # Developer docs
│   └── reference/          # Complete references
├── book.toml               # mdBook configuration
└── theme/                  # Custom styling (optional)
```

## Success Metrics

### **Phase 1 Success Criteria**
- [ ] All three user types can find what they need quickly
- [ ] Contributors can easily update documentation
- [ ] Documentation stays in sync with code changes
- [ ] Setup time for new users < 5 minutes

### **Phase 2 Migration Criteria**
- [ ] User feedback indicates navigation/search needs
- [ ] Documentation size > 20 pages
- [ ] Need for interactive features
- [ ] Community growth requires better UX

### **Quality Metrics**
- **User feedback**: Positive responses about documentation clarity
- **Contribution rate**: Easy for community to contribute docs
- **Maintenance burden**: Low overhead for keeping docs updated
- **Discoverability**: Users can find answers without asking questions

## Benefits Achieved

### **Immediate Benefits (Phase 1)**
- ✅ **Simple setup**: No build process or dependencies
- ✅ **Fast iteration**: Can focus on content over tooling
- ✅ **Universal access**: Works in GitHub, IDEs, text editors
- ✅ **Contributor friendly**: Easy for anyone to update
- ✅ **Version controlled**: Documentation changes tracked with code

### **Future Benefits (Phase 2)**
- ✅ **Enhanced UX**: Professional appearance and navigation
- ✅ **Better search**: Built-in search functionality
- ✅ **Interactive examples**: Runnable code snippets
- ✅ **Rust alignment**: Tool fits naturally with project ecosystem
- ✅ **Scalability**: Can handle large documentation sets

### **Long-term Strategy**
- ✅ **Flexible approach**: Can adapt to changing needs
- ✅ **Migration path**: Clear upgrade path when needed
- ✅ **Content preservation**: Markdown content works in any system
- ✅ **Tool alignment**: Future tools align with Rust ecosystem

## Lessons Learned

### **Content First, Presentation Second**
Focus on creating high-quality, comprehensive content before optimizing presentation. Good content in simple format beats poor content in fancy format.

### **User-Centric Organization**
Structure documentation around user needs rather than technical implementation:
- **Light users**: Quick start and immediate value
- **Power users**: Comprehensive guides and examples  
- **Developers**: Code architecture and contribution guides

### **Progressive Enhancement**
Start simple and enhance based on actual needs rather than anticipated requirements. This prevents over-engineering and keeps development focused.

## References

- [User Guide](../docs/user-guide.md) - Complete user documentation
- [Quick Start](../docs/quick-start.md) - Getting started guide
- [Configuration](../docs/configuration.md) - Configuration reference
- [The Rust Book](https://doc.rust-lang.org/book/) - Example of excellent mdBook documentation
- [mdBook Documentation](https://rust-lang.github.io/mdBook/) - mdBook user guide
