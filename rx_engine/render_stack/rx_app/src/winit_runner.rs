use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
use winit::{dpi::Position, monitor::MonitorHandle};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize, Size},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow},
    window::{Fullscreen, Window, WindowAttributes},
};

const INITIAL_WINDOW_MONITOR_FRACTION: f32 = 0.75;

struct RunnerState {
    config: rx_project::ProjectConfig,
    window: Option<Arc<Window>>,
}

impl winit::application::ApplicationHandler for RunnerState {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Poll);
        if self.window.is_none() {
            let attrs = window_attributes(event_loop, Some(&self.config));
            let window = Arc::new(
                event_loop
                    .create_window(attrs)
                    .expect("failed to create winit window"),
            );
            window.set_ime_allowed(true);
            window.set_visible(true);

            self.window = Some(window);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if self
            .window
            .as_ref()
            .is_some_and(|current_window| current_window.id() != window)
        {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn fit_aspect(
    desired: PhysicalSize<u32>,
    max_width: u32,
    max_height: u32,
) -> PhysicalSize<u32> {
    let scale = f32::min(
        max_width as f32 / desired.width as f32,
        max_height as f32 / desired.height as f32,
    );
    let width = ((desired.width as f32) * scale).floor().max(1.0) as u32;
    let height = ((desired.height as f32) * scale).floor().max(1.0) as u32;
    PhysicalSize::new(width, height)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn resolve_initial_window_physical_size(
    desired: PhysicalSize<u32>,
    window_size: rx_runtime::WindowSizeMode,
    monitor: Option<&MonitorHandle>,
) -> PhysicalSize<u32> {
    match window_size {
        rx_runtime::WindowSizeMode::Exact => desired,
        rx_runtime::WindowSizeMode::FitMonitor => {
            let Some(monitor) = monitor else {
                return desired;
            };
            let max_width =
                ((monitor.size().width as f32) * INITIAL_WINDOW_MONITOR_FRACTION).floor() as u32;
            let max_height =
                ((monitor.size().height as f32) * INITIAL_WINDOW_MONITOR_FRACTION).floor() as u32;
            fit_aspect(desired, max_width.max(1), max_height.max(1))
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn center_position(
    monitor: &MonitorHandle,
    window_size: PhysicalSize<u32>,
) -> PhysicalPosition<i32> {
    let monitor_pos = monitor.position();
    let monitor_size = monitor.size();

    let x = monitor_pos.x + ((monitor_size.width as i32 - window_size.width as i32) / 2);
    let y = monitor_pos.y + ((monitor_size.height as i32 - window_size.height as i32) / 2);
    PhysicalPosition::new(x, y)
}

#[cfg(not(target_arch = "wasm32"))]
fn pick_monitor(event_loop: &ActiveEventLoop) -> Option<MonitorHandle> {
    event_loop
        .primary_monitor()
        .or_else(|| event_loop.available_monitors().next())
}

#[cfg(target_arch = "wasm32")]
fn pick_monitor(_event_loop: &ActiveEventLoop) -> Option<MonitorHandle> {
    None
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_window_mode_override() -> Option<String> {
    std::env::var("RX_WINDOW_MODE")
        .ok()
        .map(|raw| raw.trim().to_ascii_lowercase())
}

fn window_attributes(
    event_loop: &ActiveEventLoop,
    project_config: Option<&rx_project::ProjectConfig>,
) -> WindowAttributes {
    #[cfg(target_arch = "wasm32")]
    let _ = event_loop;

    let title = project_config
        .map(|config| config.name.clone())
        .unwrap_or_else(|| "Rx".to_string());

    let mut attrs = WindowAttributes::default()
        .with_title(title)
        .with_visible(false);
    #[cfg(target_arch = "wasm32")]
    {
        attrs = attrs.with_append(true);
    }
    let Some(config) = project_config else {
        return attrs;
    };

    let desired = PhysicalSize::new(config.virtual_width, config.virtual_height);
    if desired.width == 0 || desired.height == 0 {
        return attrs;
    }

    let window_size_mode = config.window_size;

    #[cfg(target_arch = "wasm32")]
    {
        let physical = match window_size_mode {
            rx_runtime::WindowSizeMode::Exact => desired,
            rx_runtime::WindowSizeMode::FitMonitor => browser_viewport_size().unwrap_or(desired),
        };
        return attrs.with_inner_size(Size::Physical(physical));
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let Some(monitor) = pick_monitor(event_loop) else {
            let mut attrs = attrs.with_inner_size(Size::Physical(desired));
            if window_size_mode == rx_runtime::WindowSizeMode::Exact {
                attrs = attrs
                    .with_resizable(false)
                    .with_min_inner_size(Size::Physical(desired))
                    .with_max_inner_size(Size::Physical(desired));
            }
            return attrs;
        };
        if let Some(mode) = parse_window_mode_override()
            && (mode == "borderless" || mode == "borderless_fullscreen")
        {
            return attrs.with_fullscreen(Some(Fullscreen::Borderless(Some(monitor))));
        }

        let fitted =
            resolve_initial_window_physical_size(desired, window_size_mode, Some(&monitor));
        let centered = center_position(&monitor, fitted);

        attrs = attrs.with_inner_size(Size::Physical(fitted));
        if window_size_mode == rx_runtime::WindowSizeMode::Exact {
            attrs = attrs
                .with_resizable(false)
                .with_min_inner_size(Size::Physical(fitted))
                .with_max_inner_size(Size::Physical(fitted));
        }
        attrs.with_position(Position::Physical(centered))
    }
}
