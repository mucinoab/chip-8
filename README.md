# Chip 8 Emulator

A [Chip 8](https://en.wikipedia.org/wiki/CHIP-8) emulator with a web front end
using WASM.

[Live Demo](https://mucinoab.github.io/en/proyectos/#chip-8)

## How to Run

Build the WASM file by running the following command (you will need to have
[wasm-pack](https://github.com/rustwasm/wasm-pack) installed).

```bash
cd frontend && wasm-pack build --release --target web --out-dir .
```

Now run any HTTP server, for simplicity I recommend python.

```bash
python -m http-server
```
