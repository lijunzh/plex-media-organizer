# ADR-001: Iterative Development Approach

**Date**: 2025-08-17  
**Status**: Accepted  
**Context**: Need to choose development methodology for Plex Media Organizer  
**Decision**: Use iterative development with 2-week iterations  
**Consequences**: Faster feedback, easier testing, lower risk

## Context

The project needs a development methodology that allows for:
- Fast feedback and learning
- Easy testing with real-world data
- Lower risk of building the wrong thing
- Gradual feature expansion
- User validation early in the process

## Decision

**Use iterative development approach with 2-week iterations**

### **Iteration Structure**
1. **Iteration 1**: Movie MVP (Weeks 1-2)
2. **Iteration 2**: Movie Enhancement (Weeks 3-4)
3. **Iteration 3**: TV Shows (Weeks 5-6)
4. **Iteration 4**: TV Intelligence (Weeks 7-8)
5. **Iteration 5**: Music (Weeks 9-10)

### **Core Philosophy**
- **Start Small**: Begin with movies only
- **Test Small**: Use small directories for validation
- **Expand Gradually**: Add TV shows, then music
- **Refine Continuously**: Improve based on real-world usage

## Consequences

### **Positive**
- ✅ **Faster Feedback**: See working results sooner
- ✅ **Easier Testing**: Test with small, manageable datasets
- ✅ **Faster Learning**: Learn from real usage and adjust approach
- ✅ **Lower Risk**: Fail fast, learn fast, improve fast
- ✅ **User Validation**: Get user feedback on core functionality early

### **Negative**
- ❌ **Initial Scope Limited**: Can't build everything at once
- ❌ **Requires Discipline**: Must resist scope creep
- ❌ **Planning Overhead**: Need to plan each iteration carefully

## Implementation

### **Iteration 1 Success**
- ✅ **Movie Parsing**: Complete with TMDB integration
- ✅ **File Organization**: Plex naming conventions with safety features
- ✅ **CLI Interface**: Full subcommand support
- ✅ **Testing**: Comprehensive test suite (417 files, 100% success rate)
- ✅ **Performance**: 181 files/second with parallel processing

### **Lessons Learned**
- **Complete Code Review**: Essential before marking iterations complete
- **User Value Focus**: Ensure core functionality delivers user value
- **Documentation**: Keep project management docs updated

## References

- [Roadmap](../roadmap.md) - Detailed iteration plans
- [Status](../status.md) - Current project status
- [Architecture](../architecture.md) - System design details
