# 🛡️ Emergency Backup

**Emergency Backup** is an application designed to create a backup of a selected folder to a USB device—even in emergency situations where the monitor is unavailable. The backup is triggered through a specific gesture drawn with the mouse, ensuring an intuitive and accessible interface.

## Main Features

- **Initial Setup**: When launching the application (`emergency_backup.exe`), a configuration window allows you to:
  - Select the folder to be backed up
  - Optionally specify which file extensions to include
  - Save the settings for future use

- **Runs in Background**: After setup, the application continues running in the background. The configuration window cannot be closed using the standard "X" button, ensuring the app remains active and ready.

- **Gesture-Based Activation**: The backup is triggered by drawing a rectangle with the mouse along the four edges of the screen in fullscreen mode, starting from the top-left corner. This gesture simulates an emergency command to start the backup without needing a functional display.

## 🛠️ System Requirements

- Operating System: Windows
- Input Device: Mouse
- Storage Device: USB drive

## Getting Started

1. Clone the repository:
  ```
   git clone https://github.com/Tiazzo/emergency-backup.git
   ```
2. Enter the project directory:
3. Compile the project (if needed) or directly run emergency_backup.exe.
4. Follow the instructions in the configuration window to set your backup folder and optional file extensions.

## Project Structure
- src/: Application source code

- config.json: Saved configuration file

- report.pdf: Detailed project documentation

- Cargo.toml: Rust project configuration