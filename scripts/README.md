# FerrisScript Scripts

This directory contains helper scripts for development workflows.

## Documentation Linting

### Prerequisites

- **Node.js** (v16 or higher): [Download here](https://nodejs.org/)
- **npm**: Comes with Node.js

### Installation

Run once to install dependencies:

```powershell
npm install
```

### Usage Options

#### Option 1: VS Code Tasks (Recommended)

Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on Mac), type "Run Task", and select:

- **Docs: Full Check** - Runs both linting and link checking
- **Docs: Lint All** - Check markdown formatting
- **Docs: Check Links** - Verify all links work
- **Docs: Fix Issues** - Auto-fix markdown formatting issues
- **Docs: PowerShell Lint** - Use PowerShell script instead
- **Docs: PowerShell Fix** - Use PowerShell script with auto-fix

#### Option 2: npm Scripts

```powershell
# Check markdown formatting
npm run docs:lint

# Auto-fix markdown formatting issues
npm run docs:fix

# Note: For link checking, use PowerShell script (Option 3)
# Link checking requires platform-specific commands
```

#### Option 3: PowerShell Script

```powershell
# Check only
.\scripts\lint-docs.ps1

# Check and auto-fix
.\scripts\lint-docs.ps1 --fix
```

### What Gets Checked

#### Markdownlint

- Heading styles and hierarchy
- List formatting
- Code block formatting
- Line length (relaxed to 120 chars)
- Trailing spaces
- And more... (see `.markdownlint.json`)

#### Markdown Link Check

- All internal links (between docs)
- External links (with retries and timeout handling)
- Relative paths
- Anchor links

### Configuration Files

- **`.markdownlint.json`** - Markdown formatting rules
- **`.markdown-link-check.json`** - Link checking behavior (timeouts, retries, ignored domains)

### CI Integration

Documentation linting runs automatically on:

- Pull requests (when `.md` files change)
- Pushes to main branch

See `.github/workflows/docs-lint.yml` for CI configuration.

### Troubleshooting

#### "Node.js is not installed"

Download and install from [nodejs.org](https://nodejs.org/)

#### "npm: command not found"

npm comes with Node.js. Restart your terminal after installing Node.js.

#### "Module not found"

Run `npm install` to install dependencies

#### Link check fails for external sites

Some sites may block automated requests. Check `.markdown-link-check.json` to configure retries or ignore specific domains.

### Adding New Scripts

1. Create script in `scripts/` directory
2. Add entry to `package.json` scripts section (if npm-based)
3. Add VS Code task in `.vscode/tasks.json` (optional)
4. Document here in this README

## Future Scripts

Planned additions:

- `generate-toc.ps1` - Auto-generate table of contents
- `check-copyright.ps1` - Verify copyright headers
- `sync-versions.ps1` - Keep version numbers in sync
