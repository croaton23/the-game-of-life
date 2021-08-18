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
    let grid_size = field.size as i32;

    let cell_width: i32 = (width - 2 * BORDER_SIZE) / grid_size;
    let cell_height: i32 = (height - 2 * BORDER_SIZE) / grid_size;
    context.set_source_rgb(255.0, 255.0, 255.0);

    for i in 0..grid_size {
        context.move_to(f64::from(BORDER_SIZE + i * cell_width), f64::from(BORDER_SIZE));
        context.line_to(f64::from(BORDER_SIZE + i * cell_width), f64::from(height - BORDER_SIZE));
    }

    for i in 0..grid_size {
        context.move_to(f64::from(BORDER_SIZE), f64::from(BORDER_SIZE + i * cell_height));
        context.line_to(f64::from(width - BORDER_SIZE), f64::from(BORDER_SIZE + i * cell_height));
    }
    context.stroke().unwrap();

    context.set_source_rgb(0.0, 0.0, 255.0);

    for i in 0..field.size {
        for j in 0..field.size {
            // println!("{}", field.field[i][j]);
            if field.field[i][j] == 1 {
            context.rectangle(
                f64::from(BORDER_SIZE + (i as i32) * cell_width), 
                f64::from(BORDER_SIZE + (j as i32) * cell_height), 
                f64::from(cell_width), f64::from(cell_height));
            }
        }
        // println!();
    }
    context.fill().unwrap();
}