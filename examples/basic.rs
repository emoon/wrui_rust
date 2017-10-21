#[macro_use]
extern crate wrui;

use wrui::{Wrui, Window, Pos, Color};

struct TestApp<'a> {
    wrui: &'a Wrui,
    window: Window,
}

impl<'a> TestApp<'a> {
    pub fn new(wrui: &Wrui) -> TestApp {
        TestApp {
            wrui: wrui,
            window: wrui.create_window(),
        }
    }

    pub fn setup(&mut self) {
        window_set_paint_event!(self.window, self, TestApp, TestApp::paint_event);
    }

    pub fn paint_event(&mut self) {
        let painter = self.wrui.get_painter();

        painter.draw_text(Pos::new(10.0, 10.0), Color::new(1.0, 1.0, 0.0, 1.0), "test text!");

        println!("Paint event");

    }

    pub fn run(&self) {
        self.wrui.run();
    }
}


fn main() {
    // very temporary for now while testing
    let wrui = Wrui::new("../wrui/t2-output/macosx-clang-debug-default/libwrui_dimgui.dylib").unwrap();

    let mut app = TestApp::new(&wrui);

    app.setup();

    app.run();
}
