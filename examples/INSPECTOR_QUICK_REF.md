# Inspector Test - Quick Reference Card

## 🚀 Quick Setup (30 seconds)

```powershell
# 1. Compile
cargo build --package ferrisscript_godot_bind

# 2. Open Godot Editor
# 3. Create Node2D → Attach Script → Set path to:
res://examples/inspector_test.ferris

# 4. Check Inspector - should see 20+ properties!
```

---

## 📋 What's Being Tested

### 8 Property Types
```ferris
@export let mut health: i32 = 100;                    // Integer
@export let mut speed: f32 = 5.5;                     // Float
@export let mut alive: bool = true;                   // Boolean
@export let mut name: String = "Hero";                // String
@export let mut pos: Vector2 = Vector2 { x, y };      // Vector2
@export let mut color: Color = Color { r, g, b, a };  // Color
@export let mut rect: Rect2 = Rect2 { ... };          // Rect2
@export let mut xform: Transform2D = Transform2D { ... }; // Transform2D
```

### 4 Property Hints
```ferris
@export(range(0, 100, 1))              // 🎚️ Slider control
@export(enum("A", "B", "C"))           // 📋 Dropdown menu
@export(file("*.png", "*.jpg"))        // 📁 File picker
@export                                // ⚙️ Default widget
```

---

## ✅ 10-Second Smoke Test

1. **Open scene** with FerrisScriptNode
2. **Check Inspector** → See properties? ✅
3. **Change health** to 75
4. **Run scene (F5)** → Console shows "Health: 75"? ✅
5. **Done!** Inspector is working! 🎉

---

## 🧪 Critical Tests (5 minutes)

### Test 1: Property Display (30s)
- [ ] Inspector shows all properties
- [ ] Default values correct
- [ ] Sliders for range properties
- [ ] Dropdowns for enum properties

### Test 2: Property Reading (30s)
- [ ] Run scene (F5)
- [ ] Console prints all values
- [ ] Values match Inspector

### Test 3: Property Writing (1 min)
- [ ] Change `player_health` to 50
- [ ] Run scene
- [ ] Console shows "Health: 50"
- [ ] Change `character_class` to "Mage"
- [ ] Console shows "Class: Mage"

### Test 4: Range Clamping (1 min)
- [ ] Set `stamina` to 150 → clamps to 100
- [ ] Set `stamina` to -10 → clamps to 0
- [ ] Set `opacity` to 1.5 → clamps to 1.0

### Test 5: Hot-Reload (1 min)
- [ ] Edit `inspector_test.ferris`
- [ ] Change health default: 100 → 200
- [ ] Save file
- [ ] Inspector updates automatically

### Test 6: Runtime Updates (1 min)
- [ ] Run scene (F5)
- [ ] Watch console
- [ ] `rotation_degrees` increments
- [ ] `stamina` regenerates to 100
- [ ] No crashes after 30 seconds

---

## 🎯 Expected Inspector Layout

```
┌─ FerrisScriptNode ─────────────────────────────┐
│                                                  │
│ Script Path: res://examples/inspector_test.ferris│
│                                                  │
│ ▼ Basic Types                                   │
│   Player Health:      100      [slider]         │
│   Movement Speed:     5.5      [slider]         │
│   Is Alive:           ☑ true   [checkbox]       │
│   Player Name:        "Hero"   [text]           │
│                                                  │
│ ▼ Range Hints                                   │
│   Stamina:            50       [0━━●━━━━━100]   │
│   Run Speed:          7.5      [0.0━━●━━10.0]   │
│   Temperature:        20       [-100━●━━100]    │
│   Rotation Degrees:   0        [0━●━━━━━━360]   │
│   Opacity:            1.0      [0.0━━━━━●1.0]   │
│                                                  │
│ ▼ Enum Hints                                    │
│   Character Class:    [Warrior ▼]               │
│                       ├ Warrior                 │
│                       ├ Mage                    │
│                       ├ Rogue                   │
│                       └ Ranger                  │
│   Difficulty:         [Normal ▼]                │
│   Team Color:         [Red ▼]                   │
│                                                  │
│ ▼ File Hints                                    │
│   Avatar Texture:     (empty)  [📁 Browse...]   │
│   Sound Effect:       (empty)  [📁 Browse...]   │
│   Spawn Scene:        (empty)  [📁 Browse...]   │
│                                                  │
│ ▼ Godot Struct Types                            │
│   Spawn Position:     Vector2                   │
│     X: 100.0                                     │
│     Y: 200.0                                     │
│   Tint Color:         [🎨 Color Picker]         │
│     R: 1.0  G: 0.5  B: 0.0  A: 1.0             │
│   Collision Rect:     Rect2                     │
│     Position: (0.0, 0.0)                        │
│     Size: (64.0, 64.0)                          │
│   Spawn Transform:    Transform2D               │
│     Position: (0.0, 0.0)                        │
│     Rotation: 0.0                               │
│     Scale: (1.0, 1.0)                           │
│                                                  │
└──────────────────────────────────────────────────┘
```

---

## 🐛 Quick Troubleshooting

### Properties Not Showing?
```rust
// Check line 357 in godot_bind/src/lib.rs:
#[class(base=Node2D, tool)]  // ← Must have 'tool'
```

### Changes Not Saving?
```ferris
@export let mut health: i32 = 100;  // ✅ 'mut' required
@export let health: i32 = 100;      // ❌ Won't save (immutable)
```

### Range Not Clamping?
```ferris
@export(range(0, 100, 1))           // ✅ Correct (min, max, step)
@export(range(0, 100))              // ❌ Missing step parameter
```

### Hot-Reload Not Working?
```
✅ res://examples/inspector_test.ferris  (Godot resource path)
❌ C:\...\inspector_test.ferris         (Absolute path won't hot-reload)
```

---

## 📊 Test Results Template

```
Date: ___________
Tester: ___________

Quick Tests:
[ ] Property Display
[ ] Property Reading  
[ ] Property Writing
[ ] Range Clamping
[ ] Hot-Reload
[ ] Runtime Updates

Overall: PASS / FAIL

Issues Found:
_______________________
_______________________
_______________________

Performance: Smooth / Laggy / Crashes

Notes:
_______________________
_______________________
```

---

## 🎉 Success Criteria

✅ **READY FOR v0.0.4 RELEASE** if:
- All 20+ properties visible
- Inspector read/write works
- Range clamping works
- Hot-reload works
- No crashes during normal use

🔧 **NEEDS WORK** if:
- Properties not visible
- Values don't save
- Crashes when changing properties
- Hot-reload broken

---

## 📞 Need Help?

1. Check console output (Godot Output panel)
2. Look for FerrisScript errors
3. Review `INSPECTOR_TEST_GUIDE.md` (full guide)
4. Check `docs/planning/v0.0.4/TESTING_STRATEGY_PHASE5.md`

---

**Created**: October 10, 2025  
**Version**: v0.0.4  
**Status**: Ready for testing

**Good luck! 🦀🎮**
