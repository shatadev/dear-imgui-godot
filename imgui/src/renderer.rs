use godot::classes::RenderingServer;
use godot::prelude::*;

use imgui::{DrawCmd, DrawCmdParams, DrawData};

use crate::fonts::TextureRegistry;

pub struct CanvasRenderer {
    canvas: Rid,
    root_item: Rid,
    pool: Vec<Rid>,
}

impl CanvasRenderer {
    pub fn new() -> Self {
        Self {
            canvas: Rid::Invalid,
            root_item: Rid::Invalid,
            pool: Vec::new(),
        }
    }

    pub fn init(&mut self, viewport: Rid) {
        let mut rs = RenderingServer::singleton();
        self.canvas = rs.canvas_create();
        self.root_item = rs.canvas_item_create();
        rs.viewport_attach_canvas(viewport, self.canvas);
        rs.viewport_set_canvas_stacking(viewport, self.canvas, 100, 0);
        rs.canvas_item_set_parent(self.root_item, self.canvas);
    }

    fn item_at(&mut self, idx: usize, rs: &mut Gd<RenderingServer>) -> Rid {
        while self.pool.len() <= idx {
            let item = rs.canvas_item_create();
            rs.canvas_item_set_parent(item, self.root_item);
            self.pool.push(item);
        }
        self.pool[idx]
    }

    pub fn render(&mut self, draw_data: &DrawData, textures: &TextureRegistry) {
        let mut rs = RenderingServer::singleton();

        if draw_data.draw_lists_count() == 0 {
            for &item in &self.pool {
                rs.canvas_item_clear(item);
                rs.canvas_item_set_visible(item, false);
            }
            return;
        }

        let mut cmd_index = 0usize;

        for draw_list in draw_data.draw_lists() {
            let vtx = draw_list.vtx_buffer();
            let idx = draw_list.idx_buffer();

            let mut pv: Vec<Vector2> = Vec::with_capacity(vtx.len());
            let mut uv: Vec<Vector2> = Vec::with_capacity(vtx.len());
            let mut cv: Vec<Color> = Vec::with_capacity(vtx.len());
            for v in vtx {
                pv.push(Vector2::new(v.pos[0], v.pos[1]));
                uv.push(Vector2::new(v.uv[0], v.uv[1]));
                cv.push(Color::from_rgba(
                    v.col[0] as f32 / 255.0,
                    v.col[1] as f32 / 255.0,
                    v.col[2] as f32 / 255.0,
                    v.col[3] as f32 / 255.0,
                ));
            }
            let points = PackedVector2Array::from(pv.as_slice());
            let uvs = PackedVector2Array::from(uv.as_slice());
            let colors = PackedColorArray::from(cv.as_slice());

            for cmd in draw_list.commands() {
                let DrawCmd::Elements { count, cmd_params } = cmd else {
                    continue;
                };
                let DrawCmdParams {
                    clip_rect,
                    texture_id,
                    vtx_offset,
                    idx_offset,
                } = cmd_params;

                let item = self.item_at(cmd_index, &mut rs);
                rs.canvas_item_clear(item);
                rs.canvas_item_set_visible(item, true);
                rs.canvas_item_set_clip(item, true);
                rs.canvas_item_set_custom_rect_ex(item, true)
                    .rect(Rect2::new(
                        Vector2::new(clip_rect[0], clip_rect[1]),
                        Vector2::new(clip_rect[2] - clip_rect[0], clip_rect[3] - clip_rect[1]),
                    ))
                    .done();

                let mut iv: Vec<i32> = Vec::with_capacity(count);
                for k in 0..count {
                    iv.push(idx[idx_offset + k] as i32 + vtx_offset as i32);
                }
                let indices = PackedInt32Array::from(iv.as_slice());

                let tex = textures.lookup(texture_id.id()).unwrap_or(Rid::Invalid);

                rs.canvas_item_add_triangle_array_ex(item, &indices, &points, &colors)
                    .uvs(&uvs)
                    .texture(tex)
                    .done();

                cmd_index += 1;
            }
        }

        for i in cmd_index..self.pool.len() {
            rs.canvas_item_clear(self.pool[i]);
            rs.canvas_item_set_visible(self.pool[i], false);
        }
    }
}

impl Drop for CanvasRenderer {
    fn drop(&mut self) {
        let mut rs = RenderingServer::singleton();
        for item in self.pool.drain(..) {
            rs.free_rid(item);
        }
        if self.root_item != Rid::Invalid {
            rs.free_rid(self.root_item);
        }
        if self.canvas != Rid::Invalid {
            rs.free_rid(self.canvas);
        }
    }
}
