# MotorStorm Pacific Rift Discord RPC (Standalone)

![Version](https://img.shields.io/badge/version-0.2.1-blue) ![Platform](https://img.shields.io/badge/platform-Windows-blue) ![License](https://img.shields.io/badge/license-MIT-green)

A high-performance, lightweight, and standalone Rust application designed to integrate **MotorStorm: Pacific Rift** (via the RPCS3 emulator) with **Discord Rich Presence**.

This tool automatically detects when you are playing MotorStorm and updates your Discord status to show "Playing MotorStorm®: Pacific Rift", complete with the elapsed time and game logo.

---

## 🚀 Key Features

*   **Automatic Detection**
    *   Continuously monitors for the **RPCS3** process.
    *   Intelligently scans window titles to detect when **MotorStorm: Pacific Rift** is actively running.
    *   No manual configuration required.

*   **Discord Rich Presence**
    *   Displays the game logo and title on your Discord profile.
    *   Shows the "Elapsed Time" to let friends know how long you've been racing.
    *   Uses a persistent connection that reconnects automatically if Discord is restarted.

*   **Modern Terminal Interface (TUI)**
    *   Features a beautiful, retro-styled terminal dashboard.
    *   **Live Status**: See real-time connection status to Discord and the game.
    *   **Resource Monitoring**: Tracks the application's own CPU and RAM usage (extremely lightweight!).
    *   **Activity Logs**: View a scrollable history of connection events and detections.

*   **Smart & Robust**
    *   **Auto-Elevation**: Automatically requests Administrator privileges if needed to detect the emulator (crucial for RPCS3).
    *   **Zero-Config**: Works out of the box.

---

## 🎮 How to Use

1.  **Download** the latest release `MotorStormRPC.exe`.
2.  **Launch** the application.
    *   *Note: If prompted by Windows User Account Control (UAC), click **Yes**. This is required to read the emulator's window title.*
3.  **Start RPCS3** and launch **MotorStorm: Pacific Rift**.
4.  That's it! Check your Discord profile to see the status.

---

## ⌨ Controls

The application window accepts the following keyboard shortcuts:

| Key | Action |
| :--- | :--- |
| **Q** / **Esc** | **Quit** the application safely. |
| **D** | Toggle **Debug Mode** (View detailed scan logs). |
| **C** | **Clear** the log history. |

---

## ❓ Troubleshooting

### "Status Not Detected"
*   Ensure you allowed the application to run as **Administrator** when prompted.
*   Make sure the game window title contains "MotorStorm" or "Pacific Rift".

### "Discord Connection Failed"
*   Ensure your Discord desktop application is open.
*   Go to **User Settings -> Activity Privacy** and ensure **"Share your detected activities with others"** is turned **ON**.

### "Application closes immediately"
*   The app mimics a console window. If it closes instantly, it might be encountering an error. Try running it from PowerShell to see the output.

---

## 🛠 Building from Source

If you want to modify or build the project yourself, you need the [Rust Toolchain](https://rustup.rs/).

1.  **Clone the repository:**
    ```powershell
    git clone https://github.com/ZoniBoy00/MotorStormRPC.git
    cd MotorStormRPC
    ```

2.  **Build using Cargo:**
    ```powershell
    cargo build --release
    ```

3.  **Run:**
    The binary will be located in `target/release/MotorStormRPC.exe`.
    ```powershell
    ./target/release/MotorStormRPC.exe
    ```

---

## 📄 License
MIT License. See `LICENSE` for more details.
