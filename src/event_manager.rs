use rdev::{listen, Event, EventType};
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use crate::input::{Tracker, Coordinate, Status};
use crate::backup::perform_backup;

pub fn play_sound(sound: &str) {    
    // Try to get the default output stream 
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
     // Create a wrapper that controls playback of an audio source
    let sink = Sink::try_new(&stream_handle).unwrap();

    let sound_path = if cfg!(debug_assertions) {
        std::path::Path::new("src/audio").join(sound)
    } else {
        let exe_path = std::env::current_exe().unwrap();
        exe_path.parent().unwrap().join("audio").join(sound)
    };

    let file = BufReader::new(File::open(&sound_path).expect("Audio file not found"));
    let source = Decoder::new(file).expect("Failed to decode audio");
    sink.append(source);
    sink.sleep_until_end();
}


#[derive(Default)]
struct DetectionState {
    activation_detected: bool, // Flag to track if the activation shape has been detected
}

pub fn manage_events() {
        let size = match rdev::display_size() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error retrieving display size: {:?}", e);
            return;
        }
    };

    let tracker = Arc::new(Mutex::new(Tracker::new(size.0 as i32, size.1 as i32)));

    // Shared state for rectangle detection
    let detection_state = Arc::new(Mutex::new(DetectionState::default()));

    let tracker_clone = Arc::clone(&tracker);
    let detection_state_clone = Arc::clone(&detection_state);

    let callback = move |event: Event| {
        if let EventType::MouseMove { x, y } = event.event_type {
            let point = Coordinate {
                x: x.trunc() as i32,
                y: y.trunc() as i32,
            };
            
            let mut tracker = tracker_clone.lock().unwrap();
            let status = tracker.update(point);
            drop(tracker);

            match status {
                Status::ActivationShapeCompleted => handle_activation(&detection_state_clone),
                Status::ConfirmationShapeCompleted => handle_confirmation(&detection_state_clone),
                _ => {}
            }
        }
    };

    if let Err(error) = listen(callback) {
        eprintln!("Error: {:?}", error); // Print error if event listener fails
    }
}

fn handle_activation(detection_state: &Arc<Mutex<DetectionState>>) {
    let mut state = detection_state.lock().unwrap();
    // If the activation shape is not yet detected, process the command
    if !state.activation_detected {
        println!("Activation command detected. Please, draw a horizontal line to start the backup.");
        play_sound("confirmation_command.mp3"); 
        state.activation_detected = true; 
    }
}

// Function to handle the confirmation shape detection and start the backup process
fn handle_confirmation(detection_state: &Arc<Mutex<DetectionState>>) {
    let mut state = detection_state.lock().unwrap();
    if state.activation_detected {
        println!("Backup started...");
        play_sound("backup_started.mp3"); 
        match perform_backup() {
            Ok(_) => {
                println!("Backup completed.");
                play_sound("backup_completed.mp3");
            }
            Err(_e) => {
                println!("Backup failed.");
                play_sound("backup_failed.mp3");
            }
        }
        state.activation_detected = false; // Reset the detection state after backup
    }
}
