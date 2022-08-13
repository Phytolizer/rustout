use std::process::exit;

use sdl2::event::Event;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const WIDTH32: u32 = WIDTH as u32;
const HEIGHT32: u32 = HEIGHT as u32;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Could not initialize SDL: {0}")]
    SdlInit(String),
    #[error("Could not initialize SDL subsystem: {0}")]
    SdlInitSubSystem(String),
    #[error("Could not create window: {0}")]
    SdlCreateWindow(sdl2::video::WindowBuildError),
    #[error("Could not create renderer: {0}")]
    SdlCreateRenderer(sdl2::IntegerOrSdlError),
    #[error("Could not create event pump: {0}")]
    SdlCreateEventPump(String),
}

type Result<T> = std::result::Result<T, Error>;

fn run() -> Result<()> {
    let sdl = sdl2::init().map_err(Error::SdlInit)?;
    let video_subsystem = sdl.video().map_err(Error::SdlInitSubSystem)?;
    let mut window = video_subsystem
        .window("rustout", WIDTH32, HEIGHT32)
        .build()
        .map_err(Error::SdlCreateWindow)?
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(Error::SdlCreateRenderer)?;
    let mut event_pump = sdl.event_pump().map_err(Error::SdlCreateEventPump)?;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                _ => {}
            }
        }
        window.clear();
        window.present();
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        exit(1);
    }
}
