mod focus;
mod window;
mod wm;
mod wmh;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    wm::run()
}
