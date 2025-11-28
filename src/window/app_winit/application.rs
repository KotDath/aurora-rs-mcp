use std::marker::PhantomData;
use std::rc::Rc;

use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, Size};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

use crate::AppInfo;

pub fn run(event_loop: EventLoop<()>, mut app: impl ApplicationHandler<()> + 'static) {
    event_loop.run_app(&mut app).unwrap();
}

pub fn make_window(
    app_info: &AppInfo,
    elwt: &ActiveEventLoop,
    f: impl FnOnce(WindowAttributes) -> WindowAttributes,
) -> Rc<Window> {
    let mut attributes = f(WindowAttributes::default());
    let size = LogicalSize::new(720f64, 1600f64);
    attributes.title = app_info.title.clone();
    attributes.max_inner_size = Some(Size::Logical(size));
    let window = elwt.create_window(attributes);
    Rc::new(window.unwrap())
}

pub struct WinitApp<T, S, Init, InitSurface, Handler> {
    init: Init,
    init_surface: InitSurface,
    event: Handler,
    state: Option<T>,
    surface_state: Option<S>,
}

pub struct WinitAppBuilder<T, S, Init, InitSurface> {
    init: Init,
    init_surface: InitSurface,
    marker: PhantomData<(Option<T>, Option<S>)>,
}

impl<T, S, Init, InitSurface> WinitAppBuilder<T, S, Init, InitSurface>
where
    Init: FnMut(&ActiveEventLoop) -> T,
    InitSurface: FnMut(&ActiveEventLoop, &mut T) -> S,
{
    pub fn with_init(init: Init, init_surface: InitSurface) -> Self {
        Self {
            init,
            init_surface,
            marker: PhantomData,
        }
    }
    pub fn with_event_handler<F>(self, handler: F) -> WinitApp<T, S, Init, InitSurface, F>
    where
        F: FnMut(&mut T, Option<&mut S>, Event<()>, &ActiveEventLoop),
    {
        WinitApp::new(self.init, self.init_surface, handler)
    }
}

impl<T, S, Init, InitSurface, Handler> WinitApp<T, S, Init, InitSurface, Handler>
where
    Init: FnMut(&ActiveEventLoop) -> T,
    InitSurface: FnMut(&ActiveEventLoop, &mut T) -> S,
    Handler: FnMut(&mut T, Option<&mut S>, Event<()>, &ActiveEventLoop),
{
    pub fn new(init: Init, init_surface: InitSurface, event: Handler) -> Self {
        Self {
            init,
            init_surface,
            event,
            state: None,
            surface_state: None,
        }
    }
}

impl<T, S, Init, InitSurface, Handler> ApplicationHandler
    for WinitApp<T, S, Init, InitSurface, Handler>
where
    Init: FnMut(&ActiveEventLoop) -> T,
    InitSurface: FnMut(&ActiveEventLoop, &mut T) -> S,
    Handler: FnMut(&mut T, Option<&mut S>, Event<()>, &ActiveEventLoop),
{
    fn resumed(&mut self, el: &ActiveEventLoop) {
        let mut state = (self.init)(el);
        self.surface_state = Some((self.init_surface)(el, &mut state));
        self.state = Some(state);
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        let surface_state = self.surface_state.take();
        drop(surface_state);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let state = self.state.as_mut().unwrap();
        let surface_state = self.surface_state.as_mut();
        (self.event)(
            state,
            surface_state,
            Event::WindowEvent { window_id, event },
            event_loop,
        );
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(state) = self.state.as_mut() {
            (self.event)(
                state,
                self.surface_state.as_mut(),
                Event::AboutToWait,
                event_loop,
            );
        }
    }
}
