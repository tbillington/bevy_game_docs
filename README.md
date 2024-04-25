Proof of concept demonstrating usage of rustdoc json to generate Bevy game docs.

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