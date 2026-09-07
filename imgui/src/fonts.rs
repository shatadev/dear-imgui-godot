use std::cell::{Cell, RefCell};
use std::collections::HashMap;

use godot::classes::image::Format;
use godot::classes::{Image, ImageTexture, Texture2D};
use godot::prelude::*;

use imgui::{sys, Context, FontConfig, FontSource, TextureId};

pub struct TextureRegistry {
    map: HashMap<usize, Gd<Texture2D>>,
    next: usize,
}

impl TextureRegistry {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            next: 1,
        }
    }

    pub fn register(&mut self, tex: Gd<Texture2D>) -> usize {
        let id = self.next;
        self.next += 1;
        self.map.insert(id, tex);
        id
    }

    pub fn lookup(&self, id: usize) -> Option<Rid> {
        self.map.get(&id).map(|t| t.get_rid())
    }

    pub fn remove(&mut self, id: usize) {
        self.map.remove(&id);
    }
}

/// A custom TTF/OTF font queued by `add_font_from_file`, baked into the atlas on
/// every rebuild. `ptr` is the live `ImFont` for the current atlas, refreshed each
/// time it is rebuilt, and used by `push_font`.
struct FontEntry {
    handle: i64,
    data: Vec<u8>,
    size_pixels: f32,
    ptr: *mut sys::ImFont,
}

thread_local! {
    static ENTRIES: RefCell<Vec<FontEntry>> = const { RefCell::new(Vec::new()) };
    static NEXT_HANDLE: Cell<i64> = const { Cell::new(1) };
    static DIRTY: Cell<bool> = const { Cell::new(false) };
}

/// Queue a TTF/OTF font (raw bytes and logical pixel size) for the atlas, returning
/// a stable handle to pass to `push_font`. The atlas is rebuilt on the next frame.
pub(crate) fn queue_font(data: Vec<u8>, size_pixels: f32) -> i64 {
    let handle = NEXT_HANDLE.with(|h| {
        let v = h.get();
        h.set(v + 1);
        v
    });
    ENTRIES.with(|e| {
        e.borrow_mut().push(FontEntry {
            handle,
            data,
            size_pixels,
            ptr: std::ptr::null_mut(),
        })
    });
    DIRTY.with(|d| d.set(true));
    handle
}

/// True (clearing the flag) when fonts were queued since the last check and the atlas
/// must be rebuilt before the next frame.
pub(crate) fn take_dirty() -> bool {
    DIRTY.with(|d| d.replace(false))
}

/// The baked `ImFont` for a handle, or null when unknown. Dear ImGui treats a null
/// font as the default font, so an invalid handle harmlessly falls back to it.
pub(crate) fn font_ptr(handle: i64) -> *mut sys::ImFont {
    ENTRIES.with(|e| {
        e.borrow()
            .iter()
            .find(|f| f.handle == handle)
            .map(|f| f.ptr)
            .unwrap_or(std::ptr::null_mut())
    })
}

/// Build the font atlas at the given UI scale and register its texture, returning
/// the new texture id. The default font is baked at `13 * scale` pixels so text
/// stays crisp at any scale. When rebuilding at runtime, pass the previous id as
/// `old_id` so its texture can be released.
pub fn build_font_atlas(
    ctx: &mut Context,
    textures: &mut TextureRegistry,
    scale: f32,
    old_id: usize,
) -> usize {
    let atlas = ctx.fonts();
    atlas.clear();
    atlas.add_font(&[FontSource::DefaultFontData {
        config: Some(FontConfig {
            size_pixels: 13.0 * scale,
            oversample_h: 1,
            oversample_v: 1,
            pixel_snap_h: true,
            ..Default::default()
        }),
    }]);

    // Re-bake every custom font at the current scale and record its live `ImFont`
    // pointer so `push_font` can select it this atlas.
    ENTRIES.with(|e| {
        for entry in e.borrow_mut().iter_mut() {
            let id = atlas.add_font(&[FontSource::TtfData {
                data: &entry.data,
                size_pixels: entry.size_pixels * scale,
                config: None,
            }]);
            entry.ptr = atlas
                .get_font(id)
                .map(|f| f as *const _ as *mut sys::ImFont)
                .unwrap_or(std::ptr::null_mut());
        }
    });

    let (width, height, data) = {
        let tex = atlas.build_rgba32_texture();
        (
            tex.width as i32,
            tex.height as i32,
            PackedByteArray::from(tex.data),
        )
    };

    let image = Image::create_from_data(width, height, false, Format::RGBA8, &data)
        .expect("imgui font atlas image");
    let texture = ImageTexture::create_from_image(&image).expect("imgui font atlas texture");

    let id = textures.register(texture.upcast::<Texture2D>());
    atlas.tex_id = TextureId::from(id);

    if old_id != 0 {
        textures.remove(old_id);
    }
    id
}
