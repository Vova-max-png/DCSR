use clap::Parser;
mod args;
use args::*;
use wry::{
    application::{
      event::{Event, StartCause, WindowEvent},
      event_loop::{ControlFlow, EventLoop},
      window::WindowBuilder, dpi::{PhysicalSize, PhysicalPosition},
    },
    webview::WebViewBuilder,
  };

fn main() -> wry::Result<()> {
    let args = Args::parse();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
      .with_title("Docs reader")
      .with_inner_size(PhysicalSize{width: 1200, height: 900})
      .with_position(PhysicalPosition{x: 0, y: 0})
      .build(&event_loop)?;
    let _webview = WebViewBuilder::new(window)?
      .with_url(args.url.as_str())?
      .build()?;
  
    event_loop.run(move |event, _, control_flow| {
      *control_flow = ControlFlow::Wait;
  
      match event {
        Event::WindowEvent {
          event: WindowEvent::CloseRequested,
          ..
        } => *control_flow = ControlFlow::Exit,
        _ => (),
      }
    });
  }