use stdweb::{
    traits::INonElementParentNode,
    unstable::TryInto,
    web::{
        CanvasRenderingContext2d,
        document,
        Element,
        html_element::CanvasElement,
    },
    private::ConversionError,
};

pub struct Canvas {
    elem: CanvasElement,
    ctx: CanvasRenderingContext2d,
}

// Private impl
impl Canvas {
    fn _set_fill_color(&self, color: Option<&str>) {
        if color.is_some() {
            self.ctx.set_fill_style_color(color.unwrap());
        }
    }
}

// Public impl
impl Canvas {
    pub fn new(canvas_id: &str, height: u32, width: u32) -> Result<Self, ()> {
        let elem: Option<Element> = document().get_element_by_id(canvas_id);
        if elem.is_some() {
            let canvas_elem: Result<CanvasElement, ConversionError> = elem.unwrap().try_into();

            if canvas_elem.is_ok() {
                let canvas = canvas_elem.unwrap();
                canvas.set_height(height);
                canvas.set_width(width);

                let context = canvas.get_context().unwrap();

                return Ok(Self {
                    elem: canvas,
                    ctx: context,
                });
            }
        }

        Err(())
    }

    pub fn get_elem(&self) -> CanvasElement {
        return self.elem.clone();
    }

    pub fn fill_rect(&self, x: u16, y: u16, height: u16, width: u16, color: Option<&str>) {
        if color.is_some() {
            self._set_fill_color(color);
        }

        self.ctx.fill_rect(x as f64, y as f64, height as f64, width as f64);
    }

    pub fn clear(&self) {
        self.ctx.clear_rect(0_f64, 0_f64, self.elem.height() as f64, self.elem.width() as f64);
    }
}
