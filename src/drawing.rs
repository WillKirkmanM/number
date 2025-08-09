use piston_window::*;

pub const GRID_SIZE: usize = 28;
pub const PIXEL_SIZE: u32 = 20;
pub const WINDOW_SIZE: u32 = (GRID_SIZE as u32) * PIXEL_SIZE;

// Represents the state of our drawing application
pub struct App {
    pub window: PistonWindow,
    pub grid: [[u8; GRID_SIZE]; GRID_SIZE],
    is_drawing: bool,
}

fn draw_grid_on_window(grid: &[[u8; GRID_SIZE]; GRID_SIZE], c: Context, g: &mut G2d) {
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let color = if grid[y][x] > 0 {
                [1.0, 1.0, 1.0, 1.0] // White
            } else {
                [0.0, 0.0, 0.0, 1.0] // Black
            };

            rectangle(
                color,
                [
                    (x as f64) * PIXEL_SIZE as f64,
                    (y as f64) * PIXEL_SIZE as f64,
                    PIXEL_SIZE as f64,
                    PIXEL_SIZE as f64,
                ],
                c.transform,
                g,
            );
        }
    }
}

impl App {
    // Creates a new App instance
    pub fn new() -> Self {
        let window: PistonWindow = WindowSettings::new("Number Recognizer", [WINDOW_SIZE, WINDOW_SIZE])
            .exit_on_esc(true)
            .build()
            .unwrap();

        App {
            window,
            grid: [[0; GRID_SIZE]; GRID_SIZE],
            is_drawing: false,
        }
    }

    pub fn run(&mut self, recognition_fn: &dyn Fn(&[[u8; GRID_SIZE]; GRID_SIZE])) {
        while let Some(e) = self.window.next() {
            self.window.draw_2d(&e, |c, g, _| {
                clear([0.1, 0.1, 0.1, 1.0], g); // Dark Grey background
                draw_grid_on_window(&self.grid, c, g);
            });

            // Handle mouse press events
            if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
                self.is_drawing = true;
            }
            if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
                self.is_drawing = false;
            }

            // Handle mouse movement to draw on the grid
            if self.is_drawing {
                if let Some(pos) = e.mouse_cursor_args() {
                    self.draw_on_grid(pos[0], pos[1]);
                }
            }

            // Handle key presses
            if let Some(Button::Keyboard(key)) = e.press_args() {
                match key {
                    Key::C => self.clear_grid(),
                    Key::Return => {
                        println!("--- Running Recognition ---");
                        recognition_fn(&self.grid);
                    }
                    _ => {}
                }
            }
        }
    }

    // Draws a pixel on the grid based on mouse coordinates
    fn draw_on_grid(&mut self, x: f64, y: f64) {
        let grid_x = (x / PIXEL_SIZE as f64) as usize;
        let grid_y = (y / PIXEL_SIZE as f64) as usize;
        
        // Paint a slightly larger area for smoother drawing
        for y_offset in -1..=1 {
            for x_offset in -1..=1 {
                 let paint_x = grid_x as i32 + x_offset;
                 let paint_y = grid_y as i32 + y_offset;
                 if paint_x >= 0 && paint_x < GRID_SIZE as i32 && paint_y >= 0 && paint_y < GRID_SIZE as i32 {
                     self.grid[paint_y as usize][paint_x as usize] = 255; // Set pixel to white
                 }
            }
        }
    }

    // Resets the grid to all black pixels
    fn clear_grid(&mut self) {
        self.grid = [[0; GRID_SIZE]; GRID_SIZE];
        println!("Grid cleared!");
    }
}