# bevy roguelike

Meet my new baby. It lives [here](https://tomuxmon.github.io/bevy_roguelike/). A turn based system where each actor has combat capabilities influenced by equiped items. I had a burning desire to learn [Rust](https://www.rust-lang.org/). It appears I also like creating games. Wonderful [Bevy](https://bevyengine.org/) engine was used. Bevy uses only 4 letters to define itself, and so it is concise, and so is Rust. Also humans tend to forget what is in the middle of the text. No one will remember this sentence. Jokes aside It felt really natural to define data structures and write systems that manipulate those data structures. This is my first Rust project so bugs are possible as much as Rust and Bevy allows it. Preview of the running system below.

![a scene from a running game](example.png)

## Running it

To run this game locally you need [rust installed](https://www.rust-lang.org/tools/install). Then you can run it with:

- `cargo run` (the usual rust build)

If you would like to try it in the browser (inspiration from [bevy_game_template](https://github.com/NiklasEi/bevy_game_template)) you will also need to [install trunk](https://trunkrs.dev/#install), add `wasm32-unknown-unknown` rust target with `rustup target add wasm32-unknown-unknown` command. Then you can try it with:

- `trunk serve` (serve wasm build)

Check out the [live version here](https://tomuxmon.github.io/bevy_roguelike/).

You can also run and debug it inside VS Code with breakpoints and all the goodness. [launch.json](.vscode/launch.json) together with a couple of extensions allows that. Personally I just use [rust-extension-pack](https://marketplace.visualstudio.com/items?itemName=Zerotaskx.rust-extension-pack).

### Controlls

- `up` / `down` / `left` / `right` keys for movement and attack
- `,` to pick up item
- `I` to open / close inventory display
- `D` to drop item (last item from inventory or else equipment)

### Inventiory management

![inventory image](inventory.png)

- `hover` over an item in the inventory / equipment to display a hover tip
- `mouse click` an item to equip / unequip it

## bevy inventory

This crate contains basic implementation of the inventory and equipment containers. Items are Bevy's [Entities](https://docs.rs/bevy/latest/bevy/prelude/struct.Entity.html). Both inventory and equipment implement utility methods to `take` and `add` items. The only difference between inventory and equipment is that the equipment also has a generic type parameter to specify item type. Inventory and Equipment structures below.

```rust
//...

pub trait ItemType: Component + Copy + Clone + Eq + Hash + Debug + Default {}

#[derive(Debug, Clone, Component)]
pub struct Inventory {
    items: Vec<Option<Entity>>,
}

#[derive(Debug, Default, Clone, Component)]
pub struct Equipment<I: ItemType> {
    pub items: HashMap<(I, u8), Option<Entity>>,
}
//...
```

ItemType is a trait that needs to be implemented in user code. Something like this:

```rust
#![allow(dead_code)]
use bevy::prelude::*;
use bevy_inventory::ItemType;

/// deriving traits required by `ItemType`
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Component)]
pub enum SomeItemType {
    #[default]
    RightHandGear,
    LeftHandGear,
    BodyWear,
}
/// manaully implementing. no macro for that so far.
impl ItemType for SomeItemType {}
```

`bevy_inventory` is really minimal and it does not contain any bevy systems also there is no plugin for it. To use it just copy it into your workspace or just copy paste what you actually need. If you have some suggestions or know something better that I could use instead please mention it in the discussions.

## bevy inventory ui

## bevy roguelike combat

## map generator

## vec walk dir

They say you can not [load asset folder on the web](https://github.com/bevyengine/bevy/issues/2916). Yes it is true. But yet again, we know asset folder structure at compile time. A dirty macro workaround to the rescue. It is not a good idea to rely on reading folder structure in a macro. But I wanted to write my first macro.

```rust
#[cfg(target_arch = "wasm32")]
{
    let asset_files: Vec<&str> = vec_walk_dir::vec_walk_dir!("assets");
    for file in asset_files.into_iter() {
        asset_server.load_untyped(file);
    }
}
```

### Inspiration and learning sources

- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/introduction.html)
- [Procedural Macros tutorial series](https://blog.turbo.fish/proc-macro-basics/)

## License

Using the same license as bevy engine does.

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

## Credits

None of this would be possible if not the [wonderful work done by others](credits/CREDITS.md). This is my first rust project so "would be possible" applies even more :).
