use std::{fs};
use std::fs::ReadDir;
use gpui::*;

struct HelloWorld {
    text: SharedString,
}
struct PathChangeEvent {
    path: Option<String>,
}
impl EventEmitter<PathChangeEvent> for HelloWorld {}

struct MultipleChilds<T: ParentElement, U: IntoElement> {
    base: T,
    childs: Vec<U>,
}

impl<T, U> MultipleChilds<T, U>
where
    T: ParentElement + Element,
    U: IntoElement,
{
    pub fn new(base: T, childs: Vec<U>) -> MultipleChilds<T, U> {
        MultipleChilds { base, childs }
    }
}

impl<T, U> IntoElement for MultipleChilds<T, U>
where
    T: ParentElement + Element,
    U: IntoElement,
{
    type Element = T;

    fn into_element(self) -> Self::Element {
        let mut base = self.base;

        for element in self.childs {
            base = base.child(element);
        }

        base
    }
}

fn shower_text(text: &str) -> impl IntoElement + StatefulInteractiveElement {
    div()
        .id(SharedString::from(text.to_string()))
        .flex_none()
        .px_2()
        .bg(rgb(0xfff))
        .border_1()
        .rounded_md()
        .cursor_pointer()
        .child(text.to_string())
}


fn dir_name(text: &str, on_click: impl Fn(&mut WindowContext) + 'static) -> impl IntoElement {
    shower_text(text).on_click(move |_, cx| {
        cx.notify(Some(EntityId::default()));
        on_click(cx)
    })
}

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(black())
            .justify_center()
            .items_center()
            .shadow_lg()
            .text_color(rgb(0xffffff))
            .child({
                let dirs = fs::read_dir(dirs::home_dir().unwrap())
                    .map(|dir| {
                        dir.flatten()
                            .flat_map(|entry| entry.file_name().to_str().map(ToString::to_string))
                            .map(|name|
                                dir_name(&name.clone(), move |x| {})
                            )
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();

                MultipleChilds::new(div(), dirs)
            })
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(240.0), px(100.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| {
                let path: Model<ReadDir> = cx.new_model(|_cx| {
                    fs::read_dir(".").unwrap()
                });

                let a = cx.new_view(|_cx| HelloWorld {
                    text: "World".into(),
                });

                a.update(cx, |c, cx| {
                    println!("{:?}", 1)
                });

                a
            },
        )
            .unwrap();
    });
}