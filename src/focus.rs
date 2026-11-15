use crate::wmh::WindowManager;

use x11rb::connection::Connection;
use x11rb::protocol::xproto::{
    ButtonIndex, ChangeWindowAttributesAux, ConfigureWindowAux, ConnectionExt, EventMask, GrabMode,
    InputFocus, ModMask, StackMode,
};

const BORDER_FOCUSED: u32 = 0xFFFFFF; // White
const BORDER_UNFOCUSED: u32 = 0x444444; // Dark gray

impl WindowManager {
    pub(crate) fn grab_focus_click(&self, window: u32) -> Result<(), Box<dyn std::error::Error>> {
        self.conn
            .grab_button(
                false,
                window,
                EventMask::BUTTON_PRESS,
                GrabMode::ASYNC,
                GrabMode::ASYNC,
                x11rb::NONE,
                x11rb::NONE,
                ButtonIndex::ANY,
                ModMask::ANY,
            )?
            .check()?;

        Ok(())
    }

    pub(crate) fn focus_window(
        &mut self,
        window: u32,
        time: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.focused_window == Some(window) {
            return Ok(());
        }

        if let Some(previous) = self.focused_window {
            // If theres a previous window, set it to the unfocused border.
            if previous != window {
                self.conn.change_window_attributes(
                    previous,
                    &ChangeWindowAttributesAux::new().border_pixel(BORDER_UNFOCUSED),
                )?;

                self.grab_focus_click(previous)?;
            }
        }

        // Change the clicked window's border to our focused border color
        self.conn
            .change_window_attributes(
                window,
                &ChangeWindowAttributesAux::new().border_pixel(BORDER_FOCUSED),
            )?
            .check()?;

        println!("Set window {window} border to white");

        // After setting the border we remove a passive grab, which allows the newly
        // focused application to receive ordinary clicks without bread intercepting them.
        self.conn
            .ungrab_button(ButtonIndex::ANY, window, ModMask::ANY)?
            .check()?;

        // Tell x11 to focus the clicked window
        self.conn
            .set_input_focus(InputFocus::PARENT, window, time)?
            .check()?;

        // Put the selected window above other windows
        self.conn.configure_window(
            window,
            &ConfigureWindowAux::new().stack_mode(StackMode::ABOVE),
        )?;

        // Update focused_window
        self.focused_window = Some(window);
        println!("Focused window: {:?}", self.focused_window);

        self.conn.flush()?;

        Ok(())
    }
}
