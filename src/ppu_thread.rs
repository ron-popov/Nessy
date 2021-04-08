use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::mapper::Mapper;
use crate::ppu::PPU;
use crate::core::consts;

use nwg::NativeUi;
use nwd::NwgUi;

#[derive(NwgUi)]
pub struct PPUCanvas {
    #[nwg_control(size: (consts::NES_SCREEN_WIDTH as i32, consts::NES_SCREEN_HEIGHT as i32), title: "Nessy", flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnInit: [PPUCanvas::init])]
    window: nwg::Window,

    // Refresh timer
    #[nwg_control(parent: window, interval: Duration::from_millis(1000/consts::NES_SCREEN_REFRESH_RATE_HZ))]
    #[nwg_events(OnTimerTick: [PPUCanvas::update_frame])]
    refresh_timer: nwg::AnimationTimer,

    ppu_mutex: Arc::<Mutex::<PPU>>,
}

impl PPUCanvas {
    pub fn init(&self) {
        log::debug!("Initializing GUI");
        self.refresh_timer.start();
    }

    pub fn update_frame(&self) {
        log::debug!("Updating frame");
    }
}

pub fn start_ppu_thread(mapper_mutex: Arc::<Mutex::<Box::<dyn Mapper>>>) -> (thread::JoinHandle<()>, thread::JoinHandle<()>) {
    // Start UI Thread
    let mut ppu = PPU::new(mapper_mutex);

    let ppu_mutex_for_ppu: Arc::<Mutex::<PPU>> = Arc::new(Mutex::new(ppu));
    let ppu_mutex_for_ui: Arc::<Mutex::<PPU>> = Arc::clone(&ppu_mutex_for_ppu);

    // Start UI Thread
    let ui_thread = thread::spawn(move || {
        nwg::init().expect("Failed to initialize Native Windows GUI");

        log::info!("Initiaized Native Windows GUI, initializing ppu canvas");

        let ppu_canvas = PPUCanvas{ppu_mutex: ppu_mutex_for_ui, window: nwg::Window::default(), 
            refresh_timer: nwg::AnimationTimer::default()};
        let _ppu_canvas_ui = PPUCanvas::build_ui(ppu_canvas).expect("Failed to build UI");

        log::info!("Initiaized ppu canvas, dispatching NWG Events");

        nwg::dispatch_thread_events();

        log::info!("Stopped dispatching NWG Events");
    });

    // Start PPU Thread
    let ppu_thread = thread::spawn(move || {
        // thread::sleep_ms(5000);
    });

    return (ppu_thread, ui_thread);
}