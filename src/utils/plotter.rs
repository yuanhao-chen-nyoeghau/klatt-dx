use base64::prelude::*;
use plotters::{coord::Shift, prelude::*};
use plotters_svg::SVGBackend;

use crate::*;

pub type DioxusDrawingArea<'a> = DrawingArea<SVGBackend<'a>, Shift>;

#[derive(Clone, Props)]
pub struct PlottersProps<F: Fn(DioxusDrawingArea) + Clone + 'static> {
    pub class: Option<String>,
    pub style: Option<String>,
    pub id: Option<String>,

    pub size: (u32, u32),
    pub draw: F,
    pub on_click: Option<EventHandler<Event<MouseData>>>,
    pub on_dblclick: Option<EventHandler<Event<MouseData>>>,
    pub on_mousemove: Option<EventHandler<Event<MouseData>>>,
    pub on_mouseout: Option<EventHandler<Event<MouseData>>>,
    pub on_mouseup: Option<EventHandler<Event<MouseData>>>,
    pub on_mousedown: Option<EventHandler<Event<MouseData>>>,
    pub on_mouseover: Option<EventHandler<Event<MouseData>>>,
    pub on_wheel: Option<EventHandler<Event<WheelData>>>,
    pub draggable: Option<bool>,
    pub on_drag: Option<EventHandler<Event<DragData>>>,
    pub on_dragend: Option<EventHandler<Event<DragData>>>,
    pub on_dragenter: Option<EventHandler<Event<DragData>>>,
    pub on_dragleave: Option<EventHandler<Event<DragData>>>,
    pub on_dragover: Option<EventHandler<Event<DragData>>>,
    pub on_dragstart: Option<EventHandler<Event<DragData>>>,
    pub on_drop: Option<EventHandler<Event<DragData>>>,
    pub on_scroll: Option<EventHandler<Event<ScrollData>>>,
}

impl<F: Fn(DioxusDrawingArea) + Clone + 'static> PartialEq for PlottersProps<F> {
    fn eq(&self, other: &PlottersProps<F>) -> bool {
        self.size == other.size
    }
}

#[allow(non_snake_case)]
pub fn Plotters<F: Fn(DioxusDrawingArea) + Clone + 'static>(props: PlottersProps<F>) -> Element {
    let mut buf = use_signal(|| String::new());
    use_effect(move || {
        let mut buf = buf.write();
        buf.clear();
        let drawing_area = SVGBackend::with_string(&mut buf, props.size).into_drawing_area();
        (props.draw)(drawing_area);
    });
    let buf_base64 = use_memo(move || {
        let buf = buf.read();
        BASE64_STANDARD.encode(buf.as_bytes())
    });

    rsx!(img {
        class: props.class.unwrap_or_default(),
        style: props.style.unwrap_or_default(),
        id: props.id.unwrap_or_default(),

        draggable: props.draggable.unwrap_or(false),
        onclick: move |e| {
            if let Some(callback) = props.on_click.as_ref() {
                callback.call(e)
            }
        },
        ondoubleclick: move |e| {
            if let Some(callback) = props.on_dblclick.as_ref() {
                callback.call(e)
            }
        },
        onmousemove: move |e| {
            if let Some(callback) = props.on_mousemove.as_ref() {
                callback.call(e)
            }
        },
        onmousedown: move |e| {
            if let Some(callback) = props.on_mousedown.as_ref() {
                callback.call(e)
            }
        },
        onmouseup: move |e| {
            if let Some(callback) = props.on_mouseup.as_ref() {
                callback.call(e)
            }
        },
        onmouseout: move |e| {
            if let Some(callback) = props.on_mouseout.as_ref() {
                callback.call(e)
            }
        },
        onmouseover: move |e| {
            if let Some(callback) = props.on_mouseout.as_ref() {
                callback.call(e)
            }
        },
        onwheel: move |e| {
            if let Some(callback) = props.on_wheel.as_ref() {
                callback.call(e)
            }
        },
        ondrag: move |e| {
            if let Some(callback) = props.on_drag.as_ref() {
                callback.call(e)
            }
        },
        ondragend: move |e| {
            if let Some(callback) = props.on_dragend.as_ref() {
                callback.call(e)
            }
        },
        ondragenter: move |e| {
            if let Some(callback) = props.on_dragenter.as_ref() {
                callback.call(e)
            }
        },
        ondragleave: move |e| {
            if let Some(callback) = props.on_dragleave.as_ref() {
                callback.call(e)
            }
        },
        ondragover: move |e| {
            if let Some(callback) = props.on_dragover.as_ref() {
                callback.call(e)
            }
        },
        ondragstart: move |e| {
            if let Some(callback) = props.on_dragstart.as_ref() {
                callback.call(e)
            }
        },
        ondrop: move |e| {
            if let Some(callback) = props.on_drop.as_ref() {
                callback.call(e)
            }
        },
        onscroll: move |e| {
            if let Some(callback) = props.on_scroll.as_ref() {
                callback.call(e)
            }
        },
        src: "data:image/svg+xml;base64,{buf_base64}",
    })
}
