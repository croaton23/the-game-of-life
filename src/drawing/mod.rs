use gtk::cairo::{ Context };

const BORDER_SIZE: i32 = 10;
const GRID_SIZE: i32 = 40;

pub fn draw_board(context: &Context, width:i32, height:i32)
{
    context.rectangle(
        f64::from(BORDER_SIZE), 
        f64::from(BORDER_SIZE), 
        f64::from(width - 2 * BORDER_SIZE),
        f64::from(height - 2 * BORDER_SIZE));
}

pub fn draw_grid(context: &Context, width:i32, height:i32)
{
    let cell_width: i32 = (width - 2 * BORDER_SIZE) / GRID_SIZE;
    let cell_height: i32 = (height - 2 * BORDER_SIZE) / GRID_SIZE;
    context.set_source_rgb(255.0, 255.0, 255.0);

    for i in 1..(GRID_SIZE + 1) {
        context.move_to(f64::from(BORDER_SIZE + i * cell_width), f64::from(BORDER_SIZE));
        context.line_to(f64::from(BORDER_SIZE + i * cell_width), f64::from(height - BORDER_SIZE));
    }

    for i in 1..(GRID_SIZE + 1) {
        context.move_to(f64::from(BORDER_SIZE), f64::from(BORDER_SIZE + i * cell_height));
        context.line_to(f64::from(width - BORDER_SIZE), f64::from(BORDER_SIZE + i * cell_height));
    }
}