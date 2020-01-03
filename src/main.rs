extern crate graphics;

mod game_board;
mod game_board_controller;
mod game_board_view;

use crate::game_board::GameBoard;
use crate::game_board_controller::GameBoardController;
use crate::game_board_view::{GameBoardView, GameBoardViewSettings};
use glutin_window::{GlutinWindow, OpenGL};
use opengl_graphics::{Filter, GlGraphics, GlyphCache, TextureSettings};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("sudoku", [512; 2])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);

    let game_board = GameBoard::new();
    let mut game_board_controller = GameBoardController::new(game_board);
    let game_board_view_settings = GameBoardViewSettings::new();
    let mut game_board_view = GameBoardView::new(game_board_view_settings);

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
        .expect("Could not load font");

    let mut event_settings = EventSettings::new();
    event_settings.set_lazy(true);
    let mut events = Events::new(event_settings);

    while let Some(e) = events.next(&mut window) {
        game_board_controller.event(
            game_board_view.settings.position,
            game_board_view.settings.size,
            &e,
        );
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::*;

                clear([1.0; 4], g);
                game_board_view.draw(&game_board_controller, glyphs, &c, g);
            });
        }

        if let Some(args) = e.update_args() {}
    }
}
