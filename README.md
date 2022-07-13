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

whats inside? where is it? how to use it independently?

## bevy inventory ui

## bevy roguelike combat

## map generator

## vec walk dir

## License

Using the same license as bevy engine does.

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

## Credits

None of this would be possible if not the [wonderful work done by others](credits/CREDITS.md). This is my first rust project so "would be possible" applies even more :).
