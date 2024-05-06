use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId},
};

/// The main application object.
#[derive(Default)]
pub struct Application {
    main_window: Option<Window>,
}

impl Application {
    /// Returns the attributes for the main window.
    fn get_main_window_attributes() -> WindowAttributes {
        WindowAttributes::default()
            .with_title("Voxel Engine")
            .with_inner_size(LogicalSize::new(1280, 720))
    }

    /// Handles the events for the main window.
    ///
    /// Note: This function assumes that the main window is Some.
    fn handle_main_window_event(&mut self, event: WindowEvent, event_loop: &ActiveEventLoop) {
        // If this function is called that means the main window is definetly Some.
        let main_window = self.main_window.as_ref().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                main_window.request_redraw();
            }

            _ => {}
        }
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = match event_loop.create_window(Application::get_main_window_attributes()) {
            Ok(win) => win,
            Err(e) => {
                log::error!("Failed to create window: {e}");
                return;
            }
        };

        self.main_window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if self
            .main_window
            .as_ref()
            .map_or(false, |w| w.id() == window_id)
        {
            self.handle_main_window_event(event, event_loop);
        }
    }
}
