import init, { run_app } from './pkg/wasm_steganography.js';

async function setup() {
    await init();
    run_app();
}

setup();
