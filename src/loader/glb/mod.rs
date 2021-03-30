use crate::definitions::gltf;
use crate::definitions::gltf::GlTf;
use js_sys::ArrayBuffer;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

const HEADER_MAGIC: u32 = 0x46546C67;
const HEADER_VERSION_SUPPORT: u32 = 2;
const CHUNK_TYPE_JSON: u32 = 0x4E4F534A;
const CHUNK_TYPE_BIN: u32 = 0x004E4942;

#[wasm_bindgen]
extern "C" {
    pub fn alert(msg: &str);
}

#[derive(Debug)]
struct GlbHeader {
    magic: u32,
    version: u32,
    length: u32,
}

impl GlbHeader {
    fn new(bin: &ArrayBuffer) -> Result<GlbHeader, JsValue> {
        let header_magic = js_sys::Uint32Array::new_with_byte_offset_and_length(&bin, 0, 1);
        let header_version = js_sys::Uint32Array::new_with_byte_offset_and_length(&bin, 4, 1);
        let header_length = js_sys::Uint32Array::new_with_byte_offset_and_length(&bin, 8, 1);

        let magic = header_magic.get_index(0);
        let version = header_version.get_index(0);
        let length = header_length.get_index(0);

        // Todo: Improve error handling
        assert_eq!(&HEADER_MAGIC, &magic);
        assert_eq!(&HEADER_VERSION_SUPPORT, &version);
        assert_eq!(&bin.byte_length(), &length);

        Ok(GlbHeader {
            magic,
            version,
            length,
        })
    }
}

#[derive(Debug)]
pub(crate) struct GlbJson {
    byte_length: u32,
    pub(crate) gltf: GlTf,
}

impl GlbJson {
    fn new(bin: &ArrayBuffer) -> Result<GlbJson, JsValue> {
        let json_chunk_length = js_sys::Uint32Array::new_with_byte_offset_and_length(&bin, 12, 1);
        let chunk_type = js_sys::Uint32Array::new_with_byte_offset_and_length(&bin, 16, 1);

        // Todo: Improve error handling
        assert_eq!(&CHUNK_TYPE_JSON, &chunk_type.get_index(0));

        let byte_length = json_chunk_length.get_index(0);
        let content_chunk =
            js_sys::Uint8Array::new_with_byte_offset_and_length(&bin, 20, byte_length);

        let decoder = web_sys::TextDecoder::new()?;
        let json = decoder.decode_with_buffer_source(&content_chunk)?;

        let gltf: GlTf = serde_json::from_str(&json).or_else(|e| {
            return Err(JsValue::from_str(&e.to_string()));
        })?;

        Ok(GlbJson {
            byte_length,
            gltf: gltf,
        })
    }
}

#[derive(Debug)]
pub(crate) struct GlbBuffer {
    pub(crate) byte_offset: u32,
    pub(crate) byte_length: u32,
    pub(crate) bin: ArrayBuffer,
}

impl GlbBuffer {
    fn new(bin: ArrayBuffer, json_chunk: &GlbJson) -> GlbBuffer {
        let byte_offset = 20 + json_chunk.byte_length;
        let bin_chunk_length =
            js_sys::Uint32Array::new_with_byte_offset_and_length(&bin, byte_offset, 1);
        let chunk_type =
            js_sys::Uint32Array::new_with_byte_offset_and_length(&bin, byte_offset + 4, 1);

        assert_eq!(&CHUNK_TYPE_BIN, &chunk_type.get_index(0));

        let byte_length = bin_chunk_length.get_index(0);

        GlbBuffer {
            byte_offset: byte_offset + 8,
            byte_length,
            bin,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Glb {
    header: GlbHeader,
    pub(crate) json: GlbJson,
    pub(crate) buffer: GlbBuffer,
}

impl Glb {
    fn new(bin: ArrayBuffer) -> Result<Glb, JsValue> {
        let header = GlbHeader::new(&bin)?;
        let json = GlbJson::new(&bin)?;
        let buffer = GlbBuffer::new(bin, &json);

        Ok(Glb {
            header,
            json,
            buffer,
        })
    }

    pub async fn from_url(url: &str) -> Result<Glb, JsValue> {
        let window = web_sys::window().expect("could not get window object");

        let response = JsFuture::from(window.fetch_with_str(url))
            .await?
            .dyn_into::<Response>()?;

        let bin = JsFuture::from(response.array_buffer()?)
            .await?
            .dyn_into::<ArrayBuffer>()?;

        Glb::new(bin)
    }
}
