use gpui::*;
use gpui_component::menu::AppMenuBar;
use gpui_component::*;

pub struct AppTitleBar {
    app_menu_bar: Entity<AppMenuBar>,
}

impl AppTitleBar {
    pub fn new(app_menu_bar: Entity<AppMenuBar>) -> Self {
        Self { app_menu_bar }
    }
}

impl Render for AppTitleBar {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        TitleBar::new().child(
            h_flex()
                .w_full()
                .pr_2()
                .justify_between()
                .child(div().flex().items_center().child(self.app_menu_bar.clone()))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_end()
                        .px_2()
                        .gap_2()
                        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                        .child(crate::APP_TITLE),
                ),
        )
    }
}
