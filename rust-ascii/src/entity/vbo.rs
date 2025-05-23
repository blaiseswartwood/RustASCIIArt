#[derive(Clone, Copy)]
pub struct Vertex {
    pub cmdline: i32,
    pub start_pos: [f32; 2],
    pub end_pos: [f32; 2],
    pub start_time: i32,
    pub end_time: i32,
    pub font_size: f32,
    pub tid: i32,
    pub corner: i32,
    pub color: [f32; 3], // CONVERT THIS TO VEC3
    pub new_color: [f32; 3], // CONVERT THIS TO VEC3
    pub random_seed: f32,
    pub fade_time: f32,
    pub color_delay_time: i32, 
    pub char_delay_time: i32,
    pub accel_degree: f32,
}

glium::implement_vertex!(Vertex, cmdline, start_pos, end_pos, start_time, end_time, font_size, tid, corner, color, new_color, fade_time, random_seed, color_delay_time, char_delay_time, accel_degree);

pub struct Vbo {
    pub vertex_buffer: Vec<Vertex>,
    pub index_buffer: Vec<u32>
}

impl Default for Vbo {
    fn default() -> Self {
        Self::new()
    }
}

impl Vbo {
    pub fn new() -> Self {
        Vbo {
            vertex_buffer: Vec::new(),
            index_buffer: Vec::new()
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_vbe() {
        let vbo = Vbo::default();
        assert!(vbo.vertex_buffer.is_empty());
    }

    #[test]
    fn test_vbo() {
        let _vbo = Vbo::new();
    }
}