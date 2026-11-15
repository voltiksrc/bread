use x11rb::rust_connection::RustConnection;

pub struct DragState {
    pub window: u32,
    pub start_x: i16,
    pub start_y: i16,
    pub window_x: i32,
    pub window_y: i32,
}

pub struct WindowManager {
    pub(crate) conn: RustConnection,
    pub(crate) root: u32,
    pub(crate) windows: Vec<u32>,
    pub(crate) focused_window: Option<u32>,
    pub(crate) dragging: Option<DragState>,
}
