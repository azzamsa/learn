#![
    allow(clippy::wildcard_imports) // importing all seed element make life harder
]

mod generated;
mod icon;
mod text_area;
mod transform;

use seed::{prelude::*, *};
use std::mem;
use text_area::{TextArea, CssTextArea, TypedTextArea};

use crate::generated::css_classes::C;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        text_area_input: Box::new(CssTextArea::new()),
        text_area_output: Box::new(TypedTextArea::new()),
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    text_area_input: Box<dyn TextArea>,
    text_area_output: Box<dyn TextArea>,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    TextAreaInputChanged(String),
    TextAreaOutputChanged(String),
    Swap,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::TextAreaInputChanged(value) => {
            model.text_area_output.set_transformed_value(&value);
            model.text_area_input.set_value(value)
        },
        Msg::TextAreaOutputChanged(value) => {
            model.text_area_output.set_value(value)
        },
        Msg::Swap => {
            mem::swap(&mut model.text_area_input, &mut model.text_area_output);
            let input = model.text_area_input.value();
            model.text_area_output.set_transformed_value(input);
        },
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    section![
        C![C.bg_main, C.pt_12, C.md__pt_20, C.pb_6, C.px_2, C.md__px_5, C.min_h_screen],
        header![
            C![C.max_w_lg, C.mx_auto],
            h1![C![C.font_bold, C.text_white C.text_center], "CrabTail"]
        ],
        main![
            C![
                C.bg_white,
                C.max_w_5xl,
                C.mx_auto,
                C.p_8,
                C.md__p_12,
                C.my_10,
                C.rounded_lg,
                C.shadow_2xl
            ],
            view_title(),
            view_control_panel(&model.text_area_input, &model.text_area_output),
        ],
        view_footer()
    ]
}

fn view_title() -> Node<Msg> {
    section![p![
        C![C.text_lg, C.text_center, C.text_gray_600, C.pt_0],
        "Convert your TailwindCSS",
        i![
            C![C.inline_block, C.mx_1, C.align_middle C.w_4, C.h_4],
            raw_svg!(icon::get(&icon::Name::Wind))
        ],
        "to typed Rust",
        i![
            C![C.inline_block, C.mx_1, C.align_middle C.w_4, C.h_4],
            raw_svg!(icon::get(&icon::Name::Crab))
        ],
    ]]
}

fn view_control_panel(text_area_input: &Box<dyn TextArea>, text_area_output: &Box<dyn TextArea>) -> Node<Msg> {
    section![
        C![C.flex, C.flex_col, C.mt_10],
        view_text_area(text_area_input, "input", Msg::TextAreaInputChanged),
        div![
            C![C.flex, C.justify_center],
            button![
                id!["swap-btn"],
                C![C.btn, C.mb_6, C.px_3, C.py_1, C.stroke_2],
                raw_svg!(icon::get(&icon::Name::SwitchVertical)),
                ev(Ev::Click, |_| Msg::Swap),
            ]
        ],
        view_text_area(text_area_output, "output", Msg::TextAreaOutputChanged),
    ]
}

fn view_text_area(
    text_area: &Box<dyn TextArea>, 
    id: &'static str, 
    on_change: fn(String) -> Msg,
) -> Node<Msg> {
    div![
        id![id],
        C![C.mb_6, C.pt_3, C.rounded, C.bg_gray_200],
        label![
            C![C.input_label],
            text_area.label(),
        ],
        textarea![
            attrs! {
                At::Type => "text",
                At::Placeholder => text_area.placeholder(),
                At::Value => text_area.value()
            },
            C![C.input],
            input_ev(Ev::Input, on_change),
        ]
    ]
}

fn view_footer() -> Node<Msg> {
    footer![
        C![
            C.max_w_lg,
            C.mx_auto,
            C.flex,
            C.justify_center,
            C.text_white,
            C.font_medium
        ],
        a![
            C![
                C.duration_500,
                C.transform,
                C.hover___translate_y_1,
                C.hover__scale_125,
                C.hover__underline,
            ],
            attrs! {At::Href => "#"},
            "Support Me",
            i![
                C![C.inline_block, C.mx_1, C.align_middle C.w_4, C.h_4],
                raw_svg!(icon::get(&icon::Name::SparklingHeart))
            ],
        ],
        span![C![C.mx_3], "\u{2022}"],
        a![C![C.hover__underline], attrs! {At::Href => "#"}, "Meta"],
    ]
}

// ------ ------
//     Start
// ------ ------

pub fn main() {
    App::start("app", init, update, view);
}
