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
            .gap_6()
            .bg(cx.theme().background)
            .child(
                v_flex()
                    .gap_2()
                    .child(
                        div()
                            .text_3xl()
                            .font_semibold()
                            .text_color(cx.theme().foreground)
                            .child("Home"),
                    )
                    .child(
                        div()
                            .text_color(cx.theme().muted_foreground)
                            .child("Start building your GPUI app from this screen."),
                    ),
            )
            .child(
                h_flex()
                    .gap_3()
                    .child(metric("Status", "Ready", cx))
                    .child(metric("Views", "1", cx))
                    .child(metric("Actions", "0", cx)),
            )
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

fn metric(label: &'static str, value: &'static str, cx: &mut App) -> impl IntoElement {
    v_flex()
        .min_w(px(160.0))
        .gap_1()
        .rounded(cx.theme().radius)
        .border_1()
        .border_color(cx.theme().border)
        .bg(cx.theme().popover)
        .p_4()
        .child(
            div()
                .text_sm()
                .text_color(cx.theme().muted_foreground)
                .child(label),
        )
        .child(
            div()
                .text_lg()
                .font_semibold()
                .text_color(cx.theme().popover_foreground)
                .child(value),
        )
}
