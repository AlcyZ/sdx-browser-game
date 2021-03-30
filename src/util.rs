pub(crate) fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub(crate) fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}
