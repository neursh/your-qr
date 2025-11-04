# QRGen WASM
A WASM package to generate QR codes!

This is the backbone for Kewar to generate QR code on client side, very important init.

Tbh it's just the `qrcode` crate but I added a layer of converting results to basic components to pass to js, and a way to convert it to svg.

With the advantage of using wasm, the QR code can be changed immediately as the user types with minimal impact on performance, even when the input reaches thousands of characters, smooth like butter 🧈

One thing that I'm trying to figure out is how can I style the svg without sacrificing performance.
