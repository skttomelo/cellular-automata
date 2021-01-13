use ggez;
use ggez::{
    conf::{FullscreenType, WindowMode},
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics,
    graphics::{DrawParam, FilterMode, Font, Image, Rect},
    input::mouse::MouseButton,
    nalgebra::Point2,
    Context, GameResult,
};

// use cgmath::Vector2;

use std::env;
use std::path::PathBuf;

// cellular-automata imports
mod constants;
mod components;
mod systems;

use constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

struct MainState; // TODO

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState;
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // draw background
        // graphics::draw(
        //     ctx,
        //     &self.background_assets[0],
        //     graphics::DrawParam::new()
        //         .dest(Point2::<f32>::new(0.0, 0.0))
        //         .scale(Vector2::<f32>::new(4.0, 4.0)),
        // )
        // .expect("Draw call failed");

        // draw everything else

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        // TODO
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods) {
        // TODO
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) {
        // TODO
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        // TODO
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        // TODO
    }
}

pub fn main() -> GameResult {
    // command line args
    // let args: Vec<String> = env::args().collect();
    // let ip_address = match args.len() {
    //     2 => args[1].clone(),
    //     _ => String::from("127.0.0.1:1337"),
    // };

    // window
    let window = WindowMode {
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: false,
    };

    let mut cb = ggez::ContextBuilder::new("Vagabond Client", "Trevor Crow");

    // get and add resource path
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        cb = cb.add_resource_path(path);
    }

    // build and split context builder with window configuration
    let (ctx, event_loop) = &mut cb.window_mode(window).build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
