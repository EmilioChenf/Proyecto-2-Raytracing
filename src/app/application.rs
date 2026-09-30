use std::time::Instant;

use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Scale, Window, WindowOptions};

use crate::{
    camera::OrbitCamera,
    characters::CharacterKind,
    math::Vec3,
    renderer::RayTracer,
    scene::{
        create_character_selection, create_main_world, create_panda_world, create_pardo_world,
        create_polar_world, CharacterSelection, CharacterWorld, Scene,
    },
};

use super::{GameState, StateMachine};

const WIDTH: usize = 480;
const HEIGHT: usize = 270;
const MAIN_WORLD_TRIGGER_DISTANCE: f32 = 8.2;
const ORBIT_SPEED: f32 = 1.35;
const ZOOM_SPEED: f32 = 12.0;
const CHARACTER_ROTATION_SPEED: f32 = 0.55;

pub fn run() -> Result<(), minifb::Error> {
    let mut window = Window::new(
        "Proyecto 2: Diorama con Ray Tracing",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    )?;
    window.set_target_fps(60);

    let mut app = Application::new();
    let mut pixels = app.render();
    let mut previous_frame = Instant::now();

    while window.is_open() {
        let now = Instant::now();
        let delta_seconds = (now - previous_frame).as_secs_f32().min(0.1);
        previous_frame = now;

        let dirty = app.update(&window, delta_seconds);
        if dirty {
            pixels = app.render();
            window.set_title(app.title());
        }
        window.update_with_buffer(&pixels, WIDTH, HEIGHT)?;
    }

    Ok(())
}

struct Application {
    states: StateMachine,
    main_world: Scene,
    selection: CharacterSelection,
    polar_world: CharacterWorld,
    pardo_world: CharacterWorld,
    panda_world: CharacterWorld,
    camera: OrbitCamera,
    tracer: RayTracer,
    auto_rotate: bool,
    character_angle: f32,
    previous_mouse: Option<(f32, f32)>,
    previous_left_down: bool,
}

impl Application {
    fn new() -> Self {
        Self {
            states: StateMachine::default(),
            main_world: create_main_world(),
            selection: create_character_selection(),
            polar_world: create_polar_world(),
            pardo_world: create_pardo_world(),
            panda_world: create_panda_world(),
            camera: camera_for(GameState::MainWorld),
            tracer: RayTracer::default(),
            auto_rotate: true,
            character_angle: 0.0,
            previous_mouse: None,
            previous_left_down: false,
        }
    }

    fn update(&mut self, window: &Window, delta_seconds: f32) -> bool {
        let mut dirty = false;

        if let Some(entered) = self.states.update(delta_seconds) {
            self.camera = camera_for(entered);
            self.character_angle = 0.0;
            dirty = true;
        }
        if self.states.current() == GameState::Transition {
            return true;
        }

        if window.is_key_pressed(Key::Escape, KeyRepeat::No) {
            if let Some(target) = self.states.current().back_target() {
                self.states.begin_transition(target);
                return true;
            }
        }
        if window.is_key_pressed(Key::R, KeyRepeat::No)
            && self.states.current().is_character_world()
        {
            self.auto_rotate = !self.auto_rotate;
            dirty = true;
        }

        dirty |= self.update_camera(window, delta_seconds);
        dirty |= self.update_selection(window);

        if self.states.current() == GameState::MainWorld
            && self.camera.distance <= MAIN_WORLD_TRIGGER_DISTANCE
        {
            self.states.begin_transition(GameState::CharacterSelection);
            return true;
        }

        if self.auto_rotate && self.states.current().is_character_world() {
            self.character_angle = (self.character_angle
                + delta_seconds * CHARACTER_ROTATION_SPEED)
                .rem_euclid(std::f32::consts::TAU);
            let angle = self.character_angle;
            self.active_character_world_mut()
                .set_character_rotation(angle);
            dirty = true;
        }

        dirty
    }

    fn update_camera(&mut self, window: &Window, delta_seconds: f32) -> bool {
        let mut changed = false;
        let yaw = (key_axis(window, Key::A, Key::D)) * ORBIT_SPEED * delta_seconds;
        let pitch = (key_axis(window, Key::Q, Key::E)) * ORBIT_SPEED * delta_seconds;
        if yaw != 0.0 || pitch != 0.0 {
            self.camera.orbit(yaw, pitch);
            changed = true;
        }
        let zoom = key_axis(window, Key::S, Key::W) * ZOOM_SPEED * delta_seconds;
        if zoom != 0.0 {
            self.camera.zoom(zoom);
            changed = true;
        }
        if let Some((_, scroll_y)) = window.get_scroll_wheel() {
            if scroll_y != 0.0 {
                self.camera.zoom(scroll_y * 1.2);
                changed = true;
            }
        }

        let mouse = window.get_mouse_pos(MouseMode::Clamp);
        if window.get_mouse_down(MouseButton::Left) {
            if let (Some(current), Some(previous)) = (mouse, self.previous_mouse) {
                let delta_x = current.0 - previous.0;
                let delta_y = current.1 - previous.1;
                if delta_x != 0.0 || delta_y != 0.0 {
                    self.camera.orbit(-delta_x * 0.007, -delta_y * 0.007);
                    changed = true;
                }
            }
        }
        self.previous_mouse = mouse;
        changed
    }

    fn update_selection(&mut self, window: &Window) -> bool {
        let left_down = window.get_mouse_down(MouseButton::Left);
        if self.states.current() != GameState::CharacterSelection {
            self.previous_left_down = left_down;
            return false;
        }
        let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Discard) else {
            self.previous_left_down = left_down;
            return false;
        };
        let ray = self.camera.ray_from_screen(mouse_x, mouse_y, WIDTH, HEIGHT);
        let changed = self.selection.update_hover(&ray);
        if left_down && !self.previous_left_down {
            if let Some(character) = self.selection.pick(&ray) {
                self.states.begin_transition(state_for(character));
            }
        }
        self.previous_left_down = left_down;
        changed
    }

    fn render(&self) -> Vec<u32> {
        self.tracer
            .render(self.active_scene(), &self.camera, WIDTH, HEIGHT)
    }

    fn active_scene(&self) -> &Scene {
        match self.states.visible() {
            GameState::MainWorld => &self.main_world,
            GameState::CharacterSelection => &self.selection.scene,
            GameState::PolarWorld => &self.polar_world.scene,
            GameState::PardoWorld => &self.pardo_world.scene,
            GameState::PandaWorld => &self.panda_world.scene,
            GameState::Transition => &self.main_world,
        }
    }

    fn active_character_world_mut(&mut self) -> &mut CharacterWorld {
        match self.states.current() {
            GameState::PolarWorld => &mut self.polar_world,
            GameState::PardoWorld => &mut self.pardo_world,
            GameState::PandaWorld => &mut self.panda_world,
            _ => unreachable!("called only while an individual character world is active"),
        }
    }

    fn title(&self) -> &'static str {
        match self.states.visible() {
            GameState::MainWorld => "Isla flotante - acércate para continuar",
            GameState::CharacterSelection => "Selecciona un personaje con clic",
            GameState::PolarWorld => "Mundo Polar - R rotación | ESC volver",
            GameState::PardoWorld => "Mundo Pardo - R rotación | ESC volver",
            GameState::PandaWorld => "Mundo Panda - R rotación | ESC volver",
            GameState::Transition => "Transición",
        }
    }
}

fn camera_for(state: GameState) -> OrbitCamera {
    match state {
        GameState::MainWorld => {
            OrbitCamera::new(Vec3::new(0.0, -0.5, 0.0), 21.5, 2.55, 0.3).with_limits(7.0, 45.0)
        }
        GameState::CharacterSelection => {
            OrbitCamera::new(Vec3::new(0.0, 2.5, 0.0), 15.5, std::f32::consts::PI, 0.08)
                .with_limits(10.0, 28.0)
        }
        GameState::PolarWorld | GameState::PardoWorld | GameState::PandaWorld => {
            OrbitCamera::new(Vec3::new(0.0, 2.0, 0.0), 18.0, 3.45, 0.2).with_limits(9.0, 34.0)
        }
        GameState::Transition => {
            OrbitCamera::new(Vec3::zeros(), 20.0, 0.0, 0.2).with_limits(7.0, 40.0)
        }
    }
}

fn state_for(character: CharacterKind) -> GameState {
    match character {
        CharacterKind::Polar => GameState::PolarWorld,
        CharacterKind::Pardo => GameState::PardoWorld,
        CharacterKind::Panda => GameState::PandaWorld,
    }
}

fn key_axis(window: &Window, negative: Key, positive: Key) -> f32 {
    f32::from(window.is_key_down(positive)) - f32::from(window.is_key_down(negative))
}
