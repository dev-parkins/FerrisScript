From GPT 5:

Cool, I poked around the godot-rust docs & trait definitions and can propose concrete override stubs + a refined recommendation given your blocker. Here’s what I found + what I’d try next.

---

## 🔍 Findings from godot-rust docs / trait definitions

* The trait **`IObject`** defines `get_property`, `set_property`, `get_property_list`, `validate_property`, and `property_get_revert` as overridable methods. ([Godot Rust][1])
* The doc says `get_property_list` is **available only since API version 4.3**. ([Godot Rust][1])
* The “Custom Resources” doc uses exactly this override pattern: in a `#[godot_api] impl IResource`, define `get_property`, `set_property`, etc. ([Godot Rust][2])
* It also mentions that to have editor-time (Inspector) logic, the class must be annotated with `#[class(tool)]`. ([Godot Rust][2])
* The `IObject` docs state that returning `None` from `get_property` means “fallback to normal handling,” and returning `false` from `set_property` signals default behavior should proceed. ([Godot Rust][1])

So yes — there *is* an intended override path via `IObject` in godot-rust, and the pattern you sketched (Option 1) is plausible.

However, a couple of caveats:

* If your Godot / API version is less than 4.3, `get_property_list` may not be called (or may not exist) — so Inspector won’t see dynamic properties.
* Even with overrides, the default property system may already register “native” getters/setters for exported properties, so your overrides may or may not be invoked depending on ordering.
* Without `#[class(tool)]`, the overrides may not run in the editor context, meaning Inspector changes may not route to your logic at edit time.

---

## 🛠 Override stub code to try (for your version)

Here’s a minimal stub you can drop into your class (in your `#[godot_api] impl` block) to test whether override hooks are honored:

```rust
#[godot_api]
impl INode2D for FerrisScriptNode {
    // Called when Godot wants to read a property value
    fn get_property(&self, name: StringName) -> Option<Variant> {
        let prop_name = name.to_string();
        godot_print!("get_property called for '{}'", prop_name);
        // Try to read from your runtime env
        match self.env.get_exported_property(&prop_name) {
            Ok(value) => {
                // convert FerrisScript Value -> Variant
                Some(self.value_to_variant(&value))
            }
            Err(_) => None  // fallback to default behavior
        }
    }

    // Called when Godot wants to write / set a property
    fn set_property(&mut self, name: StringName, v: Variant) -> bool {
        let prop_name = name.to_string();
        godot_print!("set_property called for '{}'", prop_name);
        // Convert Variant -> Value
        match self.variant_to_value(&v) {
            Ok(val) => {
                if let Err(errmsg) = self.env.set_exported_property(&prop_name, val, true) {
                    godot_warn!("Failed to set exported property '{}': {}", prop_name, errmsg);
                    return false; // fallback to default
                }
                true  // handled by us
            }
            Err(_) => {
                godot_warn!("Invalid variant -> Value conversion for prop '{}'", prop_name);
                false  // fallback
            }
        }
    }

    // Optionally override validate_property
    fn validate_property(&self, prop: &mut PropertyInfo) {
        // Called when Godot is reflecting property metadata; you can tweak hint, name, etc.
        godot_print!("validate_property: {}", prop.name());
        // Example: if prop.name == "foo", change hint
    }

    // Optionally override revert
    fn property_get_revert(&self, name: StringName) -> Option<Variant> {
        let prop_name = name.to_string();
        if let Ok(default_val) = self.env.get_default_exported_property(&prop_name) {
            Some(self.value_to_variant(&default_val))
        } else {
            None
        }
    }
}
```

You might need to annotate your class with:

```rust
#[class(tool)]
```

so that code runs in the editor (Inspector) as well as in game runtime. That’s often required for overrides to work in editor mode. ([Godot Rust][2])

Additionally, ensure your class’s registration macro (derive or `GodotClass`) does not suppress or bypass `IObject` overrides.

---

## 🔄 Updated recommendation given the blocker

Given the blocker status (you’re paused on which approach works), here’s a refined plan:

1. **Insert the stub above**, build & attach to a test scene, and log whether `get_property` / `set_property` are triggered from Inspector / runtime changes.

   * If **both** override hooks fire correctly (in editor and runtime), that’s your approach. Proceed with Bundle 7 using that.
   * If only runtime works (but not editor), you may need `#[class(tool)]` or conditional logic.
   * If neither override gets called, fallback to Option 2 (IObject trait) or Option 3 (_get/_set) must be examined.

2. **Check that exported properties don’t “steal” the value** before your override runs.

   * If the generated backing field code has higher priority, your override may never be reached.
   * If that’s the case, you must disable the default backing for exports or ensure your override “wins” (by returning `Some(...)`/`true` always).

3. **Test fallback behavior**: returns `None` / `false` correctly goes to default property behavior (for non-exported or unknown props).

4. **Test Hot reload / script reload** to ensure that after reloading, your override stub is still active.

5. **Document results**, then bake the final override stub and integrate into Bundle 7.

---

If you like, I can produce a **diff patch** for your `crates/godot_bind` or your node-binding file, showing exactly where to drop these stubs, conditionalized, so you can test with minimal effort. Do you want me to generate that for you now?

[1]: https://godot-rust.github.io/docs/gdext/master/godot/prelude/trait.IObject.html?utm_source=chatgpt.com "IObject in godot::prelude - Rust"
[2]: https://godot-rust.github.io/book/recipes/custom-resources.html?utm_source=chatgpt.com "Custom resources - The godot-rust book"
