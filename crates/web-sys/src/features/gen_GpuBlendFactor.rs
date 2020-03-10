#![allow(unused_imports)]
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
#[doc = "The `GpuBlendFactor` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuBlendFactor`*"]
#[doc = ""]
#[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
#[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuBlendFactor {
    Zero = "zero",
    One = "one",
    SrcColor = "src-color",
    OneMinusSrcColor = "one-minus-src-color",
    SrcAlpha = "src-alpha",
    OneMinusSrcAlpha = "one-minus-src-alpha",
    DstColor = "dst-color",
    OneMinusDstColor = "one-minus-dst-color",
    DstAlpha = "dst-alpha",
    OneMinusDstAlpha = "one-minus-dst-alpha",
    SrcAlphaSaturated = "src-alpha-saturated",
    BlendColor = "blend-color",
    OneMinusBlendColor = "one-minus-blend-color",
}