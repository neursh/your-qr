# QRGen WASM
A WASM package to generate QR codes!

This is the backbone for Kewar to generate QR code on client side, so it's very important init.

Tbh it's just the `qrcode` crate but I added a layer of converting results to basic components to pass to js, and a way to convert it to svg.

With the advantage of using wasm, the QR code can change immediately as the user types with minimal impact on performance even when the input reaches thousands of characters, smooth like butter 🧈
