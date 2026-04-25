import init, { encode, decode } from './pkg/wasm_steganography.js';

async function setup() {
    await init();
    
    const fileInput = document.getElementById('image-upload');
    const secretInput = document.getElementById('secret-message');
    const outputImage = document.getElementById('output-image');
    const decodedMessage = document.getElementById('decoded-message');

    async function getUint8ArrayOfFile(file) {
        if (!file) throw new Error('No file selected.');
        const arrayBuffer = await file.arrayBuffer();
        return new Uint8Array(arrayBuffer);
    }

    document.getElementById('encode-btn').addEventListener('click', async () => {
        try {
            const file = fileInput.files[0];
            const message = secretInput.value;
            
            if (!message) throw new Error('Please enter a secret message');
            
            const uint8Array = await getUint8ArrayOfFile(file);
            const encodedImageBytes = encode(uint8Array, message);
            
            const blob = new Blob([encodedImageBytes], { type: 'image/png' });
            outputImage.src = URL.createObjectURL(blob);
            
            alert('Image encoded successfully! Right-click the output to save.');
        } catch (e) {
            console.error('Encoding Failure:', e);
            alert(`Error: ${e}`);
        }
    });

    document.getElementById('decode-btn').addEventListener('click', async () => {
        try {
            const file = fileInput.files[0];
            const uint8Array = await getUint8ArrayOfFile(file);

            const hiddenMessage = decode(uint8Array);
            decodedMessage.textContent = hiddenMessage || "<empty message>";
            
        } catch (e) {
            console.error('Decoding Failure:', e);
            decodedMessage.textContent = `Error reading message: ${e}`;
        }
    });
}

setup();
