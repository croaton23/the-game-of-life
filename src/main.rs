mod drawing;
extern crate gtk;
use std::{ thread, time };
use gtk::prelude::*;
use gtk::{ Application, ApplicationWindow };

fn app_fn(app: &gtk::Application) {
    let win = ApplicationWindow::builder()
        .application(app)
        .default_width(800)
        .default_height(600)
        .title("The game of life")
        .build();

    let area = gtk::DrawingArea::new();
    let frame = gtk::Frame::new(None);
    area.connect_draw(move |drawing_area, context| {
        let window = drawing_area.window().unwrap();
        let width = window.width();
        let height = window.height();
        
        context.set_source_rgb(0.0, 0.0, 0.0);
        
        drawing::draw_board(&context, width, height);
        context.fill().unwrap();
        drawing::draw_grid(&context, width, height);

        context.stroke().unwrap();
        drawing_area.queue_draw();
        thread::sleep(time::Duration::from_millis(1000 / 60));
        //println!("123");
        gtk::Inhibit(false)
    });

    frame.add(&area);
    win.add(&frame);
    win.show_all();
}

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(app_fn);

    app.run();
}
