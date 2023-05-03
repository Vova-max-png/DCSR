use clap::Parser;
mod args;
use args::*;
use wry::{
    application::{
      event::{Event, StartCause, WindowEvent},
      event_loop::{ControlFlow, EventLoop},
      window::{WindowBuilder}, dpi::{PhysicalPosition, LogicalSize},
      window::Fullscreen,
      window::Theme,
    },
    webview::WebViewBuilder,
  };

fn main() -> wry::Result<()> {
    let args = Args::parse();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
      .with_title("Docs reader")
      .with_position(PhysicalPosition{x: 0, y: 0})
      .with_fullscreen(Some(Fullscreen::Borderless(None)))
      .with_theme(Some(Theme::Dark))
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
      }});
}