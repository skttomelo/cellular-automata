#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ggez;
use ggez::{
    conf::{FullscreenType, WindowMode},
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics,
    graphics::{DrawMode, DrawParam, FillOptions, Mesh, MeshBuilder, Rect},
    input::mouse::MouseButton,
    Context, GameResult,
};

use specs::{Builder, RunNow, World, WorldExt};

// use cgmath::Vector2;

use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

// cellular-automata imports
mod components;
mod constants;
mod systems;

use components::{Material, MaterialType, Position, Velocity};
use constants::{COLORS, SCALE, SCREEN_HEIGHT, SCREEN_WIDTH};
use systems::{MovementSystem, SandSystem, WaterSystem, OverlapCorrectionSystem, DirtSystem};

struct Mouse {
    mouse_button: MouseButton,
    position: Position,
    mouse_held: bool,
}
impl Mouse {
    fn new() -> Mouse {
        Mouse {
            mouse_button: MouseButton::Left,
            position: Position(0.0, 0.0),
            mouse_held: false,
        }
    }
}

struct Systems {
    sand_system: SandSystem,
    water_system: WaterSystem,
    movement_system: MovementSystem,
    overlap_correction_system: OverlapCorrectionSystem,
    dirt_system: DirtSystem,
}

impl Systems {
    fn new() -> Systems {
        Systems {
            sand_system: SandSystem,
            water_system: WaterSystem,
            movement_system: MovementSystem,
            overlap_correction_system: OverlapCorrectionSystem,
            dirt_system: DirtSystem,
        }
    }

    fn run_now(&mut self, world: &World) {
        self.sand_system.run_now(world);
        self.water_system.run_now(world); // broken F to pay respect
        self.movement_system.run_now(world);
        self.overlap_correction_system.run_now(world);
        self.dirt_system.run_now(world);
    }
}

struct MainState {
    world: World,
    systems: Systems,
    mouse: Mouse,
    draw_param: DrawParam,
    meshes: HashMap<MaterialType, Mesh>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // create world and register components
        let mut world = World::new();
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Material>();

        // create rect
        let rect = Rect::new(0.0, 0.0, SCALE, SCALE);
        // create mesh
        let mut mesh_builder = MeshBuilder::new();

        let mut map: HashMap<MaterialType, Mesh> = HashMap::new();
        map.insert(
            MaterialType::Sand,
            mesh_builder
                .rectangle(
                    DrawMode::Fill(FillOptions::DEFAULT),
                    rect,
                    COLORS.get(&MaterialType::Sand).unwrap().clone(),
                )
                .build(ctx)
                .unwrap(),
        );
        map.insert(
            MaterialType::Water,
            mesh_builder
                .rectangle(
                    DrawMode::Fill(FillOptions::DEFAULT),
                    rect,
                    COLORS.get(&MaterialType::Water).unwrap().clone(),
                )
                .build(ctx)
                .unwrap(),
        );
        map.insert(
            MaterialType::Nothing,
            mesh_builder
                .rectangle(
                    DrawMode::Fill(FillOptions::DEFAULT),
                    rect,
                    COLORS.get(&MaterialType::Nothing).unwrap().clone(),
                )
                .build(ctx)
                .unwrap(),
        );
        map.insert(
            MaterialType::Dirt,
            mesh_builder
                .rectangle(
                    DrawMode::Fill(FillOptions::DEFAULT),
                    rect,
                    COLORS.get(&MaterialType::Dirt).unwrap().clone(),
                )
                .build(ctx)
                .unwrap(),
        );
        map.insert(
            MaterialType::Grass,
            mesh_builder
                .rectangle(
                    DrawMode::Fill(FillOptions::DEFAULT),
                    rect,
                    COLORS.get(&MaterialType::Grass).unwrap().clone(),
                )
                .build(ctx)
                .unwrap(),
        );

        let s = MainState {
            world: world,
            systems: Systems::new(),
            mouse: Mouse::new(),
            draw_param: DrawParam::new(),
            meshes: map,
        };
        Ok(s)
    }

    fn mouse_held(&mut self) {
        if self.mouse.mouse_held == true {
            match self.mouse.mouse_button {
                // place sand
                MouseButton::Left => self.place_entity(MaterialType::Sand),
                // palce water
                MouseButton::Right => self.place_entity(MaterialType::Water),
                // place nothing (black pixel)
                MouseButton::Other(1) => self.place_entity(MaterialType::Nothing),
                // place dirt
                MouseButton::Other(2) => self.place_entity(MaterialType::Dirt),
                _ => (),
            }
        }
    }

    fn place_entity(&mut self, material_type: MaterialType) {
        use specs::Join;
        // check to make sure there does not exist anything at the position we want to place our sand
        let mut obstructed = false;

        // have to use enclosure because immutable borrow occurs two lines down
        {
            let positions = self.world.read_storage::<Position>();

            for pos in (&positions).join() {
                if pos.0 == self.mouse.position.0 && pos.1 == self.mouse.position.1 {
                    obstructed = true;
                    break;
                }
            }
        }

        if obstructed == false {
            // register a new entity in the world with a Position, Velocity, & Material
            // println!("{:?}", mouse_position);
            self.world
                .create_entity()
                .with(Position(self.mouse.position.0, self.mouse.position.1))
                .with(Velocity {
                    vx: 0.0,
                    vy: 0.0,
                    last_vx: 0.0,
                    last_vy: 0.0,
                })
                .with(Material(material_type))
                .build();
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // placing entities on screen
        self.mouse_held();

        self.systems.run_now(&self.world);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        use specs::Join; // for joining components and iterating through them

        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let positions = self.world.read_storage::<Position>();
        let materials = self.world.read_storage::<Material>();

        for (pos, mat) in (&positions, &materials).join() {
            // draw mesh
            graphics::draw(
                ctx,
                self.meshes.get(&mat.0).unwrap(),
                self.draw_param.dest([pos.0 * SCALE, pos.1 * SCALE]),
            )
            .unwrap();
        }

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

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.mouse.position.0 = ((x / SCALE) as i32) as f32;
        self.mouse.position.1 = ((y / SCALE) as i32) as f32;
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.mouse.mouse_held = false;
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        // use specs::Join; // obviously it throws a warning because I have everything imported atm 😐
        self.mouse.position.0 = ((x / SCALE) as i32) as f32;
        self.mouse.position.1 = ((y / SCALE) as i32) as f32;
        self.mouse.mouse_button = button;
        self.mouse.mouse_held = true;
        // self.place_entity(MaterialType::Water);
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
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
