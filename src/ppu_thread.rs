use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::mapper::Mapper;
use crate::ppu::PPU;
use crate::core::consts;

use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};

struct UIState {}

impl UIState {
    fn new() -> GameResult<UIState> {
        Ok(UIState{})
    }
}

impl event::EventHandler for UIState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        log::info!("Update");
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        log::info!("Drawing board");

        let pos = graphics::Rect::new_i32(20, 50, 10, 10);
        let rect = graphics::Mesh::new_rectangle(_ctx, graphics::DrawMode::fill(), pos, graphics::BLACK).unwrap();
        let dot = graphics::draw(_ctx, &rect, (ggez::mint::Point2 { x: 1.0, y: 1.0 },));
        log::debug!("Draw result {:?}", dot);

        log::debug!("Present result {:?}", graphics::present(_ctx));

        Ok(())
    }
}

pub fn start_ppu_thread(mapper_mutex: Arc::<Mutex::<Box::<dyn Mapper>>>) -> (thread::JoinHandle<()>, thread::JoinHandle<()>) {
    // Start UI Thread
    let mut ppu = PPU::new(mapper_mutex);

    let ppu_mutex_for_ppu: Arc::<Mutex::<PPU>> = Arc::new(Mutex::new(ppu));
    let ppu_mutex_for_ui: Arc::<Mutex::<PPU>> = Arc::clone(&ppu_mutex_for_ppu);

    // Start UI Thread
    let ui_thread = thread::spawn(move || {
        let cb = ggez::ContextBuilder::new("Nessy", "Ron Popov")
            .window_setup(ggez::conf::WindowSetup::default().title("Nessy"))
            .window_mode(ggez::conf::WindowMode::default().dimensions(consts::NES_SCREEN_WIDTH as f32, consts::NES_SCREEN_HEIGHT as f32));
        let (ctx, event_loop) = &mut cb.build().unwrap();
        let state = &mut UIState::new().unwrap();
        let _ = event::run(ctx, event_loop, state);
    });

    // Start PPU Thread
    let ppu_thread = thread::spawn(move || {
        // let ppu_ref = ppu_mutex_for_ppu.lock();
        thread::sleep(Duration::from_secs(3));
    });

    return (ppu_thread, ui_thread);
}