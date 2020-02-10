<div align="center">

  <h1><code>wasm-pack-template-game-of-life</code></h1>

  <strong>A template for kick starting a Rust and WebAssembly project </strong>

  <sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>

</div>

## About

[**📚 Read this template tutorial! 📚**]

This template is designed for compiling Rust libraries into WebAssembly.


### 🗝 Add wasm32 target
```
rustup target add wasm32-unknown-unknown
```

### 🏗 Install wasm-pack
```
cargo install wasm-pack
```

### 🛠️ Build with `wasm-pack`
```
rustup run nightly wasm-pack build 
```

### 🛠️ Build with `cargo`
``` 
cargo build 
```

### 🛎 Install node dependencies and run
```
cd www && npm install && npm run start
```
