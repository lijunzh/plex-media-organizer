# 🚀 Development Workflow Guide

## **📋 PR Workflow (Always Use PRs)**

### **✅ Required Steps for Every Feature**

1. **Create Feature Branch**
   ```bash
   git checkout main
   git pull origin main
   git checkout -b feature/descriptive-name
   ```

2. **Implement Feature**
   - Write code
   - Add tests
   - **Update documentation** (see below)
   - Run quality checks

3. **Update Documentation** (MANDATORY)
   - Update `docs/CURRENT_STATUS.md` with new features/status
   - Update relevant architecture docs if needed
   - Update README if user-facing changes
   - Document any new configuration options

4. **Create PR**
   ```bash
   git add .
   git commit -m "feat: descriptive commit message"
   git push origin feature/descriptive-name
   gh pr create --title "feat: descriptive title" --body "PR description"
   ```

5. **Review & Merge**
   - Wait for CI/CD checks to pass
   - Address any review feedback
   - Merge when approved

## **📚 Documentation Update Checklist**

### **For Every Feature PR:**

- [ ] **Update `docs/CURRENT_STATUS.md`**
  - [ ] Add new features to "Recent Updates" section
  - [ ] Update component status if needed
  - [ ] Update "Next Steps" section
  - [ ] Add performance metrics if applicable

- [ ] **Update Architecture Docs** (if needed)
  - [ ] `docs/ARCHITECTURE.md` for design changes
  - [ ] `docs/IMPLEMENTATION_ROADMAP.md` for plan updates

- [ ] **Update User-Facing Docs** (if needed)
  - [ ] `README.md` for new features
  - [ ] `config_example.toml` for new config options
  - [ ] `env.example` for new environment variables

## **🚫 What NOT to Do**

- ❌ **Never push directly to main**
- ❌ **Never create separate PRs for documentation** (unless it's a pure docs PR)
- ❌ **Never merge features without documentation updates**
- ❌ **Never skip the PR review process**

## **✅ Best Practices**

### **Commit Messages**
```
feat: add CJK title configuration
fix: resolve TMDB API timeout issues
docs: update architecture for database integration
test: add comprehensive test suite for organizer
```

### **PR Titles**
```
feat: Add CJK title configuration and consolidate documentation
fix: Resolve GitHub Actions permissions for PR comments
docs: Update development workflow and best practices
```

### **PR Descriptions**
- Include feature summary
- List technical changes
- Document testing performed
- Note any breaking changes
- Reference related issues

## **🔧 Quality Assurance**

### **Pre-PR Checklist**
- [ ] All tests pass (`cargo test`)
- [ ] Code formatting correct (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated
- [ ] Commit messages follow convention
- [ ] Feature branch is up to date with main

### **CI/CD Requirements**
- [ ] GitHub Actions pass
- [ ] Code coverage maintained
- [ ] Performance benchmarks pass
- [ ] Security checks pass

## **📈 Iteration Workflow**

### **Iteration Planning**
1. **Define Goals** - What features to deliver
2. **Create Roadmap** - Update `docs/IMPLEMENTATION_ROADMAP.md`
3. **Set Milestones** - Break down into manageable PRs

### **Iteration Execution**
1. **Feature Branches** - One branch per feature
2. **Documentation First** - Update docs as part of feature PRs
3. **Continuous Integration** - All PRs go through CI/CD
4. **Code Review** - Systematic review before merge

### **Iteration Completion**
1. **Status Update** - Mark iteration as complete in docs
2. **Retrospective** - Document lessons learned
3. **Next Iteration** - Plan and begin next iteration

## **🎯 Example Workflow**

### **Adding a New Feature**

```bash
# 1. Start from clean main
git checkout main
git pull origin main

# 2. Create feature branch
git checkout -b feature/fuzzy-search

# 3. Implement feature
# ... code changes ...
# ... test changes ...

# 4. Update documentation
# Edit docs/CURRENT_STATUS.md
# Edit relevant architecture docs

# 5. Commit everything together
git add .
git commit -m "feat: add fuzzy search for better movie matching

- Implement Levenshtein distance algorithm
- Add configurable similarity threshold
- Update movie parser to use fuzzy matching
- Add comprehensive test suite
- Update documentation with new feature"

# 6. Push and create PR
git push origin feature/fuzzy-search
gh pr create --title "feat: add fuzzy search for better movie matching" --body "..."
```

## **📞 Getting Help**

- **Documentation Issues**: Check `docs/` directory
- **CI/CD Problems**: Check GitHub Actions logs
- **Code Quality**: Run `cargo clippy` and `cargo fmt`
- **Testing**: Run `cargo test --verbose`

## **🔄 Continuous Improvement**

This workflow should evolve based on:
- Team feedback
- Project needs
- Best practices
- Lessons learned

**Remember**: Documentation is part of the feature, not a separate task!
