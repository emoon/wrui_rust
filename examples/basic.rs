#[macro_use]
extern crate wrui;

use wrui::{Wrui, Window};

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
