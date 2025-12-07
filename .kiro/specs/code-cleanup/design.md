# Design Document

## Overview

This design addresses code quality improvements across four main areas: eliminating compiler warnings, fixing test failures, rewriting user documentation for accessibility, and optimizing steering documentation for efficient AI context usage. The work is primarily focused on code cleanup, documentation updates, and removing unused code rather than adding new functionality.

## Architecture

The cleanup work spans multiple layers of the application:

1. **Source Code Layer**: Address unused variables, functions, and enum variants in the Rust codebase
2. **Testing Layer**: Fix doctest failures and ensure all tests pass
3. **Documentation Layer**: Rewrite README.md for non-technical users
4. **Steering Layer**: Update and simplify AI guidance documentation

No architectural changes are required. All work maintains the existing structure while improving code quality and documentation.

## Components and Interfaces

### Compiler Warning Resolution

**Affected Files:**
- `src/main.rs` - Contains unused variables (`total_files`, `skipped_count`)
- `src/handlers.rs` - Contains unused variable (`season`)
- `src/database.rs` - Contains unused function (`get_log_file_for_test`)
- `src/path_resolver.rs` - Contains unused enum variants (`RootDirectoryNotFound`, `RootDirectoryNotAccessible`)
- `src/video_metadata.rs` - Contains unused associated function (`new`), unused enum (`EpisodeState`), unused field (`series_id`), and unused function (`determine_episode_state`)

**Resolution Strategy:**
- Remove truly unused code that serves no purpose
- Use variables that are assigned but not read
- Add `#[allow(dead_code)]` attributes only when code is intentionally kept for future use
- Prefer removal over suppression to keep the codebase clean

### Doctest Failure Resolution

**Issue:** The doctest fails with a shared library error related to `rustdoc`:
```
rustdoc: error while loading shared libraries: libLLVM.so.20.1-rust-1.90.0-stable: cannot open shared object file
```

**Root Cause Analysis:**
- The library exists at `/usr/lib/rustlib/x86_64-unknown-linux-gnu/lib/libLLVM.so.20.1-rust-1.90.0-stable`
- The dynamic linker cannot find it because this path is not in the standard library search path
- This is a system configuration issue with the Rust installation, not a code issue

**Resolution Strategy:**

Option 1 (Requires sudo): Create a symlink in `/usr/lib`:
```bash
sudo ln -s /usr/lib/rustlib/x86_64-unknown-linux-gnu/lib/libLLVM.so.20.1-rust-1.90.0-stable /usr/lib/
```

Option 2 (No sudo required): Set `LD_LIBRARY_PATH` when running tests:
```bash
export LD_LIBRARY_PATH=/usr/lib/rustlib/x86_64-unknown-linux-gnu/lib:$LD_LIBRARY_PATH
cargo test --doc
```

Option 3 (Requires sudo): Update the system library cache:
```bash
echo "/usr/lib/rustlib/x86_64-unknown-linux-gnu/lib" | sudo tee /etc/ld.so.conf.d/rust.conf
sudo ldconfig
```

**Recommended Approach:**
- Use Option 2 (LD_LIBRARY_PATH) for immediate testing without system changes
- Document this workaround in the README or developer documentation
- Note: There are no doctests in the current codebase (verified by searching for `///.*````)
- If doctests are added in the future, they will need this workaround to run

### README Rewrite

**Current State:** The README is technically focused with detailed architecture sections and developer-oriented language.

**Target Audience:** Movie enthusiasts with moderate technical ability who want to organize their video collection.

**New Structure:**
1. **Introduction**: What the app does in simple terms
2. **Why Use This**: Benefits for organizing a video collection
3. **Getting Started**: Simple installation steps
4. **Basic Usage**: How to browse, organize, and play videos
5. **Features**: Key capabilities explained simply
6. **Configuration**: Basic customization options
7. **Troubleshooting**: Common issues and solutions
8. **For Developers**: Link to technical documentation (optional section)

**Tone Guidelines:**
- Friendly and welcoming
- Avoid jargon where possible
- Explain technical terms when necessary
- Focus on what users can do, not how it works internally
- Use examples and scenarios
- Encourage collaboration
- Encourage code critiquing (although do not try to hide the fact that the code was written by AI)

### Steering Documentation Optimization

**Current Files:**
- `github-issues.md` - GitHub integration guidance
- `menu-features.md` - Menu system implementation patterns (very detailed)
- `product.md` - Product overview
- `spec-management.md` - Spec workflow guidance
- `structure.md` - Project structure and architecture
- `tech.md` - Technology stack

**Optimization Strategy:**

1. **Verify Accuracy**: Ensure all references match current codebase
   - Check that file paths are correct
   - Verify module names and structures
   - Update any outdated patterns

2. **Reduce Verbosity**: Simplify while preserving essential information
   - Remove redundant explanations
   - Consolidate similar concepts
   - Use more concise language
   - Remove overly detailed examples where simpler ones suffice

3. **Prioritize Information**: Keep most critical guidance, reduce nice-to-have details
   - Essential patterns and conventions: keep
   - Detailed examples: simplify or reduce
   - Redundant information: remove

4. **Specific Targets**:
   - `menu-features.md`: Very long (300+ lines), can be significantly condensed
   - `structure.md`: Contains some outdated references (mentions `config.json` but app uses `config.yaml`)
   - All files: Check for redundancy across files

## Data Models

No data model changes required. This work only addresses code quality and documentation.

## Test Cases

### Test Case 1: Compiler warnings eliminated

When the system compiles the library and binary crates, the build output should contain zero warnings.
**Validates: Requirements 1.1, 1.2, 1.3, 1.4, 1.5**

### Test Case 2: Unused code removed

When reviewing the codebase after cleanup, all previously unused variables, functions, and enum variants should either be used or removed.
**Validates: Requirements 1.3, 1.4, 1.5**

### Test Case 3: Tests pass

When running `cargo test` (excluding doctests if the rustdoc issue persists), all unit and integration tests should pass.
**Validates: Requirements 2.3**

### Test Case 4: README accessibility

When a non-technical user reads the README, they should understand what the application does, how to install it, and how to use basic features without needing to understand Rust or terminal concepts.
**Validates: Requirements 3.1, 3.2, 3.3, 3.4, 3.5**

### Test Case 5: Steering documentation accuracy

When steering documentation references code structures, those references should match the current codebase (e.g., `config.yaml` not `config.json`).
**Validates: Requirements 4.1, 4.2**

### Test Case 6: Steering documentation conciseness

When comparing the updated steering documentation to the original, the total line count should be reduced while preserving all essential information.
**Validates: Requirements 4.3, 4.4, 4.5**

## Error Handling

No new error handling required. This work maintains existing error handling patterns while removing unused code.

## Testing Strategy

### Manual Verification

1. **Compiler Warnings**: Run `cargo build` and verify zero warnings
2. **Test Execution**: Run `cargo test` and verify all tests pass (excluding doctests if rustdoc issue persists)
3. **README Review**: Have a non-technical user review the README for clarity
4. **Steering Accuracy**: Cross-reference steering docs with actual code

### Automated Testing

- Existing unit tests should continue to pass
- No new tests required for this cleanup work

### Documentation Testing

- README should be readable by target audience
- Steering docs should be verified against actual codebase structure
- Token count reduction can be measured by comparing file sizes

## Implementation Notes

### Compiler Warning Priority

Address warnings in this order:
1. Unused variables (easiest to fix)
2. Unused functions (may require more analysis)
3. Unused enum variants (may indicate incomplete implementation)

### README Rewrite Approach

- Start fresh rather than editing existing README
- Keep technical details in a separate developer documentation file if needed
- Use concrete examples (e.g., "organize your Star Trek episodes" rather than "organize video files")

### Steering Documentation Approach

- Review each file individually
- Look for cross-file redundancy
- Prioritize information that directly helps with code generation
- Remove or condense examples that are overly detailed

### Doctest Issue

The rustdoc shared library error is a system configuration issue where the LLVM library is not in the dynamic linker's search path. This can be resolved by setting `LD_LIBRARY_PATH=/usr/lib/rustlib/x86_64-unknown-linux-gnu/lib:$LD_LIBRARY_PATH` before running `cargo test --doc`. Since there are currently no doctests in the codebase, this is not blocking any immediate work, but the workaround should be documented for future reference.
