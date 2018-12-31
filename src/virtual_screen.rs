use sfml::graphics::{Color, Drawable, PrimitiveType, RenderStates, RenderTarget, VertexArray};
use sfml::system::{Vector2f, Vector2u};

pub struct VirtualScreen {
    screen_size: Vector2u,
    pixel_size: f32,
    vertices: VertexArray,
}

impl VirtualScreen {
    pub fn new(w: u32, h: u32, pixel_size: f32, color: Color) -> Self {
        let mut vertices = VertexArray::new(PrimitiveType::Triangles, (w * h * 6) as usize);
        let screen_size = Vector2u::new(w, h);
        for x in 0..w {
            for y in 0..h {
                let index = ((x * screen_size.y + y) * 6) as usize;
                let coord2d = Vector2f::new((x as f32) * pixel_size, (y as f32) * pixel_size);

                vertices[index].position = coord2d;
                vertices[index].color = color;

                // top-right
                vertices[index + 1].position = coord2d + Vector2f::new(pixel_size, 0.0);
                vertices[index + 1].color = color;

                // bottom-right
                vertices[index + 2].position = coord2d + Vector2f::new(pixel_size, pixel_size);
                vertices[index + 2].color = color;

                // Triangle-2
                // bottom-right
                vertices[index + 3].position = coord2d + Vector2f::new(pixel_size, pixel_size);
                vertices[index + 3].color = color;

                // bottom-left
                vertices[index + 4].position = coord2d + Vector2f::new(0.0, pixel_size);
                vertices[index + 4].color = color;

                // top-left
                vertices[index + 5].position = coord2d;
                vertices[index + 5].color = color;
            }
        }
        VirtualScreen {
            vertices: VertexArray::new(PrimitiveType::Triangles, (w * h * 6) as usize),
            screen_size: Vector2u::new(w, h),
            pixel_size: pixel_size,
        }
    }
}

impl Drawable for VirtualScreen {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut RenderTarget,
        states: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_vertex_array(&self.vertices, states);
    }
}
