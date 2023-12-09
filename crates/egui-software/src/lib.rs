use epaint::Rect;

macro_rules! color_to_u32 {
    ($r:expr, $g:expr, $b:expr) => {
        b | (g << 8) | (r << 16)
    };
}

struct RenderBuffer<'a> {
    buf: &'a mut [u32],
    clip_rect: Rect,
}

impl<'a> RenderBuffer<'a> {
    fn new(buf: &'a mut [u32]) -> Self {
        Self {
            buf,
            clip_rect: Rect::EVERYTHING,
        }
    }

    fn draw_rect(&mut self) {
        
    }
}

pub fn render(paint_jobs: &[epaint::ClippedPrimitive], buf: &mut [u32]) {
    // create a new render buffer for this paint job, it doesn't need to
    // share state between frames
    let mut rb = RenderBuffer::new(buf);

    for prim in paint_jobs {
        render_primitive(prim, &mut rb);
    }
}

fn render_primitive(prim: &epaint::ClippedPrimitive, rb: &mut RenderBuffer<'_>) {
    // update the renderbuffers cliprect
    rb.clip_rect = prim.clip_rect;
}
