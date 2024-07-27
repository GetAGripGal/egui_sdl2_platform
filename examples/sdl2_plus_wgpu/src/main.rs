//! A simple example of how to create an sdl window with wgpu context and drawing en egui window ontop of it
use std::time::Instant;

use egui_sdl2_platform::sdl2;
use sdl2::event::{Event, WindowEvent};

/// Runs the demo app
async fn run() -> anyhow::Result<()> {
    // Initialize sdl
    let sdl = sdl2::init().map_err(|e| anyhow::anyhow!("Failed to create sdl context: {}", e))?;
    // Create the video subsystem
    let mut video = sdl
        .video()
        .map_err(|e| anyhow::anyhow!("Failed to initialize sdl video subsystem: {}", e))?;
    // Create the sdl window
    let window = video
        .window("With love <3 - GetAGripGal", 1280, 720)
        .position_centered()
        .resizable()
        .build()?;
    // Get the sdl event pump
    let mut event_pump = sdl
        .event_pump()
        .map_err(|e| anyhow::anyhow!("Failed to get sdl event pump: {}", e))?;

    // Create the wgpu instance
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    // Create the wgpu surface
    let surface = unsafe {
        instance.create_surface_unsafe(wgpu::SurfaceTargetUnsafe::from_window(&window)?)?
    };

    // Request the adapter
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .ok_or(anyhow::anyhow!("Failed to request the adapter."))?;

    // Request the device and queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                ..Default::default()
            },
            None,
        )
        .await?;

    // Get surface format
    let surface_format = surface.get_capabilities(&adapter).formats[0];
    // Configure the surface
    let mut surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_DST,
        format: surface_format,
        view_formats: vec![wgpu::TextureFormat::Bgra8UnormSrgb],
        alpha_mode: wgpu::CompositeAlphaMode::Opaque,
        width: window.size().0,
        height: window.size().1,
        present_mode: wgpu::PresentMode::AutoVsync,
        desired_maximum_frame_latency: 1,
    };
    surface.configure(&device, &surface_config);

    // Create the egui + sdl2 platform
    let mut platform = egui_sdl2_platform::Platform::new(window.size())?;
    // Create the egui render pass
    let mut egui_pass = egui_wgpu_backend::RenderPass::new(&device, surface_format, 1);

    // The clear color
    let mut color = [0.0, 0.0, 0.0, 1.0];
    // The textedit text
    let mut text = String::new();

    // Get the time before the loop started
    let start_time = Instant::now();

    // The main loop
    'main: loop {
        // Update the time
        platform.update_time(start_time.elapsed().as_secs_f64());

        // Get the egui context and begin drawing the frame
        let ctx = platform.context();
        // Draw an egui window
        egui::Window::new("Hello, world!").show(&ctx, |ui| {
            ui.label("Hello, world!");
            if ui.button("Greet").clicked() {
                println!("Hello, world!");
            }
            ui.horizontal(|ui| {
                ui.label("Color: ");
                ui.color_edit_button_rgba_premultiplied(&mut color);
            });
            ui.code_editor(&mut text);
        });

        // Get the output texture
        let output = surface.get_current_texture()?;
        // Get the output view
        let view = output.texture.create_view(&Default::default());

        // Stop drawing the egui frame and get the full output
        let full_output = platform.end_frame(&mut video)?;
        // Get the paint jobs
        let paint_jobs = platform.tessellate(&full_output);

        // Create the command encoder
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Main Command Encoder"),
        });

        // Clear the screen
        {
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: color[0] as f64,
                            g: color[1] as f64,
                            b: color[2] as f64,
                            a: color[3] as f64,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                label: None,
                ..Default::default()
            });
        }

        // Upload all the resources to the egui render pass
        let screen_descriptor = egui_wgpu_backend::ScreenDescriptor {
            physical_width: surface_config.width,
            physical_height: surface_config.height,
            // The sdl scale factor remains constant, so we shall set it to 1
            scale_factor: 1.0,
        };
        // Add the textures to the egui render pass
        let tdelta = full_output.textures_delta;
        egui_pass.add_textures(&device, &queue, &tdelta)?;
        egui_pass.update_buffers(&device, &queue, &paint_jobs, &screen_descriptor);

        // Execute the egui render pass
        egui_pass.execute(&mut encoder, &view, &paint_jobs, &screen_descriptor, None)?;

        // Submit the command encoder
        queue.submit([encoder.finish()]);
        // Present the output
        output.present();

        // Remove the textures from the render pass
        egui_pass.remove_textures(tdelta)?;

        // Handle sdl events
        for event in event_pump.poll_iter() {
            // Handle sdl events
            match event {
                Event::Window {
                    window_id,
                    win_event,
                    ..
                } if window_id == window.id() => match win_event {
                    WindowEvent::Close => break 'main,
                    WindowEvent::SizeChanged(w, h) => {
                        if w > 0 && h > 0 {
                            surface_config.width = w as u32;
                            surface_config.height = h as u32;
                            surface.configure(&device, &surface_config);
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
            // Let the egui platform handle the event
            platform.handle_event(&event, &sdl, &video);
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    pollster::block_on(run())?;
    Ok(())
}
