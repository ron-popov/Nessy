use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::mapper::Mapper;
use crate::ppu::PPU;
use crate::core::consts;

use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};

struct UIState {
    ppu_mutex: Arc::<Mutex::<PPU>>,
}

impl UIState {
    fn new(ppu_mutex: Arc::<Mutex::<PPU>>) -> GameResult<UIState> {
        Ok(UIState{ppu_mutex: ppu_mutex})
    }
}

impl event::EventHandler for UIState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        log::info!("Update");
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        log::info!("Drawing board");

        let bitmap_lock = self.ppu_mutex.lock().unwrap();
        let bitmap = bitmap_lock.get_picture();

        for x in 0..bitmap.get_width() {
            log::trace!("{}", x);
            for y in 0..bitmap.get_height() {

                let p = bitmap.get_pixel(x, y).unwrap();

                let pos = graphics::Rect::new_i32(x as i32, y as i32, 1, 1);

                let color = match p.red {
                    255 => graphics::BLACK,
                    _ => graphics::WHITE,
                };

                let rect = graphics::Mesh::new_rectangle(_ctx, graphics::DrawMode::fill(), pos, color).unwrap();
                let dot = graphics::draw(_ctx, &rect, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
            }
        }

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
        let state = &mut UIState::new(ppu_mutex_for_ui).unwrap();
        let _ = event::run(ctx, event_loop, state);
    });

    // Start PPU Thread
    let ppu_thread = thread::spawn(move || {
        // let ppu_ref = ppu_mutex_for_ppu.lock();
        thread::sleep(Duration::from_secs(10));
    });

    return (ppu_thread, ui_thread);
}