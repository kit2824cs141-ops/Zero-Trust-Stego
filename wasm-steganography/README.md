# Zero-Trust LSB Steganography (WASM & Yew)

A highly secure, browser-based image steganography tool that allows you to easily conceal cryptographic/confidential textual data within the Least Significant Bits (LSB) of PNG images. All cryptographic data manipulation and pixel shifting happens entirely on your local machine using WebAssembly (WASM)—meaning your original images, and the concealed text, never leave your browser context.

The application's modern and beautiful interface, alongside its internal logic, is fully built using **Rust** and the **Yew** frontend framework.

## 🚀 Key Features

- **Zero-Trust Architecture**: Absolutely no external backend requests. Everything runs locally in the browser utilizing WebAssembly.
- **LSB Encoding/Decoding Algorithm**: A highly performant pixel manipulation implementation natively using the Rust `image` crate.
- **Yew UI Framework**: Fully interactive, modern interface authored entirely in strongly-typed Rust (`src/lib.rs`).
- **Reactive Interactions**: Elegant DOM updates for missing files, empty messages, validation states, and encoding success.
- **Glassmorphism Design**: High-fidelity dark mode with dynamic visual gradients.

## 🛠 Prerequisites

To build and serve the application, make sure you have the following installed on your machine:
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Wasm-Pack](https://rustwasm.github.io/wasm-pack/installer/) for building the WebAssembly package
- A basic static HTTP server (e.g., `python -m http.server`)

## 🖥 Installation & Building

1. Make sure you are in the project root directory.
2. Build the WebAssembly binaries formatted for web target usage:
   ```bash
   wasm-pack build --target web
   ```
   *This command checks out your `Cargo.toml`, fetches necessary crates (like `yew`, `web-sys`), and compiles them into a compact `.wasm` file bundled natively inside the automatically generated `/pkg` directory.*

## 🏃 Running the Application

Because native ES Modules are utilized for the WebAssembly import (inside `main.js` and `index.html`), the project cannot be launched directly via double-clicking `index.html` from the file system. Instead, it must be run from a local HTTP server.

Run from your command line:
```bash
# Using Python
python -m http.server 8080
```

Now, navigate to: [http://localhost:8080](http://localhost:8080) in your web browser.

## 🔑 How to Use

### 1. Inject & Download (Encoding)
- Upload a standard PNG Image.
- Enter your classified/secret textual payload in the provided text area.
- Click **"Inject & Download"**.
- A preview of your new image (which appears identical to the original) will visually pop up on screen. Right-click and "Save Image As" to download your newly encoded payload carrier!

### 2. Extract Payload (Decoding)
- Upload an existing, encoded PNG image (this could be straight from your file directory).
- Click **"Extract Payload"**.
- Provided that there is a correctly mapped LSB encoded payload text array within the image, it will extract itself natively from WebAssembly and reveal your confidential string.

## 💻 Tech Stack
- Frontend Interface & DOM Core: [Yew](https://yew.rs/) (Rust)
- Package Tooling & DOM Binding: [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/), `wasm-pack`
- Pixel Manipulation: [image crate](https://crates.io/crates/image)
- Styling: Plain CSS
