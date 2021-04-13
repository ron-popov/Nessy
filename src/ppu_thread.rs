use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

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

        let start_time = Instant::now();

        let bitmap_lock = self.ppu_mutex.lock().unwrap();
        let bitmap = (*bitmap_lock.get_picture()).clone();
        drop(bitmap_lock); // Drop bitmap_lock as soon as possible because the PPU cant run when the lock is held

        // // Transform bitmap to a lot of rectangles - very very slow
        graphics::clear(_ctx, graphics::Color::from((255, 255, 255, 255)));

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
                let _ = graphics::draw(_ctx, &rect, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
            }
        }

        let present_res = graphics::present(_ctx);
        log::debug!("Present result {:?}", present_res);

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
        for _ in 0..10 {
            let mut ppu_ref = ppu_mutex_for_ppu.lock().unwrap();
            ppu_ref.update_frame();
            drop(ppu_ref);

            thread::sleep(Duration::from_secs(5));
        }
    });

    return (ppu_thread, ui_thread);
}