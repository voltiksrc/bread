use crate::wmh::WindowManager;

use x11rb::connection::Connection;
use x11rb::protocol::xproto::{
    ChangeWindowAttributesAux, ConfigureRequestEvent, ConfigureWindowAux, ConnectionExt, InputFocus,
};

const BORDER_WIDTH: u32 = 3;
const BORDER_UNFOCUSED: u32 = 0x444444; // Dark gray

impl WindowManager {
    pub(crate) fn manage_window(&mut self, window: u32) -> Result<(), Box<dyn std::error::Error>> {
        println!("Window wants to open: {window}");

        if !self.windows.contains(&window) {
            self.windows.push(window);
        }

        self.conn.configure_window(
            window,
            &ConfigureWindowAux::new()
                .border_width(BORDER_WIDTH)
                .x(50 + (self.windows.len() as i32 * 40))
                .y(50 + (self.windows.len() as i32 * 40)),
        )?;

        self.conn.change_window_attributes(
            window,
            &ChangeWindowAttributesAux::new().border_pixel(BORDER_UNFOCUSED),
        )?;

        self.grab_focus_click(window)?;

        self.conn.map_window(window)?;
        self.conn.flush()?;

        self.focus_window(window, x11rb::CURRENT_TIME)?;

        println!("Opened window: {window}");

        Ok(())
    }

    pub(crate) fn remove_window(&mut self, window: u32) -> Result<(), Box<dyn std::error::Error>> {
        // Remove the window from our windows vector.
        self.windows.retain(|&id| id != window);

        // Clear the focus of the destroyed window if it was focused, otherwise
        // bread could try change the border of an already destroyed window.
        if self.focused_window == Some(window) {
            self.focused_window = None;

            if let Some(&next) = self.windows.last() {
                self.focus_window(next, x11rb::CURRENT_TIME)?;
            } else {
                self.conn.set_input_focus(
                    InputFocus::POINTER_ROOT,
                    self.root,
                    x11rb::CURRENT_TIME,
                )?;
            }
        }

        Ok(())
    }

    pub(crate) fn configure_request(
        &self,
        event: &ConfigureRequestEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Handle configuration requests which determine what a window
        // wants to look like and be sized at.
        println!("Window {} requested configuration", event.window);

        let mut aux = ConfigureWindowAux::from_configure_request(event);

        // Make sure read controls border widths, not the client
        if self.windows.contains(&event.window) {
            aux = aux.border_width(BORDER_WIDTH);
        }

        self.conn.configure_window(event.window, &aux)?;
        self.conn.flush()?;

        Ok(())
    }
}
