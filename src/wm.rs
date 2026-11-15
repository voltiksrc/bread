use crate::wmh::WindowManager;

use x11rb::connection::Connection;
use x11rb::protocol::Event;
use x11rb::protocol::xproto::{ChangeWindowAttributesAux, ConnectionExt, EventMask};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = x11rb::connect(None)?;

    let screen = &conn.setup().roots[screen_num];
    let root = screen.root;

    println!("Bread connected to X11!");
    println!("Root window: {}", root);
    println!(
        "Resolution: {}x{}..",
        screen.width_in_pixels, screen.height_in_pixels
    );

    // Redirect x11 requests to bread so we can handle and manage windows.
    conn.change_window_attributes(
        root,
        &ChangeWindowAttributesAux::new()
            .event_mask(EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY),
    )?
    .check()?;

    println!("Bread claimed the root window.");

    // Window and focus tracking variables
    let mut wm = WindowManager {
        conn,
        root,
        windows: Vec::new(),
        focused_window: None,
        dragging: None,
    };

    // Main window event loop
    loop {
        let event = wm.conn.wait_for_event()?;

        match event {
            Event::MapRequest(event) => {
                wm.manage_window(event.window)?;
            }

            Event::DestroyNotify(event) => {
                println!("Window destroyed: {}", event.window);
                wm.remove_window(event.window)?;
            }

            Event::UnmapNotify(event) => {
                // A window can be hidden without being destroyed.
                if event.event == wm.root && wm.windows.contains(&event.window) {
                    wm.remove_window(event.window)?;
                }
            }

            Event::ConfigureRequest(event) => {
                wm.configure_request(&event)?;
            }

            Event::ButtonPress(event) => {
                println!("Window {} was clicked", event.event);

                if wm.windows.contains(&event.event) {
                    // Focus the clicked window using our helper function
                    wm.focus_window(event.event, event.time)?;
                }
            }

            _ => {}
        }
    }
}
