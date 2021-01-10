use ggez;
use ggez::{
    conf::{FullscreenType, WindowMode},
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics,
    input::mouse::MouseButton,
    Context, GameResult,
};
// use ggez::graphics::{DrawParam, FilterMode, Font, Image, Rect};
// use ggez::nalgebra::Point2;

use std::env;
use std::path::PathBuf;

mod entity_data;

use entity_data::{Entity, EntityType};

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

struct MainState {
    world: Vec<Vec<Entity>>,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let world = vec![
            vec![Entity::new(0.0, 0.0, EntityType::Nothing); SCREEN_WIDTH as usize];
            SCREEN_HEIGHT as usize
        ];
        for x in 0..SCREEN_WIDTH as usize {
            for y in 0..SCREEN_HEIGHT as usize {
                world[y][x].set_pos(x as f32, y as f32);
            }
        }

        let s = MainState { world: world };
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.world[300][400].get_entity_type_as_ref() == &EntityType::Nothing {
            self.world[300][400].set_type(EntityType::Sand);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // draw background
        // don't need

        // draw everything else
        for x in 0..SCREEN_WIDTH as usize {
            for y in 0..SCREEN_HEIGHT as usize {
                self.world[y][x].draw(ctx)?;
            }
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        unimplemented!();
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        unimplemented!();
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        unimplemented!();
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        unimplemented!();
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        unimplemented!();
    }
}

pub fn main() -> GameResult {
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