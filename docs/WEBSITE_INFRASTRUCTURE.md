# FerrisScript Website Infrastructure

**Status**: 🚧 In Progress  
**Domain**: `ferrisscript.dev` ✅  
**Priority**: Low (can work on between features)  
**Target**: Before v0.0.3 release (or after - GitHub URLs work fine)

---

## 📋 Setup Checklist

### 1. ✅ Domain Acquisition

- [x] **Purchase ferrisscript.dev domain**
  - Provider: Namecheap / Cloudflare / Squarespace
  - Date: October 6, 2025
  - **Note**: `.dev` TLD requires HTTPS (enforced by browsers)

---

### 2. ⏳ Static Hosting Setup

- [ ] **Choose hosting provider**
  - **Recommended Options**:
    - ✨ **Netlify** - Free tier, easy GitHub integration, auto-SSL
    - ✨ **Vercel** - Fast, great DX, auto-deployments
    - **GitHub Pages** - Free, simple, but limited features
  
  - **Recommended**: Netlify or Vercel for best developer experience

- [ ] **Connect GitHub repository**
  - Branch: `main` (auto-deploy on push)
  - Build command: TBD (depends on framework choice)
  - Publish directory: TBD

- [ ] **Configure build settings**
  - Set up automatic deployments
  - Configure preview deployments for PRs (optional)
  - Set up build notifications

---

### 3. ⏳ DNS Configuration

- [ ] **Create CNAME for docs subdomain**
  - Subdomain: `docs.ferrisscript.dev`
  - Target: Hosting provider CNAME (e.g., `ferrisscript.netlify.app`)
  
  - **Example Netlify Setup**:

    ```
    Type: CNAME
    Name: docs
    Value: ferrisscript.netlify.app
    TTL: Auto
    ```

- [ ] **Wait for DNS propagation**
  - Typical time: 24-48 hours
  - Check propagation: `nslookup docs.ferrisscript.dev`

- [ ] **Configure custom domain in hosting provider**
  - Add `docs.ferrisscript.dev` in hosting dashboard
  - Enable HTTPS/SSL (automatic with Netlify/Vercel)

---

### 4. ⏳ Documentation Site Deployment

- [ ] **Choose documentation framework**
  
  **Option A: Docusaurus** (Recommended)
  - Pros: Modern, React-based, excellent search, versioning support
  - Cons: Heavier, requires Node.js build
  - Setup: `npx create-docusaurus@latest docs classic`
  
  **Option B: mdBook**
  - Pros: Rust-native, lightweight, simple
  - Cons: Less features, basic search
  - Setup: `cargo install mdbook && mdbook init docs`
  
  **Option C: VitePress**
  - Pros: Fast, Vue-based, good DX
  - Cons: Vue ecosystem (not Rust-native)
  - Setup: `npm init vitepress`

- [ ] **Convert ERROR_CODES.md to site format**
  - Create searchable error code index
  - Add navigation structure
  - Preserve error code anchors (e.g., `#e001`)

- [ ] **Set up documentation structure**

  ```
  docs/
  ├── errors/           # Error code documentation
  │   ├── index.md      # Error codes overview
  │   ├── lexical.md    # E001-E099
  │   ├── syntax.md     # E100-E199
  │   ├── type.md       # E200-E299
  │   └── runtime.md    # E400-E499
  ├── guides/           # User guides
  │   ├── getting-started.md
  │   ├── godot-integration.md
  │   └── examples.md
  └── reference/        # API reference
      ├── syntax.md
      ├── types.md
      └── builtins.md
  ```

- [ ] **Configure search functionality**
  - Built-in search (Docusaurus/VitePress)
  - Or Algolia DocSearch (free for open source)

- [ ] **Add site metadata**
  - Title: FerrisScript Documentation
  - Description: A Rust-inspired scripting language for Godot
  - Social preview image
  - Favicon

---

### 5. ⏳ Verification & Testing

- [ ] **Verify HTTPS is enabled**
  - Test: `https://docs.ferrisscript.dev`
  - Chrome/browsers enforce HTTPS for `.dev` domains
  - Certificate should auto-provision (Let's Encrypt)

- [ ] **Test error code links**
  - Direct links: `https://docs.ferrisscript.dev/errors/E001`
  - Anchor links: `https://docs.ferrisscript.dev/errors#e001`
  - Verify all E001-E299 codes work

- [ ] **Test site functionality**
  - Navigation works on all pages
  - Search returns relevant results
  - Mobile responsive design
  - Fast load times

- [ ] **Update compiler environment variable**
  - Set `FERRIS_DOCS_BASE=https://docs.ferrisscript.dev` in production
  - Test that error messages show new URLs
  - Verify backwards compatibility (GitHub URLs still work)

---

## 🔗 URL Structure

### Current (Phase 3A Implementation)

**Without env var** (default):

```
https://github.com/dev-parkins/FerrisScript/blob/main/docs/ERROR_CODES.md#e001
```

**With env var** (`FERRIS_DOCS_BASE=https://docs.ferrisscript.dev`):

```
https://docs.ferrisscript.dev/errors/E001
```

### Recommended Site Structure

```
docs.ferrisscript.dev/
├── /                     # Homepage
├── /errors/              # Error codes index
├── /errors/E001          # Individual error pages
├── /guides/              # User guides
└── /reference/           # API reference
```

---

## 🎨 Optional Enhancements (After Launch)

- [ ] Add dark mode support
- [ ] Implement copy-to-clipboard for code snippets
- [ ] Add interactive error examples (WASM playground?)
- [ ] Set up analytics (optional, privacy-respecting)
- [ ] Add RSS feed for updates
- [ ] Create changelog page
- [ ] Add version selector (for future versions)

---

## 💻 Local Development

Once framework is chosen, typical workflow:

```bash
# Install dependencies
npm install  # or cargo install mdbook

# Start dev server
npm run dev  # or mdbook serve

# Build for production
npm run build  # or mdbook build

# Preview production build
npm run preview
```

---

## 📚 Resources

### Domain & DNS

- Domain Registrar: Namecheap / Cloudflare / Squarespace
- DNS Management: Registrar dashboard
- DNS Propagation Check: https://www.whatsmydns.net/

### Hosting Providers

- Netlify: https://www.netlify.com/
- Vercel: https://vercel.com/
- GitHub Pages: https://pages.github.com/

### Documentation Frameworks

- Docusaurus: https://docusaurus.io/
- mdBook: https://rust-lang.github.io/mdBook/
- VitePress: https://vitepress.dev/

### SSL/HTTPS

- Let's Encrypt: https://letsencrypt.org/ (automatic with Netlify/Vercel)
- `.dev` TLD Requirements: https://get.dev/

---

## 🚀 Deployment Strategy

**Approach**: Progressive rollout

1. **Phase 1**: GitHub URLs (current - working) ✅
2. **Phase 2**: Set up infrastructure (this checklist)
3. **Phase 3**: Deploy basic site with ERROR_CODES.md
4. **Phase 4**: Test with `FERRIS_DOCS_BASE` env var locally
5. **Phase 5**: Announce docs site, update README
6. **Phase 6**: Enhance site with guides and examples

**No rush**: GitHub URLs work fine. Can complete infrastructure any time before/after v0.0.3.

---

## 📝 Notes

- ✅ Domain purchased: October 6, 2025
- 🎯 Work can proceed in parallel with feature development
- ⚡ No compiler changes needed (hybrid URL system already implemented)
- 📖 Infrastructure work is documented in [PHASE_3_ERROR_DOCS_RECOVERY.md](./planning/v0.0.3/PHASE_3_ERROR_DOCS_RECOVERY.md#documentation-website-deferred-to-phase-9-or-v004)

---

**Last Updated**: October 6, 2025  
**Next Review**: When starting Phase 9 or v0.0.4 planning
