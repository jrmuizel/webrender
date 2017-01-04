extern crate webrender;
extern crate webrender_traits;

use webrender_traits::{ColorF, RendererKind};

fn main() {
    let res_path = None;

    let opts = webrender::RendererOptions {
        device_pixel_ratio: 1.0,
        resource_override_path: res_path,
        enable_aa: false,
        enable_profiler: false,
        enable_recording: false,
        enable_scrollbars: false,
        debug: true,
        precache_shaders: true,
        renderer_kind: RendererKind::Native,
        enable_subpixel_aa: false,
        clear_framebuffer: true,
        clear_empty_tiles: false,
        clear_color: ColorF::new(1.0, 1.0, 1.0, 1.0),
    };

    let (mut renderer, sender) = webrender::renderer::Renderer::new(opts);

}
