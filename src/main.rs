use std::thread;
use emergency_backup::event_manager;
use emergency_backup::gui;
fn main() {
    gui::start_gui();

    let events =  thread::spawn(|| {
        event_manager::manage_events();

        loop {
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    events.join().unwrap();
    
}
