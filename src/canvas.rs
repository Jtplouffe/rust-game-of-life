use std::fmt::Error;

use stdweb::{
    traits::INonElementParentNode,
    unstable::TryInto,
    web::{
        document,
        Element,
        html_element::CanvasElement,
    },
};
use stdweb::private::ConversionError;

pub struct Canvas {
    elem: CanvasElement,
}

impl Canvas {
    pub fn new(canvas_id: &str) -> Result<Self, ()> {
        let elem: Option<Element> = document().get_element_by_id(canvas_id);
        if elem.is_some() {
            let canvas_elem: Result<CanvasElement, ConversionError> = elem.unwrap().try_into();

            if canvas_elem.is_ok() {
                return Ok(Self {
                    elem: canvas_elem.unwrap(),
                });
            }
        }

        Err(())
    }
}