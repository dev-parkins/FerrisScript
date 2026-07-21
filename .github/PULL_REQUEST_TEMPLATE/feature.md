## ✨ New Feature

<!-- Provide a clear and concise description of the new feature -->

## Related Issues

<!-- Link to feature requests or discussions -->

- Closes #
- Relates to #

## Motivation

**Why is this feature needed?**
<!-- Describe the problem this feature solves -->

**Who will benefit from this feature?**
<!-- Identify the target users -->

**What alternatives were considered?**
<!-- Describe any alternative approaches you evaluated -->

## Feature Description

<!-- Detailed description of what the feature does -->

### User-Facing Changes

<!-- How will users interact with this feature? -->

### Example Usage

```rust
// Show how users will use this feature
// Example code demonstrating the new functionality
```

## Changes Made

<!-- List the specific changes you made -->

### Compiler Changes

-
-

### Runtime Changes

-
-

### API Changes

-
-

### Documentation Changes

-
-

## Testing

- [ ] All existing tests pass (`cargo test`)
- [ ] Added comprehensive tests for new functionality
- [ ] Added integration tests (if applicable)
- [ ] Manual testing completed
- [ ] Benchmarks added (for performance-critical features)

**Test Coverage:**

```bash
# Show test commands and results
cargo test --workspace
cargo test --test integration_tests
```

**Manual Testing:**

```bash
# Commands demonstrating the feature
cargo run examples/new-feature-demo.ferris
```

## Code Quality

- [ ] Code formatted with `cargo fmt`
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Public APIs documented with rustdoc comments
- [ ] CHANGELOG.md updated (add to "Added" section)
- [ ] Examples added demonstrating the feature
- [ ] README.md updated (if user-facing)

## Documentation

<!-- Ensure the feature is well-documented -->

- [ ] Rustdoc comments for all public APIs
- [ ] Example code in docs/examples
- [ ] User guide updated (if applicable)
- [ ] Architecture documentation updated (if needed)

## Breaking Changes

- [ ] This PR introduces breaking changes

**If yes, describe them:**
<!-- Explain what breaks and why it was necessary -->

**Migration Guide:**
<!-- Provide step-by-step instructions for users to update their code -->

```rust
// Before
old_api_usage()

// After
new_api_usage()
```

## Performance Impact

<!-- Describe any performance implications -->

**Performance Considerations:**

- [ ] No significant performance impact
- [ ] Performance improved
- [ ] Performance impact acceptable (explain below)

**Benchmarks:**
<!-- Include benchmark results if performance-critical -->

```bash
# Before
test bench_feature ... bench: 1,234 ns/iter (+/- 56)

# After  
test bench_feature ... bench: 987 ns/iter (+/- 43)
```

## Screenshots / Demo Output

**Example Output:**

```
<!-- Paste terminal output showing the feature in action -->
```

## Godot Integration

<!-- If this affects Godot bindings -->

- [ ] Godot bindings updated
- [ ] Tested in Godot test project
- [ ] Godot example added/updated

**Godot Test Results:**
<!-- Describe testing in Godot environment -->

## Compatibility

<!-- Describe compatibility considerations -->

- **Minimum Rust version:** <!-- e.g., 1.70+ -->
- **Godot version compatibility:** <!-- e.g., 4.2+ -->
- **Platform compatibility:** <!-- Windows, Linux, macOS -->

## Checklist

- [ ] I have read the [CONTRIBUTING.md](../../CONTRIBUTING.md) guide
- [ ] Feature is complete and production-ready
- [ ] All tests pass locally
- [ ] Documentation is comprehensive
- [ ] Examples demonstrate the feature clearly
- [ ] Breaking changes are documented with migration guide
- [ ] My commits follow [Conventional Commits](https://www.conventionalcommits.org/) (e.g., `feat: add async script loading`)

## Future Work

<!-- Optional: Related features or improvements for future PRs -->

- [ ] Future enhancement 1
- [ ] Future enhancement 2

## Notes to Reviewers

<!-- Specific areas you'd like reviewers to focus on -->

**Key areas for review:**

- API design and ergonomics
- Edge case handling
- Documentation clarity
- Test coverage

---

Thank you for contributing to FerrisScript! 🦀✨
