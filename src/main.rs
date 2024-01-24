mod dst;
mod model;
mod renderer;

use std::{env, fs::File, io::Read, path::Path, sync::Arc};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

async fn get_wgpu_device(gpu: &wgpu::Instance, surface: &wgpu::Surface<'_>) -> (wgpu::Adapter, wgpu::Device, wgpu::Queue) {
    log::info!("Available adapters:");
    //for a in gpu.enumerate_adapters(wgpu::Backends::all()) {
    //    log::info!("{}    {:?}", a.is_surface_supported(surface), a.get_info())
    //}
    let adapter = gpu.request_adapter(&wgpu::RequestAdapterOptions{
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(surface),
            force_fallback_adapter: false,
        })
        .await
        .expect("Failed to find an appropriate adapter");

    //log::info!("Selected adapter: {:?}", adapter.get_info());

    let (dev, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
            required_limits: wgpu::Limits::default(),
        }, None)
        .await
        .expect("Failed to create device");
    (adapter, dev, queue)
}

fn surface_srgb_format(adapter: &wgpu::Adapter, surface: &wgpu::Surface) -> wgpu::TextureFormat {
    let surface_caps = surface.get_capabilities(&adapter);
    surface_caps.formats.iter().copied()
        .filter(|f| f.is_srgb())
        .next().unwrap_or(surface_caps.formats[0])
}

fn load_design() -> Vec<model::EmbOp> {
    let args: Box<[String]> = env::args().collect();
    if args.len() < 2 {
        panic!("No file specified.");
    }
    let path = Path::new(&args[1]);
    let mut file = File::open(&path).expect("unable to open file");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("unable to read file");
    dst::decode_dst(&buf, ROYGBIV)
}

fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Info).init();
    log::info!("starting up");
    let gpu = wgpu::Instance::default();
    let event_loop = EventLoop::new().unwrap();
    let window = winit::window::WindowBuilder::new().build(&event_loop).unwrap();
    let surface = gpu.create_surface(&window).unwrap();

    let (adapter, device, queue) = pollster::block_on(get_wgpu_device(&gpu, &surface));
    let surface_format = surface_srgb_format(&adapter, &surface);

    let mut size = window.inner_size();
    size.width = size.width.max(1);
    size.height = size.height.max(1);
    let mut surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        ..surface.get_default_config(&adapter, size.width, size.height).unwrap()
    };
    surface.configure(&device, &surface_config);

    let mut display = renderer::EmbDisplay::new(&device, surface_format);

    let window = &window;
    event_loop
        .run(move |event, target| {
            // Have the closure take ownership of the resources.
            // `event_loop.run` never returns, therefore we must do this to ensure
            // the resources are properly cleaned up.
            let _ = (&gpu, &adapter, &display);

            if let Event::WindowEvent {
                window_id: _,
                event,
            } = event
            {
                match event {
                    WindowEvent::Resized(new_size) => {
                        // Reconfigure the surface with the new size
                        surface_config.width = new_size.width.max(1);
                        surface_config.height = new_size.height.max(1);
                        surface.configure(&device, &surface_config);
                        // On macos the window needs to be redrawn manually after resizing
                        window.request_redraw();
                    }
                    WindowEvent::RedrawRequested => {
                        let frame = surface
                            .get_current_texture()
                            .expect("Failed to acquire next swap chain texture");
                        let view = frame
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());
                        let mut encoder =
                            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: None,
                            });
                        
                        display.render(&mut encoder, &view);

                        queue.submit(Some(encoder.finish()));
                        frame.present();
                    }
                    WindowEvent::CloseRequested => target.exit(),
                    _ => {}
                };
            }
        })
        .unwrap();
}

const ROYGBIV: &[[f32;3]]  = &[[1.0, 0.0, 0.0]];
