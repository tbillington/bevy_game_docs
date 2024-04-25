Proof of concept demonstrating usage of rustdoc json to generate Bevy game docs.

Mostly just interested in getting peoples thoughts on such a tool!

https://tbillington.github.io/bevy_game_docs/

<img width="451" alt="image" src="https://github.com/tbillington/bevy_game_docs/assets/2771466/dd1b32ab-e846-4877-8946-3bda232c632e">

Goals
- Make it easy to see components, resources, and systems across your project at a glance
- Easily see which queries a component is referenced in and whether it's mutable accessed
- Apply lints/warnings, eg prevent a component being used mutably outside it's module

To generate rustdoc json:

```sh
cargo +nightly rustdoc --lib -p eng -- --document-private-items -Z unstable-options --output-format json
```

Rebuild styles:

```sh
npx tailwindcss -i ./style.css -o ./static/style.css --watch
```

Uses zola to generate html:

```sh
zola build
```