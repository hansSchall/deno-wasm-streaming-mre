# deno-wasm-streaming-mre

MRE for https://github.com/denoland/deno/issues/18547

Although the issue was opened for WASM streaming, it seems to be not an issue
with streaming only. The issue happens with manual instantiation, too, but
streaming behaves slightly different.

You can comment on this in the `Discussions` section

## Disclaimer

It might look like this is an issue with rust atomics, wasm_bindgen,
wasm_bindgen_futures or using tokio in wasm.

BUT: Deno behaves differently than **all other** runtimes: Bun, Node.js,
Chromium-browsers and Firefox

On these platforms everything works as expected while on Deno weird things
happen.

## Reproduce

### Prerequisites

- `Deno` (of course)
- `Node.js`
- `Bun`
- `Rust`
- `wasm-pack`

### Compiling backend

1. `rustup override set nightly`
2. `wasm-pack build --target web`

### Running

For browsers: `deno serve -A server.ts`

#### `streaming.js`

Uses the default, which is streaming from fetch

Works with: Deno, Bun, Browsers (linked by index.html)

#### `deno.js`

Loads the WASM and does manual instantiation

Works with: Deno

#### `node.js`

Loads the WASM and does manual instantiation

Works with: Node

## Expected output

The expected result is:

```
A
B
C
D
E
finished
```

Deno streaming prints:

```
A
B
C
```

Deno manual prints:

```
A
B
C
D
```

There is no code that terminates Deno (feel free to double-check this), but the
process exits with (micro)tasks waiting and unresolved promises.

## Things to try out

### Add timeouts

When you set a timeout (comment out the respective lines in `main.js` and
`src/lin.rs`), more code is run _just before_ the timeout is executed.

This indicates that some tasks are pushed to the queue, but are forgotten there
until the timeout triggers a new event loop iteration.

### Attach a debugger

Some missing code is executed _after_ the message
`Program finished. Waiting for inspector to disconnect to exit the process...`
