# editor

WIP rw editor/renderer using wasm, rust and webgl.

Uses https://github.com/asny/three-d/ for internals

## hacking

`npm run dev` to run the dev server which does hot-reload and stuff, `npm run build` to just build the js+wasm to `pkg`

in webpack config there is `WasmPackPlugin` which uses the root dir as the main crate, this crate does the scene rendering and stuff and it imports the three-d crate which abstracts all the 3d stuff.

`index.js` does some stuff to resize the canvas but some of it doesn't work (FPS) because that's handled also by the rust code - this needs to be unified somewhere at some point.
