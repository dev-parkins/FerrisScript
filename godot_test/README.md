# RustyScript Test Project

This is a minimal Godot 4.x project for testing the RustyScript GDExtension.

## Quick Start

1. Build the extension:
   ```bash
   cargo build --package rustyscript_godot_bind
   ```

2. Open this project in Godot 4.2+

3. Run the test scene (F5)

4. Check Output panel for: "Hello, Godot!"

## What's Included

- `project.godot` - Godot project configuration
- `test_scene.tscn` - Scene with RustyScriptNode
- `icon.svg` - Godot default icon

## RustyScriptNode Properties

- **script_path**: Path to .rscr file (relative to Godot project or absolute)

## Methods

- **reload_script()**: Reload the current script (useful during development)

## Testing Different Scripts

Edit `test_scene.tscn` and change the `script_path` property to test different examples:

- `res://../examples/hello.rscr` - Print "Hello, Godot!"
- `res://../examples/branch.rscr` - Test if/else
- `res://../examples/loop.rscr` - Test while loops
- `res://../examples/functions.rscr` - Test function calls

## Troubleshooting

See `../docs/PHASE6_TESTING.md` for detailed troubleshooting guide.
