use crate::event::*;
use crate::{EventBuffer, EventStream, Settings, Window, WindowContents};
use futures_executor::LocalPool;
use futures_util::task::LocalSpawnExt;
use std::cell::RefCell;
use std::future::Future;
use std::sync::Arc;
use winit::event::Event as WinitEvent;
use winit::event_loop::{ControlFlow, EventLoop};

/// The entry point for a blinds-based application
///
/// `run` acts as the executor for your async application, and it handles your event loop on both
/// desktop and web. It is a single-threaded executor, because wasm doesn't support multithreading
/// at the moment.
///
/// Currently blinds only supports one window, and `settings` determines how it will be
/// constructed.
pub fn run<F, T>(settings: Settings, app: F) -> !
where
    T: 'static + Future<Output = ()>,
    F: 'static + FnOnce(Window, EventStream) -> T,
{
    let stream = EventStream::new();
    let buffer = stream.buffer();

    let event_loop = EventLoop::new();
    let window = Arc::new(WindowContents::new(&event_loop, settings));
    let pool = LocalPool::new();
    pool.spawner()
        .spawn_local(app(Window(window.clone()), stream))
        .expect("Failed to start application");

    do_run(event_loop, window, pool, buffer)
}

//#[cfg(feature = "gl")]
use glow::Context;

//#[cfg(feature = "gl")]
/// The entry point for a blinds-based application using OpenGL
///
/// `run_gl` acts the same as [`run`] except it provides a [`glow`] context
///
/// [`run`]: run
/// [`glow`]: glow
pub fn run_gl<T, F>(settings: Settings, app: F) -> !
where
    T: 'static + Future<Output = ()>,
    F: 'static + FnOnce(Window, Context, EventStream) -> T,
{
    let stream = EventStream::new();
    let buffer = stream.buffer();

    let event_loop = EventLoop::new();
    let (window, ctx) = WindowContents::new_gl(&event_loop, settings);
    let window = Arc::new(window);
    let pool = LocalPool::new();
    pool.spawner()
        .spawn_local(app(Window(window.clone()), ctx, stream))
        .expect("Failed to start application");

    do_run(event_loop, window, pool, buffer)
}

fn do_run(
    event_loop: EventLoop<()>,
    window: Arc<WindowContents>,
    mut pool: LocalPool,
    buffer: Arc<RefCell<EventBuffer>>,
) -> ! {
    #[cfg(feature = "gilrs")]
    let mut gilrs = gilrs::Gilrs::new();

    let mut finished = pool.try_run_one();

    event_loop.run(move |event, _, ctrl| {
        match event {
            WinitEvent::NewEvents(winit::event::StartCause::Init) => {
                *ctrl = ControlFlow::Poll;
            }
            WinitEvent::WindowEvent { event, .. } => {
                if let winit::event::WindowEvent::CloseRequested = &event {
                    *ctrl = ControlFlow::Exit;
                }
                if let winit::event::WindowEvent::Resized(size) = &event {
                    window.resize(*size);
                }
                if let Some(event) = window_event(event, &window) {
                    buffer.borrow_mut().push(event);
                }
            }
            WinitEvent::LoopDestroyed | WinitEvent::MainEventsCleared => {
                buffer.borrow_mut().mark_ready();
                #[cfg(feature = "gilrs")]
                process_gilrs_events(&mut gilrs, &buffer);
                finished = pool.try_run_one();
            }
            _ => (),
        }
        if finished {
            *ctrl = ControlFlow::Exit;
        }
    })
}

#[cfg(feature = "gilrs")]
fn process_gilrs_events(
    gilrs: &mut Result<gilrs::Gilrs, gilrs::Error>,
    buffer: &Arc<RefCell<EventBuffer>>,
) {
    if let Ok(gilrs) = gilrs.as_mut() {
        while let Some(ev) = gilrs.next_event() {
            if let Some(ev) = gamepad_event(ev) {
                buffer.borrow_mut().push(ev);
            }
        }
    }
}
