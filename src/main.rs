mod generator;
mod solver;

use generator::Method;
use ggez::{conf, event, graphics, Context, ContextBuilder, GameError, GameResult};
use std::path;

/// A grid of 20x20 tiles.
pub const GRID_SIZE: i16 = 100;

/// Sutible size of each tile. (Width, Height)
const GRID_CELL_SIZE: (i16, i16) = (8, 8);

/// Size of the application window.
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE as f32 * GRID_CELL_SIZE.1 as f32,
);

// GUI Color representations
const BLACK: graphics::Color =
    graphics::Color::new(188f32 / 255f32, 140f32 / 255f32, 76f32 / 255f32, 1f32);
const WHITE: graphics::Color =
    graphics::Color::new(255f32 / 255f32, 200f32 / 255f32, 128f32 / 255f32, 1f32);

/// GUI logic and event implementation structure.
struct AppState {
    selected_square: Option<(usize, usize)>,
    highlighted_squares: Vec<(usize, usize)>,
    grid: Vec<Vec<u8>>,
    method: Method,
    col: bool,
}

impl AppState {
    /// Initialise new application, i.e. initialise new game and load resources.
    fn new(ctx: &mut Context) -> GameResult<AppState> {
        let state = AppState {
            selected_square: None,
            highlighted_squares: Vec::new(),
            grid: vec![vec![0; GRID_SIZE as usize]; GRID_SIZE as usize],
            col: false,
            method: Method::RecursiveBacktracking
        };

        Ok(state)
    }
}

impl event::EventHandler<GameError> for AppState {
    /// For updating game logic, which front-end doesn't handle.
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    /// Draw interface, i.e. draw game board
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // clear interface with gray background colour
        graphics::clear(ctx, BLACK);

        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new_i32(
                        GRID_CELL_SIZE.0 as i32 * i as i32,
                        GRID_CELL_SIZE.1 as i32 * j as i32,
                        GRID_CELL_SIZE.0 as i32,
                        GRID_CELL_SIZE.1 as i32,
                    ),
                    if self.grid[i as usize][j as usize] == 0 {
                        BLACK
                    } else if self.grid[i as usize][j as usize] == 1 {
                        WHITE
                    } else if self.grid[i as usize][j as usize] == 2 {
                        graphics::Color::RED
                    } else if self.grid[i as usize][j as usize] == 4 {
                        graphics::Color::BLUE
                    } else {
                        graphics::Color::BLACK
                    },
                )
                .expect("Failed to create history background.");
                graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
                    .expect("Failed to draw history background");
            }
        }

        graphics::present(ctx).expect("Failed to update graphics.");
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.grid = vec![vec![0; GRID_SIZE as usize]; GRID_SIZE as usize];
        generator::new(&mut self.grid, &self.method);
    }
}

pub fn main() -> GameResult {
    //let resource_dir = path::PathBuf::from("./resources");

    let context_builder = ContextBuilder::new("maze", "Emil Hultcrantz")
        //.add_resource_path(resource_dir) // Import image files to GGEZ
        .window_setup(
            conf::WindowSetup::default()
                .title("Maze") // Set window title "Chess"
                //.icon("/icon.png"), // Set application icon
        )
        .window_mode(
            conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1) // Set window dimensions
                .resizable(false), // Fixate window size
        )
        .modules(conf::ModuleConf::default().audio(false));
    let (mut contex, event_loop) = context_builder.build().expect("Failed to build context.");

    let state = AppState::new(&mut contex).expect("Failed to create state.");
    event::run(contex, event_loop, state) // Run window event loop
}
