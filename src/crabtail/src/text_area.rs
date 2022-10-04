use crate::transform;

pub trait TextArea {
    fn label(&self) -> &'static str;
    fn placeholder(&self) -> &'static str;
    fn value(&self) -> &str;
    fn set_value(&mut self, value: String);
    fn set_transformed_value(&mut self, input: &str);
    fn new() -> Self where Self: Default {
        Self::default()
    }
}

#[derive(Default)]
pub struct CssTextArea {
    value: String,
}

impl TextArea for CssTextArea {
    fn label(&self) -> &'static str {
        "CSS"
    }
    fn placeholder(&self) -> &'static str {
        "py-2 text-white hover:bg-yellow-500"
    }
    fn value(&self) -> &str {
        &self.value
    }
    fn set_value(&mut self, value: String) {
        self.value = value
    }
    fn set_transformed_value(&mut self, input: &str) {
        self.set_value(transform::to_css(&input))
    }
}

#[derive(Default)]
pub struct TypedTextArea {
    value: String,
}

impl TextArea for TypedTextArea {
    fn label(&self) -> &'static str {
        "Typed"
    }
    fn placeholder(&self) -> &'static str {
        "C.py_2, C.text_white, C.hover__bg_yellow_500"
    }
    fn value(&self) -> &str {
        &self.value
    }
    fn set_value(&mut self, value: String) {
        self.value = value
    }
    fn set_transformed_value(&mut self, input: &str) {
        self.set_value(transform::to_typed(&input))
    }
}
