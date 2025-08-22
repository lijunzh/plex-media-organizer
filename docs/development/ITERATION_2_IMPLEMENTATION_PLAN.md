# Iteration 2 Implementation Plan: Enhanced Movie Parsing & Database Integration

**Branch**: `feature/iteration-2-enhanced-movie-parsing`  
**Timeline**: 2 weeks  
**Status**: 🚀 **READY TO START**

## 🎯 **Iteration 2 Objectives**

Based on roadmap alignment, Iteration 2 focuses on **Enhanced Movie Parsing** and **SQLite Database Integration**:

### **Primary Goals**
1. **Enhanced Movie Parsing** (Chinese/Japanese, multi-part, collections)
2. **SQLite Database Integration** for movie storage and caching
3. **Improved TMDB Matching** with database-backed results
4. **Better Organization** with database persistence

### **Success Criteria**
- ✅ **95%+ parsing accuracy** for complex movie titles
- ✅ **< 2s response time** for database queries
- ✅ **80%+ TMDB match success rate**
- ✅ **Zero data loss** during organization

## 📋 **Implementation Phases**

### **Phase 1: SQLite Database Foundation (Week 1)**
**Duration**: 3-4 days  
**Priority**: High

#### **1.1 Database Schema Design**
- [ ] Design movie table schema
- [ ] Design parsing cache table
- [ ] Design TMDB results cache table
- [ ] Create migration system

#### **1.2 Database Integration**
- [ ] Add SQLite dependencies to `Cargo.toml`
- [ ] Create `src/database/` module
- [ ] Implement database connection management
- [ ] Add database configuration to `AppConfig`

#### **1.3 Core Database Operations**
- [ ] Implement movie CRUD operations
- [ ] Implement cache operations
- [ ] Add database initialization
- [ ] Create database tests

### **Phase 2: Enhanced Movie Parsing (Week 1-2)**
**Duration**: 4-5 days  
**Priority**: High

#### **2.1 Chinese/Japanese Title Support**
- [ ] Enhance `FilenameParser` for CJK characters
- [ ] Add language detection logic
- [ ] Implement CJK-specific tokenization
- [ ] Add CJK test cases

#### **2.2 Multi-Part Movie Support**
- [ ] Detect movie series/collections
- [ ] Parse part numbers (Part 1, Part 2, etc.)
- [ ] Handle anthology movies
- [ ] Add multi-part test cases

#### **2.3 Collection Detection**
- [ ] Identify movie collections (Marvel, Star Wars, etc.)
- [ ] Parse collection-specific patterns
- [ ] Handle numbered vs. named collections
- [ ] Add collection test cases

### **Phase 3: Database-Backed TMDB Integration (Week 2)**
**Duration**: 3-4 days  
**Priority**: Medium

#### **3.1 TMDB Result Caching**
- [ ] Cache TMDB search results
- [ ] Implement cache expiration
- [ ] Add cache hit/miss metrics
- [ ] Create cache management tests

#### **3.2 Enhanced Search Strategies**
- [ ] Implement fallback search strategies
- [ ] Add fuzzy matching for titles
- [ ] Handle year variations
- [ ] Add search strategy tests

#### **3.3 Database-Backed Organization**
- [ ] Store organization decisions
- [ ] Track file movements
- [ ] Implement rollback capability
- [ ] Add organization tests

### **Phase 4: Integration & Testing (Week 2)**
**Duration**: 2-3 days  
**Priority**: Medium

#### **4.1 System Integration**
- [ ] Integrate database with existing components
- [ ] Update CLI commands for database
- [ ] Add database status commands
- [ ] Create integration tests

#### **4.2 Performance Optimization**
- [ ] Optimize database queries
- [ ] Add connection pooling
- [ ] Implement batch operations
- [ ] Performance testing

#### **4.3 Documentation & Cleanup**
- [ ] Update documentation
- [ ] Add database usage examples
- [ ] Create migration guide
- [ ] Code cleanup and review

## 🗄️ **Database Schema Design**

### **Movies Table**
```sql
CREATE TABLE movies (
    id INTEGER PRIMARY KEY,
    original_filename TEXT NOT NULL,
    parsed_title TEXT NOT NULL,
    year INTEGER,
    quality TEXT,
    source TEXT,
    release_group TEXT,
    language TEXT,
    collection_name TEXT,
    part_number INTEGER,
    tmdb_id INTEGER,
    tmdb_title TEXT,
    tmdb_year INTEGER,
    confidence_score REAL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### **Parsing Cache Table**
```sql
CREATE TABLE parsing_cache (
    id INTEGER PRIMARY KEY,
    filename_hash TEXT UNIQUE NOT NULL,
    parsed_data TEXT NOT NULL, -- JSON
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### **TMDB Cache Table**
```sql
CREATE TABLE tmdb_cache (
    id INTEGER PRIMARY KEY,
    search_key TEXT UNIQUE NOT NULL, -- "title:year"
    tmdb_results TEXT NOT NULL, -- JSON
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## 🔧 **Technical Implementation Details**

### **Dependencies to Add**
```toml
[dependencies]
rusqlite = { version = "0.29", features = ["bundled"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
```

### **New Modules to Create**
- `src/database/` - Database operations
- `src/database/schema.rs` - Database schema
- `src/database/movies.rs` - Movie CRUD operations
- `src/database/cache.rs` - Caching operations
- `src/parsing/enhanced.rs` - Enhanced parsing logic

### **Configuration Updates**
```toml
[database]
path = "data/movies.db"
cache_ttl_hours = 24
max_connections = 10
```

## 🧪 **Testing Strategy**

### **Unit Tests**
- Database operations
- Enhanced parsing logic
- Cache operations
- TMDB integration

### **Integration Tests**
- End-to-end movie processing
- Database persistence
- Cache hit/miss scenarios
- Performance benchmarks

### **Test Data**
- Chinese/Japanese movie titles
- Multi-part movies
- Collection movies
- Complex filename patterns

## 📊 **Success Metrics**

### **Performance Targets**
- Database query time: < 100ms
- Cache hit rate: > 80%
- Overall processing time: < 2s per movie
- Memory usage: < 100MB

### **Accuracy Targets**
- Parsing accuracy: > 95%
- TMDB match rate: > 80%
- False positive rate: < 5%
- False negative rate: < 10%

## 🚀 **Next Steps**

1. **Start with Phase 1**: Database foundation
2. **Implement incrementally**: Each phase builds on the previous
3. **Test thoroughly**: Maintain 100% test pass rate
4. **Document as we go**: Keep documentation updated
5. **Review and iterate**: Regular code reviews

## 📝 **Notes**

- **No learning system**: That's reserved for Iteration 3
- **Focus on stability**: Maintain existing functionality
- **Database-first**: All new features use database
- **Backward compatibility**: Existing config still works
- **Performance conscious**: Monitor and optimize throughout

---

**Ready to begin implementation! 🚀**
