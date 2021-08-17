mod drawing;
mod gameplay;

extern crate gtk;
use std::{ thread, time };
use gameplay::Game;
use gtk::prelude::*;
use gtk::{ Application, ApplicationWindow };
use std::sync::{Arc, Mutex};

const GRID_SIZE: usize = 5;

fn app_fn(app: &gtk::Application) {
    let win = ApplicationWindow::builder()
        .application(app)
        .default_width(800)
        .default_height(600)
        .title("The game of life")
        .build();

    let area = gtk::DrawingArea::new();
    let frame = gtk::Frame::new(None);
    let mut field = Arc::new(Mutex::new(gameplay::Field::new(GRID_SIZE)));
    field.lock().unwrap().init();

    area.connect_draw(move |drawing_area, context| {
        const dt:f64 = 1000.0;
        let window = drawing_area.window().unwrap();
        let width = window.width();
        let height = window.height();
        
        drawing::draw_board(&context, width, height);
        drawing::draw_grid(&field.lock().unwrap(), &context, width, height);
        field.lock().unwrap().tick(dt);
        
        drawing_area.queue_draw();
        thread::sleep(time::Duration::from_millis(dt.round() as u64));
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
