# Markdown Syntax Highlighting for FerrisScript 🎨

**Date**: October 7, 2025  
**Status**: Research & Planning  
**Target Version**: v0.4.0+ (Post-core language stability)  
**Priority**: Low-Medium (Documentation enhancement, not blocking)

---

## 🎯 Research Objective

Investigate syntax highlighting options for FerrisScript code blocks in markdown documentation to:

1. Understand GitHub Linguist integration for official language support
2. Evaluate JavaScript-based syntax highlighters for documentation sites
3. Identify fallback strategies (using similar languages like Rust)
4. Determine optimal implementation timeline based on roadmap priorities
5. Provide recommendations for short-term and long-term solutions

---

## 📊 Current State

### What We Have (v0.0.3)

- ✅ **`.ferris` file extension** - Established file format
- ✅ **VS Code TextMate grammar** - In progress (v0.0.2, see `v0.1.0-ROADMAP.md`)
- ✅ **Syntax highlighting in VS Code** - TextMate-based
- ✅ **Language syntax** - Rust-inspired (static typing, explicit types)

### Current Limitations

- ❌ **No GitHub markdown support** - Code blocks show as plain text
- ❌ **No web documentation syntax highlighting** - If/when we build docs site
- ❌ **Not recognized by GitHub Linguist** - Language not detected in repos
- ❌ **No Prism.js/Highlight.js support** - Popular highlighters don't know FerrisScript

### Example Problem

**Current markdown code block** (no highlighting):

````markdown
```ferrisscript
fn _ready() {
    let position: Vector2 = Vector2 { x: 100.0, y: 200.0 };
    self.position = position;
}
```
````

**Rendered on GitHub**: Plain text (no syntax highlighting)

**Workaround** (using Rust highlighting):

````markdown
```rust
fn _ready() {
    let position: Vector2 = Vector2 { x: 100.0, y: 200.0 };
    self.position = position;
}
```
````

**Rendered**: Rust syntax highlighting (mostly works, but not perfect)

---

## 🔍 Solution Options

### Option 1: GitHub Linguist Integration (Official Support)

**What is GitHub Linguist?**

- Official GitHub tool for language detection and syntax highlighting
- Powers syntax highlighting on github.com
- Defines language metadata (extensions, colors, categories)
- Uses TextMate grammars for highlighting

**GitHub Linguist Project**: https://github.com/github/linguist

#### How to Add a Language

**Requirements**:

1. **TextMate grammar** (`.tmLanguage.json` or `.tmLanguage`) ✅ We have this (v0.1.0)
2. **Language metadata** in `languages.yml`:

   ```yaml
   FerrisScript:
     type: programming
     color: "#CE422B"  # Rust-like orange/red
     extensions:
     - ".ferris"
     tm_scope: source.ferrisscript
     ace_mode: rust  # Fallback mode
     language_id: 999999  # Unique ID
   ```

3. **Sample code** for testing (in `samples/` directory)
4. **Pull request to Linguist repo** with above files

**Effort**:

- Packaging: 1-2 days
- PR submission & review: 2-4 weeks (depends on maintainers)
- **Total**: ~1 month from submission to merge

**Benefits**:

- ✅ Official GitHub support
- ✅ Automatic syntax highlighting in all GitHub markdown
- ✅ Language detection in repositories
- ✅ Shows up in GitHub's language statistics
- ✅ Zero maintenance after merge (GitHub maintains it)

**Drawbacks**:

- ⚠️ External dependency (Linguist maintainers must approve)
- ⚠️ Can be rejected or delayed
- ⚠️ No control over update timeline
- ⚠️ Only helps on GitHub (not other platforms or docs sites)

**Requirements Before Submission**:

- ✅ Stable TextMate grammar (from v0.1.0)
- ✅ Established file extension (`.ferris`)
- ⚠️ Some adoption/usage (GitHub prefers languages with users)
- ⚠️ Stable language syntax (avoid breaking changes)

**Recommendation**: **Submit to Linguist in v0.1.0 or v0.2.0**

- After LSP/editor support is stable
- After syntax is relatively stable
- Before major adoption (so highlighting exists when users arrive)

**Reference**: https://github.com/github/linguist/blob/master/CONTRIBUTING.md#adding-a-language

---

### Option 2: Prism.js (Web Documentation Highlighter)

**What is Prism.js?**

- Popular JavaScript syntax highlighter for web pages
- Lightweight, extensible, customizable
- Used by many documentation sites
- Supports custom language definitions

**Prism.js Project**: https://prismjs.com/

#### How to Add a Language

**Implementation**:

1. **Create language definition** (JavaScript):

   ```javascript
   Prism.languages.ferrisscript = {
     'comment': /\/\/.*/,
     'string': /(["'])(?:\\.|(?!\1)[^\\\r\n])*\1/,
     'keyword': /\b(?:fn|let|mut|if|else|while|return|self)\b/,
     'type': /\b(?:i32|f32|i64|f64|bool|String|Vector2)\b/,
     'number': /\b0x[\da-f]+\b|(?:\b\d+\.?\d*|\B\.\d+)(?:e[+-]?\d+)?/i,
     'operator': /[+\-*\/%=!<>]=?|&&|\|\|/,
     'punctuation': /[{}[\];(),.:]/
   };
   ```

2. **Load in documentation site**:

   ```html
   <script src="prism.js"></script>
   <script src="prism-ferrisscript.js"></script>
   <link rel="stylesheet" href="prism.css">
   ```

3. **Use in markdown** (after rendering to HTML):

   ````markdown
   ```ferrisscript
   fn _ready() {
       print("Hello, Ferris!");
   }
   ```
   ````

**Effort**:

- Language definition: 1-2 days (based on TextMate grammar)
- Testing: 1 day
- Documentation: 1 day
- **Total**: 3-4 days

**Benefits**:

- ✅ Full control over highlighting rules
- ✅ Works on any website (docs site, blog, tutorials)
- ✅ Lightweight and fast
- ✅ Customizable themes
- ✅ Can update independently

**Drawbacks**:

- ❌ Doesn't help on GitHub
- ⚠️ Requires documentation site to exist
- ⚠️ Must maintain language definition separately
- ⚠️ Must distribute to users (if they build docs)

**Use Cases**:

- Official documentation site (e.g., ferrisscript.org)
- Tutorial sites
- Blog posts about FerrisScript
- API documentation generators

**Recommendation**: **Implement when building documentation site** (v0.4.0+)

- After language is stable
- When official docs site is built
- Can be packaged with docs

**Reference**: https://prismjs.com/extending.html#language-definitions

---

### Option 3: Highlight.js (Alternative Web Highlighter)

**What is Highlight.js?**

- Another popular JavaScript syntax highlighter
- Automatic language detection
- Similar to Prism.js but different API
- Used by many documentation sites (Stack Overflow, etc.)

**Highlight.js Project**: https://highlightjs.org/

#### How to Add a Language

**Implementation**:

1. **Create language definition** (JavaScript):

   ```javascript
   export default function(hljs) {
     return {
       name: 'FerrisScript',
       aliases: ['ferris', 'ferrisscript'],
       keywords: {
         keyword: 'fn let mut if else while return self',
         type: 'i32 f32 i64 f64 bool String Vector2',
         literal: 'true false'
       },
       contains: [
         hljs.C_LINE_COMMENT_MODE,
         hljs.QUOTE_STRING_MODE,
         hljs.NUMBER_MODE,
         // ... more patterns
       ]
     };
   }
   ```

2. **Register language**:

   ```javascript
   hljs.registerLanguage('ferrisscript', ferrisscript);
   hljs.highlightAll();
   ```

**Effort**: Similar to Prism.js (3-4 days)

**Benefits**:

- ✅ Automatic language detection (nice for mixed docs)
- ✅ Large community and plugin ecosystem
- ✅ Works on any website
- ✅ Customizable

**Drawbacks**:

- ⚠️ Slightly heavier than Prism.js
- ⚠️ Similar limitations (doesn't help on GitHub)

**Comparison to Prism.js**:

- **Prism.js**: More lightweight, better for controlled environments
- **Highlight.js**: Better auto-detection, larger ecosystem
- **Choice**: Depends on documentation site framework

**Recommendation**: **Alternative to Prism.js** (v0.4.0+)

- Choose based on docs site framework
- If using Docusaurus/MkDocs/etc., check what they recommend

**Reference**: https://highlightjs.readthedocs.io/en/latest/language-guide.html

---

### Option 4: Shiki (VS Code Powered Highlighter)

**What is Shiki?**

- Syntax highlighter powered by VS Code's TextMate engine
- Uses **same grammar as VS Code** (perfect for consistency!)
- Very accurate highlighting (exactly matches editor)
- Used by modern docs sites (VuePress, Astro, etc.)

**Shiki Project**: https://shiki.matsu.io/

#### How It Works

**Key Advantage**: **Reuses VS Code TextMate grammar!**

- Same `.tmLanguage.json` file we already have
- No need to write separate highlighting rules
- Consistency between editor and docs

**Implementation**:

1. **Package TextMate grammar**:

   ```javascript
   import { getHighlighter } from 'shiki';
   
   const highlighter = await getHighlighter({
     theme: 'nord',
     langs: [
       {
         id: 'ferrisscript',
         scopeName: 'source.ferrisscript',
         path: './ferrisscript.tmLanguage.json',  // Reuse VS Code grammar!
         aliases: ['ferris']
       }
     ]
   });
   ```

2. **Highlight code**:

   ```javascript
   const html = highlighter.codeToHtml(code, { lang: 'ferrisscript' });
   ```

**Effort**:

- Integration: 1 day (reuse existing grammar)
- Testing: 1 day
- **Total**: 2 days

**Benefits**:

- ✅ **Reuses VS Code grammar** (no duplicate work!)
- ✅ Exact consistency with VS Code highlighting
- ✅ Very accurate (VS Code's mature engine)
- ✅ Supports all VS Code themes
- ✅ Modern and actively maintained
- ✅ Works with static site generators (Astro, VitePress)

**Drawbacks**:

- ⚠️ Heavier than Prism.js/Highlight.js (bundles TextMate engine)
- ⚠️ Build-time highlighting (not runtime like Prism.js)
- ⚠️ Better for static sites than client-side highlighting
- ❌ Doesn't help on GitHub

**Use Cases**:

- Static documentation sites (VitePress, Astro, Docusaurus)
- API documentation generators
- Tutorial sites with build step

**Recommendation**: **Best option for documentation site** (v0.4.0+)

- Reuses existing VS Code work
- Perfect consistency
- Modern tooling

**Reference**: https://shiki.matsu.io/guide/

---

### Option 5: Fallback Strategy (Use Rust Highlighting)

**Short-Term Workaround**:
Since FerrisScript syntax is heavily inspired by Rust, we can use Rust highlighting as a fallback.

**Markdown Usage**:

````markdown
```rust
// Works reasonably well for FerrisScript code
fn _ready() {
    let position: Vector2 = Vector2 { x: 100.0, y: 200.0 };
    self.position = position;
}
```
````

**Accuracy**:

- ✅ Keywords match (`fn`, `let`, `mut`, `if`, `else`, etc.)
- ✅ Types mostly match (`i32`, `f32`, `bool`, `String`)
- ✅ Comments match (`//`, `/* */`)
- ✅ Strings and numbers match
- ⚠️ Some FerrisScript-specific features not highlighted (e.g., Godot types like `Vector2`)
- ⚠️ Rust-specific keywords highlighted incorrectly (e.g., `impl`, `trait`)

**Benefits**:

- ✅ Works everywhere immediately (GitHub, GitLab, docs sites)
- ✅ Zero effort
- ✅ Good enough for early documentation
- ✅ No maintenance burden

**Drawbacks**:

- ⚠️ Not 100% accurate
- ⚠️ May confuse users ("Why is this Rust?")
- ⚠️ Doesn't highlight FerrisScript-specific features

**Recommendation**: **Use until v0.4.0**

- Good enough for current docs
- Replace with proper highlighting once language is stable
- Add note in docs: "Currently using Rust highlighting as approximation"

---

## 📊 Comparison Matrix

| Option | GitHub Support | Docs Site Support | Effort | Maintenance | Accuracy | Timeline |
|--------|----------------|-------------------|--------|-------------|----------|----------|
| **GitHub Linguist** | ✅ Official | ❌ No | Medium (1 month) | ✅ GitHub maintains | ✅ High | v0.1.0-v0.2.0 |
| **Prism.js** | ❌ No | ✅ Yes | Low (3-4 days) | ⚠️ We maintain | ✅ High | v0.4.0+ |
| **Highlight.js** | ❌ No | ✅ Yes | Low (3-4 days) | ⚠️ We maintain | ✅ High | v0.4.0+ |
| **Shiki** | ❌ No | ✅ Yes | Very Low (2 days) | ✅ Reuses grammar | ✅ Very High | v0.4.0+ |
| **Rust Fallback** | ✅ Works | ✅ Works | Zero | Zero | ⚠️ Medium | Now |

---

## 🗓️ Recommended Implementation Timeline

### Phase 1: v0.0.3-v0.1.0 (Current) - **Use Rust Fallback**

**Status**: Immediate workaround

**Actions**:

1. Use `rust` language tag in all markdown code blocks
2. Add note in documentation:

   ```markdown
   > **Note**: Code examples currently use Rust syntax highlighting as FerrisScript 
   > syntax is similar. Native FerrisScript highlighting coming in v0.4.0.
   ```

3. Document this decision in contribution guide

**Rationale**:

- Language syntax still evolving
- Focus on core features (LSP, Godot integration)
- Good enough for early adopters
- Zero effort

---

### Phase 2: v0.1.0-v0.2.0 - **Submit to GitHub Linguist**

**Status**: After VS Code TextMate grammar is stable

**Actions**:

1. **Package Linguist submission** (1-2 days):
   - Create `languages.yml` entry
   - Include TextMate grammar (`.tmLanguage.json`)
   - Add sample FerrisScript code
   - Write submission rationale

2. **Submit PR to GitHub Linguist** (2-4 weeks review):
   - Link to FerrisScript project
   - Demonstrate usage/adoption
   - Respond to maintainer feedback

3. **Update docs after merge**:
   - Switch markdown code blocks to `ferrisscript`
   - Remove "Rust fallback" notes

**Prerequisites**:

- ✅ Stable TextMate grammar (v0.1.0)
- ✅ Established `.ferris` extension
- ⚠️ Some community adoption (helps PR acceptance)
- ✅ Stable core syntax (avoid breaking changes)

**Rationale**:

- Timing is right after v0.1.0 (editor support stable)
- Language syntax relatively stable (core features done)
- Early enough that users get highlighting when they discover FerrisScript
- One-time effort, maintained by GitHub

**Effort**: 1-2 days (packaging) + 2-4 weeks (review wait time)

---

### Phase 3: v0.4.0+ - **Documentation Site Syntax Highlighting**

**Status**: When building official documentation site

**Recommended Approach**: **Shiki** (reuses VS Code grammar)

**Actions**:

1. **Set up Shiki integration** (2 days):
   - Install Shiki in docs site build
   - Configure to load FerrisScript TextMate grammar
   - Test with various code examples
   - Verify theme compatibility

2. **Package for distribution**:
   - Include in docs site repository
   - Document for contributors
   - Add to docs build pipeline

3. **Update documentation**:
   - Switch all code blocks to `ferrisscript`
   - Showcase syntax highlighting in README
   - Add highlighting examples to docs

**Alternative**: Prism.js or Highlight.js (if Shiki doesn't fit docs framework)

**Rationale**:

- Happens when we need it (docs site doesn't exist yet)
- Shiki reuses existing VS Code work (minimal effort)
- Perfect consistency between editor and docs
- Modern, well-maintained tool

**Effort**: 2 days (Shiki) or 3-4 days (Prism.js/Highlight.js)

---

### Phase 4: v0.5.0+ (Optional) - **Additional Platforms**

**Status**: As needed for ecosystem growth

**Potential Additions**:

- **GitLab syntax highlighting** (if community uses GitLab)
- **Stack Overflow syntax highlighting** (if tag gets created)
- **Discord/Reddit code highlighting** (if community is active)
- **Jupyter Notebook support** (if FerrisScript REPL exists)

**Effort**: Varies by platform

---

## 🎯 Recommended Approach

### Short-Term (Now - v0.1.0): **Rust Fallback**

- ✅ Use `rust` in markdown code blocks
- ✅ Add explanatory note in docs
- ✅ Focus on core language development
- ✅ Zero effort, good enough

### Medium-Term (v0.1.0 - v0.2.0): **GitHub Linguist Submission**

- 📋 Package TextMate grammar for Linguist
- 📋 Submit PR after v0.1.0 stability
- 📋 Provides GitHub markdown support
- 📋 ~1 month effort + review time

### Long-Term (v0.4.0+): **Shiki for Documentation Site**

- 📋 Integrate Shiki when building docs site
- 📋 Reuse VS Code grammar (consistency!)
- 📋 Modern, accurate highlighting
- 📋 ~2 days effort

---

## 📝 Dependencies & Prerequisites

### For GitHub Linguist Submission (v0.1.0-v0.2.0)

**Required**:

- [x] `.ferris` file extension established ✅
- [ ] Stable TextMate grammar (v0.1.0) ⏳
- [ ] VS Code extension published (v0.1.0) ⏳
- [ ] Sample FerrisScript code (examples/) ✅
- [ ] Language relatively stable (core features done) ⏳

**Nice-to-Have**:

- [ ] Some GitHub repositories using FerrisScript (shows adoption)
- [ ] Active community (demonstrates demand)
- [ ] Stable v0.1.0 release (shows maturity)

### For Documentation Site Highlighting (v0.4.0+)

**Required**:

- [ ] Documentation site exists (static site generator) ⏳
- [ ] Stable TextMate grammar ✅ (from v0.1.0)
- [ ] Build system for docs (npm, etc.) ⏳

**Nice-to-Have**:

- [ ] Multiple themes tested
- [ ] Mobile-responsive highlighting
- [ ] Dark/light mode support

---

## 🚫 Out of Scope

### Not Recommended

1. **Custom GitHub Actions for highlighting** ❌
   - Too complex, maintenance burden
   - GitHub Linguist is the proper way

2. **Browser extensions for highlighting** ❌
   - User burden (must install extension)
   - GitHub Linguist is better solution

3. **Server-side rendering for GitHub** ❌
   - Can't modify GitHub's rendering
   - Use Linguist instead

4. **Multiple highlighter implementations** ❌
   - Pick one for docs site (Shiki recommended)
   - Don't maintain Prism.js AND Highlight.js

---

## 📚 Reference Links

### Official Projects

- **GitHub Linguist**: https://github.com/github/linguist
- **Linguist Contributing Guide**: https://github.com/github/linguist/blob/master/CONTRIBUTING.md
- **Prism.js**: https://prismjs.com/
- **Highlight.js**: https://highlightjs.org/
- **Shiki**: https://shiki.matsu.io/

### Examples of Language Additions

- **Linguist Language PR Example**: https://github.com/github/linguist/pull/5555 (Zig language)
- **Prism.js Language Examples**: https://prismjs.com/#supported-languages
- **Shiki Language Examples**: https://shiki.matsu.io/languages

### Documentation Site Examples

- **Rust Documentation** (mdBook): https://doc.rust-lang.org/book/
- **TypeScript Documentation** (custom): https://www.typescriptlang.org/docs/
- **Vue.js Documentation** (VitePress + Shiki): https://vuejs.org/guide/

---

## 📊 Success Metrics

### GitHub Linguist (v0.1.0-v0.2.0)

- ✅ PR accepted and merged
- ✅ FerrisScript recognized in GitHub repos
- ✅ Code blocks highlighted on github.com
- ✅ Language shows in GitHub language statistics

### Documentation Site (v0.4.0+)

- ✅ Syntax highlighting works on all docs pages
- ✅ Highlighting matches VS Code exactly
- ✅ Fast page load times (< 500ms)
- ✅ Works on mobile devices
- ✅ Dark/light mode support

---

## 💡 Open Questions

1. **Which documentation site generator will we use?**
   - MkDocs (Python, popular for technical docs)
   - Docusaurus (React, Facebook's tool)
   - VitePress (Vue, modern and fast)
   - Astro (Multi-framework, very modern)
   - **Decision**: Choose based on v0.4.0 documentation needs

2. **Should we submit to Linguist before v0.1.0?**
   - Pro: Early users get highlighting immediately
   - Con: Syntax might still change (breaking changes)
   - **Recommendation**: Wait until v0.1.0 stable

3. **Should we maintain separate grammars for Linguist and VS Code?**
   - No - keep single source of truth (VS Code grammar)
   - Linguist uses TextMate grammars (same format)
   - **Decision**: Single grammar file, shared across tools

4. **How to handle FerrisScript-specific types in Rust fallback?**
   - `Vector2`, `Node2D`, etc. won't highlight correctly
   - Option: Add comments explaining custom types
   - **Decision**: Accept limitation, fix in v0.4.0+

---

## 🗺️ Roadmap Integration

### Add to v0.1.0 (or v0.2.0) Roadmap

**New Section**: "Documentation & Syntax Highlighting"

```markdown
### GitHub Linguist Submission

**Status**: 🔴 Not Started  
**Priority**: Low-Medium  
**Rationale**: Enable syntax highlighting in GitHub markdown

**Prerequisites**:
- Stable TextMate grammar (from LSP work)
- Published VS Code extension
- Stable core language syntax

**Implementation**:
1. Package TextMate grammar for Linguist
2. Create language metadata (`languages.yml`)
3. Add sample FerrisScript code
4. Submit PR to github/linguist

**Effort**: 1-2 days (packaging) + 2-4 weeks (review)
```

### Add to v0.4.0 Roadmap (Proposed)

**New Roadmap**: `docs/planning/v0.4.0-roadmap.md`

**Focus**: Documentation Site & Tooling Enhancements

**Key Features**:

- Official documentation site (VitePress/Astro/etc.)
- Shiki syntax highlighting (reuses VS Code grammar)
- API documentation generator
- Interactive code playground (stretch goal)
- Tutorial system

**Syntax Highlighting Section**:

```markdown
### Shiki Integration for Documentation Site

**Status**: 🔴 Not Started  
**Priority**: Medium  
**Rationale**: Professional syntax highlighting for docs

**Implementation**:
1. Install Shiki in docs site
2. Configure FerrisScript TextMate grammar
3. Test with code examples
4. Verify theme compatibility

**Effort**: 2 days
```

---

## ✅ Summary & Recommendations

### Immediate Actions (v0.0.3 - Now)

1. ✅ **Use Rust fallback** in all markdown:
   - Use ` ```rust ` for FerrisScript code blocks
   - Add explanatory note in docs
   - Zero effort, works immediately

### v0.1.0 - v0.2.0 Actions

1. 📋 **Submit to GitHub Linguist**:
   - After TextMate grammar is stable
   - Package and submit PR
   - Provides official GitHub support
   - ~1 month timeline

### v0.4.0+ Actions

1. 📋 **Implement Shiki for docs site**:
   - When building documentation site
   - Reuses VS Code grammar (consistency)
   - Modern, accurate highlighting
   - ~2 days effort

### Key Principle: "Start simple, upgrade incrementally"

- Rust fallback is good enough for now
- GitHub Linguist provides broad GitHub support
- Shiki provides perfect docs site highlighting
- Don't over-engineer early (language still evolving)

---

## Document End

This research document provides the foundation for FerrisScript's markdown syntax highlighting strategy and informed the roadmap decisions for v0.1.0-v0.2.0 (Linguist) and v0.4.0+ (docs site).
