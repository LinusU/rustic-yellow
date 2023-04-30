use glium::glutin::platform::run_return::EventLoopExtRunReturn;
use rustic_yellow::{Game, KeyboardEvent};
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::sync::{atomic::AtomicU64, Arc};
use std::thread;

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
    let render_delay = Arc::new(AtomicU64::new(16_743));

    let (sender1, receiver1) = mpsc::channel();
    let (sender2, receiver2) = mpsc::sync_channel(1);

    let mut eventloop = glium::glutin::event_loop::EventLoop::new();
    let window_builder = create_window_builder();
    let context_builder = glium::glutin::ContextBuilder::new();
    let display =
        glium::backend::glutin::Display::new(window_builder, context_builder, &eventloop).unwrap();
    set_window_size(display.gl_window().window());

    let mut texture = glium::texture::texture2d::Texture2d::empty_with_format(
        &display,
        glium::texture::UncompressedFloatFormat::U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        rustic_yellow::SCREEN_W as u32,
        rustic_yellow::SCREEN_H as u32,
    )
    .unwrap();

    let gamethread = thread::spawn(move || run_game(sender2, receiver1));

    let periodic = timer_periodic(render_delay.clone());

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
                    KeyboardInput { state: Pressed, virtual_keycode: Some(VirtualKeyCode::Key1), .. }
                        => render_delay.store(16_743, std::sync::atomic::Ordering::Relaxed), // 59.7 fps
                    KeyboardInput { state: Pressed, virtual_keycode: Some(VirtualKeyCode::Key2), .. }
                        => render_delay.store(10_000, std::sync::atomic::Ordering::Relaxed), // 100 fps
                    KeyboardInput { state: Pressed, virtual_keycode: Some(VirtualKeyCode::Key3), .. }
                        => render_delay.store(8_333, std::sync::atomic::Ordering::Relaxed), // 120 fps
                    KeyboardInput { state: Pressed, virtual_keycode: Some(VirtualKeyCode::Key4), .. }
                        => render_delay.store(5_000, std::sync::atomic::Ordering::Relaxed), // 200 fps
                    KeyboardInput { state: Pressed, virtual_keycode: Some(VirtualKeyCode::Key5), .. }
                        => render_delay.store(4_166, std::sync::atomic::Ordering::Relaxed), // 240 fps
                    KeyboardInput { state: Pressed, virtual_keycode: Some(VirtualKeyCode::Key6), .. }
                        => render_delay.store(2_500, std::sync::atomic::Ordering::Relaxed), // 400 fps
                    KeyboardInput { state: Pressed, virtual_keycode: Some(glutinkey), modifiers, .. } => {
                        if let Some(key) = glutin_to_keyboard(glutinkey) {
                            let _ = sender1.send(KeyboardEvent::Down { key, shift: modifiers.shift() });
                        }
                    },
                    KeyboardInput { state: Released, virtual_keycode: Some(glutinkey), .. } => {
                        if let Some(key) = glutin_to_keyboard(glutinkey) {
                            let _ = sender1.send(KeyboardEvent::Up { key });
                        }
                    }
                    _ => (),
                },
                _ => (),
            },
            Event::MainEventsCleared => {
                periodic.recv().unwrap();

                match receiver2.try_recv() {
                    Ok(data) => { recalculate_screen(&display, &mut texture, &data); },
                    Err(mpsc::TryRecvError::Empty) => (),
                    Err(..) => stop = true, // Remote end has hung-up
                }
            }
            _ => (),
        }
        if stop {
            *controlflow = glium::glutin::event_loop::ControlFlow::Exit;
        }
    });

    let _ = gamethread.join();
}

fn glutin_to_keyboard(
    key: glium::glutin::event::VirtualKeyCode,
) -> Option<rustic_yellow::KeyboardKey> {
    use glium::glutin::event::VirtualKeyCode;
    match key {
        VirtualKeyCode::Escape => Some(rustic_yellow::KeyboardKey::Escape),
        VirtualKeyCode::Left => Some(rustic_yellow::KeyboardKey::Left),
        VirtualKeyCode::Up => Some(rustic_yellow::KeyboardKey::Up),
        VirtualKeyCode::Right => Some(rustic_yellow::KeyboardKey::Right),
        VirtualKeyCode::Down => Some(rustic_yellow::KeyboardKey::Down),
        VirtualKeyCode::Back => Some(rustic_yellow::KeyboardKey::Backspace),
        VirtualKeyCode::Return => Some(rustic_yellow::KeyboardKey::Return),
        VirtualKeyCode::Space => Some(rustic_yellow::KeyboardKey::Space),
        VirtualKeyCode::A => Some(rustic_yellow::KeyboardKey::A),
        VirtualKeyCode::B => Some(rustic_yellow::KeyboardKey::B),
        VirtualKeyCode::C => Some(rustic_yellow::KeyboardKey::C),
        VirtualKeyCode::D => Some(rustic_yellow::KeyboardKey::D),
        VirtualKeyCode::E => Some(rustic_yellow::KeyboardKey::E),
        VirtualKeyCode::F => Some(rustic_yellow::KeyboardKey::F),
        VirtualKeyCode::G => Some(rustic_yellow::KeyboardKey::G),
        VirtualKeyCode::H => Some(rustic_yellow::KeyboardKey::H),
        VirtualKeyCode::I => Some(rustic_yellow::KeyboardKey::I),
        VirtualKeyCode::J => Some(rustic_yellow::KeyboardKey::J),
        VirtualKeyCode::K => Some(rustic_yellow::KeyboardKey::K),
        VirtualKeyCode::L => Some(rustic_yellow::KeyboardKey::L),
        VirtualKeyCode::M => Some(rustic_yellow::KeyboardKey::M),
        VirtualKeyCode::N => Some(rustic_yellow::KeyboardKey::N),
        VirtualKeyCode::O => Some(rustic_yellow::KeyboardKey::O),
        VirtualKeyCode::P => Some(rustic_yellow::KeyboardKey::P),
        VirtualKeyCode::Q => Some(rustic_yellow::KeyboardKey::Q),
        VirtualKeyCode::R => Some(rustic_yellow::KeyboardKey::R),
        VirtualKeyCode::S => Some(rustic_yellow::KeyboardKey::S),
        VirtualKeyCode::T => Some(rustic_yellow::KeyboardKey::T),
        VirtualKeyCode::U => Some(rustic_yellow::KeyboardKey::U),
        VirtualKeyCode::V => Some(rustic_yellow::KeyboardKey::V),
        VirtualKeyCode::W => Some(rustic_yellow::KeyboardKey::W),
        VirtualKeyCode::X => Some(rustic_yellow::KeyboardKey::X),
        VirtualKeyCode::Y => Some(rustic_yellow::KeyboardKey::Y),
        VirtualKeyCode::Z => Some(rustic_yellow::KeyboardKey::Z),

        _ => None,
    }
}

fn recalculate_screen(
    display: &glium::Display,
    texture: &mut glium::texture::texture2d::Texture2d,
    datavec: &[u8],
) {
    use glium::Surface;

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
        glium::uniforms::MagnifySamplerFilter::Nearest,
    );
    target.finish().unwrap();
}

fn run_game(sender: SyncSender<Vec<u8>>, receiver: Receiver<KeyboardEvent>) {
    Game::new(sender, receiver).boot();
}

fn timer_periodic(delay: Arc<AtomicU64>) -> Receiver<()> {
    let (tx, rx) = std::sync::mpsc::sync_channel(1);
    std::thread::spawn(move || loop {
        let micros = delay.load(std::sync::atomic::Ordering::Relaxed);
        std::thread::sleep(std::time::Duration::from_micros(micros));
        if tx.send(()).is_err() {
            break;
        }
    });
    rx
}

fn set_window_size(window: &glium::glutin::window::Window) {
    use glium::glutin::dpi::{LogicalSize, PhysicalSize};

    let dpi = window.scale_factor();

    let physical_size = PhysicalSize::<u32>::from((
        rustic_yellow::SCREEN_W as u32,
        rustic_yellow::SCREEN_H as u32,
    ));
    let logical_size = LogicalSize::<u32>::from_physical(physical_size, dpi);

    window.set_inner_size(logical_size);
}
