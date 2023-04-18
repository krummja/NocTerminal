#![allow(unused)]

// Dependencies
extern crate wgpu;
extern crate env_logger;

use std::borrow::Cow;

use winit::{
    event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use wgpu::Surface;

// Library Internal
mod atlas;
mod point;
mod rectangle;
mod size;
mod state;
mod terminal;
mod texture;
mod vertex;
mod shape;
mod tileset;

use state::State;


pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut terminal = State::new(window).await;

    event_loop.run(move | event, _, control_flow | {
        match event {

            Event::WindowEvent {
                window_id,
                ref event,
            } if window_id == terminal.window().id() => {
                if !terminal.input(event) {
                    match event {

                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input: KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => {
                            *control_flow = ControlFlow::Exit;
                        }

                        WindowEvent::Resized(physical_size) => {
                            terminal.resize(*physical_size);
                        }

                        WindowEvent::ScaleFactorChanged {
                            new_inner_size,
                            ..
                        } => {
                            terminal.resize(**new_inner_size);
                        }

                        _ => {}

                    }
                }
            }

            Event::RedrawRequested(
                window_id,
            ) if window_id == terminal.window().id() => {
                terminal.update();
                match terminal.render() {

                    Ok(_) => {}

                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => {
                        terminal.resize(terminal.size);
                    }

                    // The system is out of memory, we should probably quit...
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        *control_flow = ControlFlow::Exit;
                    }

                    // All other errors (Outdated, Timeout) should be resolved
                    // by the next frame
                    Err(e) => eprintln!("{:?}", e),

                }
            }

            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once unless we manually
                // request it
                terminal.window().request_redraw();
            }

            _ => {}
        }
    });
}
