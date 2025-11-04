echo Building QRGen WASM...
cd qrgen
cargo install wasm-pack
wasm-pack build --target web
cd ..

echo Building server...
cd server
cargo build --release
cd ..

echo Building web...
cd web
yarn build
cd ..
