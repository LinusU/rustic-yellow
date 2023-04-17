use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use glium::glutin::platform::run_return::EventLoopExtRunReturn;
use rustic_yellow::{AudioPlayer, Game, KeypadEvent};
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Default)]
struct RenderOptions {
    pub linear_interpolation: bool,
}

#[cfg(target_os = "windows")]
fn create_window_builder() -> glium::glutin::window::WindowBuilder {
    use glium::glutin::platform::windows::WindowBuilderExtWindows;
    glium::glutin::window::WindowBuilder::new()
        .with_drag_and_drop(false)
        .with_inner_size(glium::glutin::dpi::LogicalSize::<u32>::from((
            rustic_yellow::SCREEN_W as u32,
            rustic_yellow::SCREEN_H as u32,
        )))
        .with_title("Rustic Yellow")
}

#[cfg(not(target_os = "windows"))]
fn create_window_builder() -> glium::glutin::window::WindowBuilder {
    glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::<u32>::from((
            rustic_yellow::SCREEN_W as u32,
            rustic_yellow::SCREEN_H as u32,
        )))
        .with_title("Rustic Yellow")
}

fn main() {
    let scale = 4;

    let (player, cpal_audio_stream) = CpalPlayer::get();

    let (sender1, receiver1) = mpsc::channel();
    let (sender2, receiver2) = mpsc::sync_channel(1);

    let mut eventloop = glium::glutin::event_loop::EventLoop::new();
    let window_builder = create_window_builder();
    let context_builder = glium::glutin::ContextBuilder::new();
    let display =
        glium::backend::glutin::Display::new(window_builder, context_builder, &eventloop).unwrap();
    set_window_size(display.gl_window().window(), scale);

    let mut texture = glium::texture::texture2d::Texture2d::empty_with_format(
        &display,
        glium::texture::UncompressedFloatFormat::U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        rustic_yellow::SCREEN_W as u32,
        rustic_yellow::SCREEN_H as u32,
    )
    .unwrap();

    let mut renderoptions = <RenderOptions as Default>::default();

    let gamethread = thread::spawn(move || run_game(Box::new(player), sender2, receiver1));

    let periodic = timer_periodic(16_743);

    #[rustfmt::skip]
    eventloop.run_return(move |ev, _evtarget, controlflow| {
        use glium::glutin::event::ElementState::{Pressed, Released};
        use glium::glutin::event::VirtualKeyCode;
        use glium::glutin::event::{Event, KeyboardInput, WindowEvent};

        let mut stop = false;
        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => stop = true,
                WindowEvent::KeyboardInput { input, .. } => match input {
                    KeyboardInput { state: Pressed, virtual_keycode: Some(VirtualKeyCode::Escape), .. }
                        => stop = true,
                    KeyboardInput { state: Pressed, virtual_keycode: Some(VirtualKeyCode::Key1), .. }
                        => set_window_size(display.gl_window().window(), 1),
                    KeyboardInput { state: Pressed, virtual_keycode: Some(VirtualKeyCode::R), .. }
                        => set_window_size(display.gl_window().window(), scale),
                    KeyboardInput { state: Pressed, virtual_keycode: Some(VirtualKeyCode::T), .. }
                        => { renderoptions.linear_interpolation = !renderoptions.linear_interpolation; }
                    KeyboardInput { state: Pressed, virtual_keycode: Some(glutinkey), .. } => {
                        if let Some(key) = glutin_to_keypad(glutinkey) {
                            let _ = sender1.send(KeypadEvent::Down(key));
                        }
                    },
                    KeyboardInput { state: Released, virtual_keycode: Some(glutinkey), .. } => {
                        if let Some(key) = glutin_to_keypad(glutinkey) {
                            let _ = sender1.send(KeypadEvent::Up(key));
                        }
                    }
                    _ => (),
                },
                _ => (),
            },
            Event::MainEventsCleared => {
                match receiver2.recv() {
                    Ok(data) => {
                        periodic.recv().unwrap();
                        recalculate_screen(&display, &mut texture, &data, &renderoptions);
                    }
                    Err(..) => stop = true, // Remote end has hung-up
                }
            }
            _ => (),
        }
        if stop {
            *controlflow = glium::glutin::event_loop::ControlFlow::Exit;
        }
    });

    drop(cpal_audio_stream);
    let _ = gamethread.join();
}

fn glutin_to_keypad(key: glium::glutin::event::VirtualKeyCode) -> Option<rustic_yellow::KeypadKey> {
    use glium::glutin::event::VirtualKeyCode;
    match key {
        VirtualKeyCode::Z => Some(rustic_yellow::KeypadKey::A),
        VirtualKeyCode::X => Some(rustic_yellow::KeypadKey::B),
        VirtualKeyCode::Up => Some(rustic_yellow::KeypadKey::Up),
        VirtualKeyCode::Down => Some(rustic_yellow::KeypadKey::Down),
        VirtualKeyCode::Left => Some(rustic_yellow::KeypadKey::Left),
        VirtualKeyCode::Right => Some(rustic_yellow::KeypadKey::Right),
        VirtualKeyCode::Space => Some(rustic_yellow::KeypadKey::Select),
        VirtualKeyCode::Return => Some(rustic_yellow::KeypadKey::Start),
        _ => None,
    }
}

fn recalculate_screen(
    display: &glium::Display,
    texture: &mut glium::texture::texture2d::Texture2d,
    datavec: &[u8],
    renderoptions: &RenderOptions,
) {
    use glium::Surface;

    let interpolation_type = if renderoptions.linear_interpolation {
        glium::uniforms::MagnifySamplerFilter::Linear
    } else {
        glium::uniforms::MagnifySamplerFilter::Nearest
    };

    let rawimage2d = glium::texture::RawImage2d {
        data: std::borrow::Cow::Borrowed(datavec),
        width: rustic_yellow::SCREEN_W as u32,
        height: rustic_yellow::SCREEN_H as u32,
        format: glium::texture::ClientFormat::U8U8U8,
    };
    texture.write(
        glium::Rect {
            left: 0,
            bottom: 0,
            width: rustic_yellow::SCREEN_W as u32,
            height: rustic_yellow::SCREEN_H as u32,
        },
        rawimage2d,
    );

    // We use a custom BlitTarget to transform OpenGL coordinates to row-column coordinates
    let target = display.draw();
    let (target_w, target_h) = target.get_dimensions();
    texture.as_surface().blit_whole_color_to(
        &target,
        &glium::BlitTarget {
            left: 0,
            bottom: target_h,
            width: target_w as i32,
            height: -(target_h as i32),
        },
        interpolation_type,
    );
    target.finish().unwrap();
}

fn run_game(
    player: Box<dyn AudioPlayer>,
    sender: SyncSender<Vec<u8>>,
    receiver: Receiver<KeypadEvent>,
) {
    Game::new(player, sender, receiver).boot();
}

fn timer_periodic(micros: u64) -> Receiver<()> {
    let (tx, rx) = std::sync::mpsc::sync_channel(1);
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_micros(micros));
        if tx.send(()).is_err() {
            break;
        }
    });
    rx
}

fn set_window_size(window: &glium::glutin::window::Window, scale: u32) {
    use glium::glutin::dpi::{LogicalSize, PhysicalSize};

    let dpi = window.scale_factor();

    let physical_size = PhysicalSize::<u32>::from((
        rustic_yellow::SCREEN_W as u32 * scale,
        rustic_yellow::SCREEN_H as u32 * scale,
    ));
    let logical_size = LogicalSize::<u32>::from_physical(physical_size, dpi);

    window.set_inner_size(logical_size);
}

struct CpalPlayer {
    buffer: Arc<Mutex<Vec<(f32, f32)>>>,
    sample_rate: u32,
}

impl CpalPlayer {
    fn get() -> (CpalPlayer, cpal::Stream) {
        let device = match cpal::default_host().default_output_device() {
            Some(e) => e,
            None => panic!("No audio device found"),
        };

        // We want a config with:
        // chanels = 2
        // SampleFormat F32
        // Rate at around 44100

        let wanted_samplerate = cpal::SampleRate(44100);
        let supported_configs = match device.supported_output_configs() {
            Ok(e) => e,
            Err(e) => panic!("Error while querying configs: {:?}", e),
        };
        let mut supported_config = None;
        for f in supported_configs {
            if f.channels() == 2 && f.sample_format() == cpal::SampleFormat::F32 {
                if f.min_sample_rate() <= wanted_samplerate
                    && wanted_samplerate <= f.max_sample_rate()
                {
                    supported_config = Some(f.with_sample_rate(wanted_samplerate));
                } else {
                    supported_config = Some(f.with_max_sample_rate());
                }
                break;
            }
        }
        if supported_config.is_none() {
            panic!("No supported audio configuration found");
        }

        let selected_config = supported_config.unwrap();

        let sample_format = selected_config.sample_format();
        let config: cpal::StreamConfig = selected_config.into();

        let err_fn = |err| eprintln!("An error occurred on the output audio stream: {}", err);

        let shared_buffer = Arc::new(Mutex::new(Vec::new()));
        let stream_buffer = shared_buffer.clone();

        let player = CpalPlayer {
            buffer: shared_buffer,
            sample_rate: config.sample_rate.0,
        };

        #[rustfmt::skip]
        let stream = match sample_format {
            cpal::SampleFormat::F32 => device.build_output_stream(&config, move|data: &mut [f32], _callback_info: &cpal::OutputCallbackInfo| cpal_thread(data, &stream_buffer), err_fn, None).unwrap(),
            cpal::SampleFormat::U16 => device.build_output_stream(&config, move|data: &mut [u16], _callback_info: &cpal::OutputCallbackInfo| cpal_thread(data, &stream_buffer), err_fn, None).unwrap(),
            cpal::SampleFormat::I16 => device.build_output_stream(&config, move|data: &mut [i16], _callback_info: &cpal::OutputCallbackInfo| cpal_thread(data, &stream_buffer), err_fn, None).unwrap(),
            _ => panic!("Unsupported sample format: {:?}", sample_format),
        };

        stream.play().unwrap();

        (player, stream)
    }
}

fn cpal_thread<T: cpal::Sample + cpal::FromSample<f32>>(
    outbuffer: &mut [T],
    audio_buffer: &Arc<Mutex<Vec<(f32, f32)>>>,
) {
    let mut inbuffer = audio_buffer.lock().unwrap();
    let outlen = ::std::cmp::min(outbuffer.len() / 2, inbuffer.len());
    for (i, (in_l, in_r)) in inbuffer.drain(..outlen).enumerate() {
        outbuffer[i * 2] = cpal::Sample::from_sample(in_l);
        outbuffer[i * 2 + 1] = cpal::Sample::from_sample(in_r);
    }
}

impl AudioPlayer for CpalPlayer {
    fn play(&mut self, buf_left: &[f32], buf_right: &[f32]) {
        debug_assert!(buf_left.len() == buf_right.len());

        let mut buffer = self.buffer.lock().unwrap();

        for (l, r) in buf_left.iter().zip(buf_right) {
            if buffer.len() > self.sample_rate as usize {
                // Do not fill the buffer with more than 1 second of data
                // This speeds up the resync after the turning on and off the speed limiter
                return;
            }
            buffer.push((*l, *r));
        }
    }

    fn samples_rate(&self) -> u32 {
        self.sample_rate
    }

    fn underflowed(&self) -> bool {
        (*self.buffer.lock().unwrap()).is_empty()
    }
}
