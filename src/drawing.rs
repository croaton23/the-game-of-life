use gtk::cairo::{ Context };

use crate::gameplay::Field;

const BORDER_SIZE: i32 = 10;

pub fn draw_board(context: &Context, width:i32, height:i32)
{
    context.set_source_rgb(0.0, 0.0, 0.0);

    context.rectangle(
        f64::from(BORDER_SIZE), 
        f64::from(BORDER_SIZE), 
        f64::from(width - 2 * BORDER_SIZE),
        f64::from(height - 2 * BORDER_SIZE));
    
    context.fill().unwrap();
}

pub fn draw_grid(field: &Field, context: &Context, width:i32, height:i32)
{
    let grid_size = field.size as f64;

    let cell_width: f64 = f64::from(width - 2 * BORDER_SIZE) / grid_size;
    let cell_height: f64 = f64::from(height - 2 * BORDER_SIZE) / grid_size;
    context.set_source_rgb(255.0, 255.0, 255.0);

    for i in 0..field.size + 1 {
        context.move_to(BORDER_SIZE as f64 + i as f64 * cell_width, f64::from(BORDER_SIZE));
        context.line_to(BORDER_SIZE as f64 + i as f64 * cell_width, f64::from(height - BORDER_SIZE));
        context.move_to(f64::from(BORDER_SIZE), BORDER_SIZE as f64 + i as f64 * cell_height);
        context.line_to(f64::from(width - BORDER_SIZE), BORDER_SIZE as f64 + i as f64 * cell_height);
    }
    context.stroke().unwrap();

    context.set_source_rgb(0.0, 0.0, 255.0);

    for i in 0..field.size {
        for j in 0..field.size {
            if field.field[i][j] == 1 {
            context.rectangle(
                BORDER_SIZE as f64 + (i as f64 + 0.1) * cell_width, 
                BORDER_SIZE as f64 + (j as f64 + 0.1) * cell_height, 
                cell_width * 0.8, cell_height * 0.8);
            }
        }
    }
    context.fill().unwrap();
}