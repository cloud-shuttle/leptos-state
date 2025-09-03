# 📚 **leptos-state Documentation**

> **🚧 Important Notice: Architectural Redesign in Progress**
> 
> We are currently undergoing a major architectural redesign to fix fundamental type system issues and create a more robust, maintainable library. The current v0.2.x version has some limitations with advanced features.

---

## 📋 **Documentation Overview**

This documentation covers the complete `leptos-state` library, from current usage to the upcoming architectural redesign. Our goal is to provide comprehensive guidance for both current users and those planning to migrate to v1.0.0.

---

## 🎯 **Current Status: v0.2.2**

### **✅ What Works Right Now**
- **Core State Machines** - Basic functionality compiles and tests pass
- **Simple Stores** - Basic store management works
- **Code Generation** - Actually generates working code in multiple languages
- **Testing Framework** - 90+ tests pass in isolation

### **⚠️ Current Limitations**
- **Advanced Features** - Some features don't work together due to type system issues
- **WASM-Only** - Examples can't run on native targets
- **Feature Flags** - Some advanced features don't compile properly

---

## 📖 **User Documentation**

### **Getting Started**
- **[📖 User Guide](./user-guide/README.md)** - Complete guide to using leptos-state v0.2.2
- **[📝 Examples](../examples/)** - Working code samples and demonstrations
- **[🔧 API Reference](../api-reference/)** - Current API documentation

### **Current Features**
- **Store Management** - Zustand-inspired state management
- **State Machines** - XState-inspired finite state machines
- **Leptos Integration** - Hooks and reactivity for Leptos applications
- **Code Generation** - Generate code in multiple languages

---

## 🏗️ **Architectural Redesign (v1.0.0)**

### **Planning Documents**
- **[🗺️ Development Roadmap](./development/ROADMAP.md)** - High-level roadmap and future plans
- **[🏗️ Architectural Redesign Plan](./development/ARCHITECTURAL_REDESIGN.md)** - Complete redesign overview
- **[🔧 Technical Specification](./development/TECHNICAL_SPECIFICATION.md)** - Implementation details
- **[📅 Implementation Timeline](./development/IMPLEMENTATION_TIMELINE.md)** - Week-by-week development plan

### **Redesign Goals**
- **Fix Type System Issues** - No more compilation errors with feature combinations
- **Feature Independence** - All features work together without conflicts
- **Better Performance** - Optimized for modern Rust and WASM
- **Future-Proof** - Architecture that can grow with your needs

---

## 🔄 **Migration Planning**

### **Migration Resources**
- **[🔄 Migration Guide](./migration/V0_2_TO_V1_0_MIGRATION.md)** - Complete migration from v0.2.x to v1.0.0
- **[📊 Migration Checklist](./migration/V0_2_TO_V1_0_MIGRATION.md#migration-checklist)** - Step-by-step migration steps
- **[🔧 Migration Tools](./migration/V0_2_TO_V1_0_MIGRATION.md#automatic-migration-tools)** - Automatic migration assistance

### **Migration Timeline**
- **v0.2.2** - Current version (September 2025)
- **v1.0.0-alpha** - Alpha release with new architecture (October 2025)
- **v1.0.0-beta** - Beta release with migration tools (November 2025)
- **v1.0.0** - Final release (December 2025)

---

## 🧪 **Development & Testing**

### **Development Resources**
- **[🧪 Testing Strategy](./development/TESTING_STRATEGY.md)** - Comprehensive testing approach
- **[📋 Contributing Guidelines](../CONTRIBUTING.md)** - How to contribute to the project
- **[🐛 Issue Reporting](https://github.com/cloud-shuttle/leptos-state/issues)** - Report bugs and request features

### **Quality Assurance**
- **Test Coverage** - Target: 95%+ with property-based testing
- **Performance Testing** - Continuous benchmarking and regression detection
- **Integration Testing** - All feature combinations tested together
- **Migration Testing** - Ensure v0.2.x → v1.0.0 compatibility

---

## 🌐 **Community & Support**

### **Getting Help**
- **GitHub Issues** - [Report bugs and request features](https://github.com/cloud-shuttle/leptos-state/issues)
- **GitHub Discussions** - [Ask questions and get help](https://github.com/cloud-shuttle/leptos-state/discussions)
- **Documentation** - This comprehensive documentation
- **Examples** - Working code samples for common use cases

### **Community Resources**
- **Contributing** - Help improve the library and documentation
- **Testing** - Test features and report issues
- **Feedback** - Provide input on design decisions
- **Examples** - Share your use cases and implementations

---

## 📊 **Documentation Status**

### **✅ Complete**
- **[🗺️ Development Roadmap](./development/ROADMAP.md)** - Updated with redesign plan
- **[🏗️ Architectural Redesign Plan](./development/ARCHITECTURAL_REDESIGN.md)** - Complete redesign overview
- **[🔧 Technical Specification](./development/TECHNICAL_SPECIFICATION.md)** - Implementation details
- **[📅 Implementation Timeline](./development/IMPLEMENTATION_TIMELINE.md)** - Development timeline
- **[📖 User Guide](./user-guide/README.md)** - Current usage documentation
- **[🔄 Migration Guide](./migration/V0_2_TO_V1_0_MIGRATION.md)** - Upgrade instructions
- **[🧪 Testing Strategy](./development/TESTING_STRATEGY.md)** - Testing approach

### **🚧 In Progress**
- **API Reference** - Updating for v1.0.0 architecture
- **Examples** - Converting to new architecture
- **Performance Guides** - Benchmarking and optimization
- **Video Tutorials** - Step-by-step guides

### **📋 Planned**
- **Interactive Examples** - Live, editable code samples
- **Migration Tools** - Automatic code conversion
- **Performance Dashboard** - Real-time performance metrics
- **Community Showcase** - User implementations and case studies

---

## 🚀 **Quick Navigation**

### **For Current Users (v0.2.x)**
1. **[📖 User Guide](./user-guide/README.md)** - Learn how to use the current version
2. **[📝 Examples](../examples/)** - See working code samples
3. **[🔄 Migration Guide](./migration/V0_2_TO_V1_0_MIGRATION.md)** - Plan your upgrade path

### **For Future Users (v1.0.0)**
1. **[🏗️ Architectural Redesign Plan](./development/ARCHITECTURAL_REDESIGN.md)** - Understand the new architecture
2. **[🔧 Technical Specification](./development/TECHNICAL_SPECIFICATION.md)** - See implementation details
3. **[📅 Implementation Timeline](./development/IMPLEMENTATION_TIMELINE.md)** - Follow development progress

### **For Contributors**
1. **[🧪 Testing Strategy](./development/TESTING_STRATEGY.md)** - Understand testing approach
2. **[📋 Contributing Guidelines](../CONTRIBUTING.md)** - Learn how to contribute
3. **[🗺️ Development Roadmap](./development/ROADMAP.md)** - See development priorities

---

## 📚 **Additional Resources**

### **External Links**
- **[GitHub Repository](https://github.com/cloud-shuttle/leptos-state)** - Source code and issues
- **[Crates.io](https://crates.io/crates/leptos-state)** - Package registry
- **[Documentation](https://docs.rs/leptos-state)** - Generated API docs
- **[Leptos Documentation](https://docs.rs/leptos)** - Leptos framework docs

### **Related Projects**
- **[Leptos](https://github.com/leptos-rs/leptos)** - Rust web framework
- **[rustate](https://github.com/rustate/rustate)** - State machine library
- **[bounce](https://github.com/bounce-rs/bounce)** - State management for Yew

---

## 🔄 **Documentation Updates**

### **Recent Updates**
- **September 4, 2025** - Complete documentation overhaul for architectural redesign
- **September 4, 2025** - Added comprehensive migration planning
- **September 4, 2025** - Created detailed implementation timeline
- **September 4, 2025** - Added comprehensive testing strategy

### **Upcoming Updates**
- **October 2025** - v1.0.0-alpha documentation
- **November 2025** - Migration tools and guides
- **December 2025** - Complete v1.0.0 documentation

---

## 🤝 **Contributing to Documentation**

### **How to Help**
1. **Report Issues** - File issues for unclear or missing documentation
2. **Suggest Improvements** - Propose better explanations or examples
3. **Add Examples** - Share your use cases and implementations
4. **Translate** - Help translate documentation to other languages

### **Documentation Standards**
- **Clarity** - Write for developers of all skill levels
- **Examples** - Include working code samples
- **Accuracy** - Ensure all information is current and correct
- **Completeness** - Cover all features and edge cases

---

*This documentation is actively maintained and updated as the project evolves. For the latest information, check the [GitHub repository](https://github.com/cloud-shuttle/leptos-state) or [discussions](https://github.com/cloud-shuttle/leptos-state/discussions). Last updated: September 4, 2025*
