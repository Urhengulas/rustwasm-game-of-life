# rustwasm-game-of-life

Based on https://rustwasm.github.io/book/game-of-life/introduction.html

## 🚴 Usage

```bash
.$ wasm-pack build
.$ cd www
./www$ npm run start
# serves webpage on https://localhost:8080
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
