use std::collections::HashMap;

use godot::classes::image::Format;
use godot::classes::{Image, ImageTexture, Texture2D};
use godot::prelude::*;

use imgui::{Context, TextureId};

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
}

pub fn build_font_atlas(ctx: &mut Context, textures: &mut TextureRegistry) {
    let atlas = ctx.fonts();
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
}
