use gpui::*;
use gpui_component::{ActiveTheme as _, *};

use crate::screens::HomeScreen;
use crate::ui::AppTitleBar;

pub struct App {
    title_bar: Entity<AppTitleBar>,
    home_screen: Entity<HomeScreen>,
}

impl App {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        window.set_window_title(crate::APP_TITLE);

        let app_menu_bar = crate::app_menus::init(crate::APP_TITLE, cx);
        let title_bar = cx.new(|_| AppTitleBar::new(app_menu_bar.clone()));
        let home_screen = cx.new(|_| HomeScreen::new());

        Self {
            title_bar,
            home_screen,
        }
    }
}

impl Render for App {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(self.title_bar.clone())
            .child(self.home_screen.clone())
    }
}
