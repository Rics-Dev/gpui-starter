use gpui::*;
use gpui_component::{ActiveTheme as _, *};

pub struct HomeScreen;

impl HomeScreen {
    pub fn new() -> Self {
        Self
    }
}

impl Render for HomeScreen {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .p_6()
            .bg(cx.theme().background)
            .child(
                v_flex()
                    .flex_1()
                    .rounded(cx.theme().radius)
                    .border_1()
                    .border_color(cx.theme().border)
                    .bg(cx.theme().secondary)
                    .items_center()
                    .justify_center()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_medium()
                            .text_color(cx.theme().secondary_foreground)
                            .child("Main Content"),
                    )
                    .child(
                        div()
                            .text_color(cx.theme().muted_foreground)
                            .child("Replace this panel with your application UI."),
                    ),
            )
    }
}
