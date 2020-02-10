<div align="center">

  <h1><code>wasm-pack-game-of-life</code></h1>

  <strong>A project for kick starting Rust and WebAssembly </strong>

  <sub> <a href="https://rustwasm.github.io/"> Rust and WebAssembly Working Group</a></sub>

</div>

## About

[**📚 Clone this repo ! 📚**]

Run these commands ! 

### 🗝 Add `wasm32 target`
```
rustup target add wasm32-unknown-unknown
```

### 🏗 Install `wasm-pack`
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

### 🛎 Install `node` dependencies and run
```
cd www && npm install && npm run start
```

## Game of life rules

- Si une cellule morte a 3 cellules voisines vivantes alors elle devient vivante.
- Si une cellule vivante a 2 ou 3 cellules voisine vivante alors elle reste vivante.
- Si une cellule vivante a moins de 2 ou plus de 3 cellules voisines vivante alors elle devient morte.
