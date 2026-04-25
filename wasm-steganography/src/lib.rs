use wasm_bindgen::prelude::*;
use image::{load_from_memory, ImageOutputFormat, RgbaImage};
use std::io::Cursor;
use yew::prelude::*;
use web_sys::{HtmlInputElement, Blob, BlobPropertyBag, Url, File};

#[wasm_bindgen]
pub fn encode(image_data: &[u8], secret_message: &str) -> Result<Vec<u8>, JsValue> {
    let mut img: RgbaImage = load_from_memory(image_data)
        .map_err(|e| JsValue::from_str(&format!("Failed to load image: {}", e)))?
        .to_rgba8();

    let mut message_bytes = secret_message.as_bytes().to_vec();
    message_bytes.push(0); 

    let mut bits = message_bytes.into_iter().flat_map(|b| {
        (0..8).map(move |i| (b >> (7 - i)) & 1)
    });

    for pixel in img.pixels_mut() {
        for channel in 0..3 { 
            if let Some(bit) = bits.next() {
                pixel[channel] = (pixel[channel] & 0xFE) | bit;
            }
        }
    }

    let mut output = Cursor::new(Vec::new());
    img.write_to(&mut output, ImageOutputFormat::Png)
        .map_err(|e| JsValue::from_str(&format!("Failed to encode to PNG: {}", e)))?;

    Ok(output.into_inner())
}

#[wasm_bindgen]
pub fn decode(image_data: &[u8]) -> Result<String, JsValue> {
    let img: RgbaImage = load_from_memory(image_data)
        .map_err(|e| JsValue::from_str(&format!("Failed to load image: {}", e)))?
        .to_rgba8();

    let mut current_byte = 0u8;
    let mut bit_count = 0;
    let mut message_bytes = Vec::new();

    'outer: for pixel in img.pixels() {
        for channel in 0..3 {
            let bit = pixel[channel] & 1;
            current_byte = (current_byte << 1) | bit;
            bit_count += 1;

            if bit_count == 8 {
                if current_byte == 0 {
                    break 'outer;
                }
                
                message_bytes.push(current_byte);
                current_byte = 0;
                bit_count = 0;
            }
        }
    }

    String::from_utf8(message_bytes)
        .map_err(|e| JsValue::from_str(&format!("Decoded bytes are not valid UTF-8: {}", e)))
}

#[function_component(App)]
pub fn app() -> Html {
    let secret_message = use_state(|| String::new());
    let selected_file = use_state(|| None::<File>);
    let output_image_url = use_state(|| None::<String>);
    let decoded_message = use_state(|| None::<String>);
    let error_message = use_state(|| None::<String>);

    let on_file_change = {
        let selected_file = selected_file.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    selected_file.set(Some(file));
                }
            }
        })
    };

    let on_message_change = {
        let secret_message = secret_message.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            secret_message.set(input.value());
        })
    };

    let on_encode = {
        let selected_file = selected_file.clone();
        let secret_message = secret_message.clone();
        let output_image_url = output_image_url.clone();
        let error_message = error_message.clone();

        Callback::from(move |_| {
            let selected_file = selected_file.clone();
            let secret_message = secret_message.clone();
            let output_image_url = output_image_url.clone();
            let error_message = error_message.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Some(file) = (*selected_file).clone() {
                    let msg = (*secret_message).clone();
                    if msg.is_empty() {
                        error_message.set(Some("Please enter a secret message".to_string()));
                        return;
                    }

                    match wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await {
                        Ok(val) => {
                            let uint8_arr = js_sys::Uint8Array::new(&val);
                            let mut bytes = vec![0; uint8_arr.length() as usize];
                            uint8_arr.copy_to(&mut bytes);

                            match encode(&bytes, &msg) {
                                Ok(encoded_bytes) => {
                                    let mut bag = BlobPropertyBag::new();
                                    bag.type_("image/png");
                                    let encoded_uint8_arr = js_sys::Uint8Array::from(encoded_bytes.as_slice());
                                    let parts = js_sys::Array::new();
                                    parts.push(&encoded_uint8_arr);

                                    match Blob::new_with_u8_array_sequence_and_options(&parts, &bag) {
                                        Ok(blob) => {
                                            match Url::create_object_url_with_blob(&blob) {
                                                Ok(url) => {
                                                    output_image_url.set(Some(url));
                                                    error_message.set(None);
                                                },
                                                Err(_) => error_message.set(Some("Failed to create URL".to_string()))
                                            }
                                        },
                                        Err(_) => error_message.set(Some("Failed to create Blob".to_string()))
                                    }
                                },
                                Err(e) => error_message.set(Some(format!("Encoding Error: {:?}", e)))
                            }
                        },
                        Err(e) => error_message.set(Some(format!("File Read Error: {:?}", e)))
                    }
                } else {
                    error_message.set(Some("No file selected".to_string()));
                }
            });
        })
    };

    let on_decode = {
        let selected_file = selected_file.clone();
        let decoded_message = decoded_message.clone();
        let error_message = error_message.clone();

        Callback::from(move |_| {
            let selected_file = selected_file.clone();
            let decoded_message = decoded_message.clone();
            let error_message = error_message.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Some(file) = (*selected_file).clone() {
                    match wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await {
                        Ok(val) => {
                            let uint8_arr = js_sys::Uint8Array::new(&val);
                            let mut bytes = vec![0; uint8_arr.length() as usize];
                            uint8_arr.copy_to(&mut bytes);

                            match decode(&bytes) {
                                Ok(msg) => {
                                    decoded_message.set(Some(msg));
                                    error_message.set(None);
                                },
                                Err(e) => {
                                    decoded_message.set(Some(format!("Error reading message: {:?}", e)));
                                    error_message.set(Some(format!("Decoding Error: {:?}", e)));
                                }
                            }
                        },
                        Err(e) => error_message.set(Some(format!("File Read Error: {:?}", e)))
                    }
                } else {
                    error_message.set(Some("No file selected".to_string()));
                }
            });
        })
    };

    html! {
        <main>
            <header>
                <h1>{"Zero-Trust Stego"}</h1>
                <p class="subtitle">{"Securely conceal cryptographic intel within image data."}</p>
            </header>

            <section class="glass-card" aria-label="Input Configuration">
                <div class="input-group" style="margin-bottom: 1.5rem;">
                    <label for="image-upload">{"1. Target PNG Image"}</label>
                    <input type="file" id="image-upload" accept="image/png" onchange={on_file_change} aria-label="Upload Target PNG Image" />
                </div>
                
                <div class="input-group" style="margin-bottom: 2rem;">
                    <label for="secret-message">{"2. Classified Payload"}</label>
                    <textarea id="secret-message" placeholder="Type the confidential payload you wish to embed..." rows="4" aria-label="Secret text payload input" onchange={on_message_change}></textarea>
                </div>

                {
                    if let Some(err) = &*error_message {
                        html! {
                            <div style="color: #ef4444; margin-bottom: 1rem; font-weight: 600;">
                                {err}
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }

                <div class="action-buttons">
                    <button id="encode-btn" aria-label="Inject payload and Download Image" onclick={on_encode}>
                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect><path d="M7 11V7a5 5 0 0 1 10 0v4"></path></svg>
                        {"Inject & Download"}
                    </button>
                    <button id="decode-btn" aria-label="Extract payload from Image" onclick={on_decode}>
                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect><path d="M7 11V7a5 5 0 0 1 9.9-1"></path></svg>
                        {"Extract Payload"}
                    </button>
                </div>
            </section>

            <hr class="divider"/>

            <section class="output-section">
                <article class="glass-card output-box" aria-label="Extracted Payload Area">
                    <h3 class="decoded-title">{"Extracted Intel"}</h3>
                    <pre id="decoded-message" aria-live="polite">
                        {
                            if let Some(msg) = &*decoded_message {
                                msg.clone()
                            } else {
                                String::new()
                            }
                        }
                    </pre>
                </article>
                
                <article class="glass-card output-box" aria-label="Visual Output Area">
                    <h3 class="output-title">{"Steganographic Artifact Preview"}</h3>
                    <div class="image-preview-container">
                        {
                            if let Some(url) = &*output_image_url {
                                html! { <img id="output-image" alt="Embedded artifact will appear here" aria-live="polite" src={url.clone()} style="display: block;" /> }
                            } else {
                                html! {
                                    <>
                                        <img id="output-image" alt="Embedded artifact will appear here" aria-live="polite" />
                                        <div class="preview-placeholder">{"Awaiting steganographic operations..."}</div>
                                    </>
                                }
                            }
                        }
                    </div>
                </article>
            </section>
        </main>
    }
}

#[wasm_bindgen]
pub fn run_app() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let root = document.get_element_by_id("yew-root").unwrap();
    yew::Renderer::<App>::with_root(root).render();
}
