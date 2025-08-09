# 🎵 Termi Music Player

A lightweight, terminal-based music player written in **Rust**, built purely for fun and exploration.  
It supports **playing music from files or directories**, basic playback controls, and a visual timeline — all from your terminal!  

> ⚡ Hobby project — experimenting with "mad" ideas and pushing Rust to do cool things in the CLI space.

---

## ✨ Features

- 📂 **Open File or Directory** — play a single track or an entire folder of music.
- ▶️ **Play / Pause** — toggle playback easily.
- ⏭ **Next / Previous** — jump between tracks.
- 🔊 **Volume Control** — increase or decrease volume.
- 📊 **Visual Timeline** — shows playback progress in real-time.
- 🎼 **Track Info** — display current track name, elapsed time, and total time.

---

<!-- 
## 📸 Preview (Concept)
```
```
-->

---

## 🛠 Installation

### 1️⃣ Clone the Repository
```bash
git clone https://github.com/mahikoishor/termi-music-player.git
cd termi-music-player
```

### 2️⃣ Build the Project
Make sure you have **Rust** installed ([Install Rust](https://www.rust-lang.org/tools/install)).

```bash
cargo build --release
```

### 3️⃣ Run the Player
```bash
cargo run
```
- You can pass either:
  - A single file (`.mp3`, `.wav`, `.flac`, etc.)
  - A directory containing multiple tracks

---

## 🎮 Controls

| Key         | Action          |
|-------------|-----------------|
| `Space`     | Play/Pause    |
| `<`         | Next track      |
| `>`         | Previous track  |
| `↑↓`        | Volume up/down  |
| `o`         | Open Folder/File|
| `ESC`       | Quit player     |

---

## 💡 Future Ideas
- 🎶 Shuffle & repeat modes
- 🎨 Theming for different terminal styles
- 📜 Playlist management
- 📡 Streaming from online sources

---

## 📝 License
This project is licensed under the GPL-2.0 License.  
Feel free to fork, hack, and play around with it!

---

**Author:** [Mahi Koishor](https://github.com/mahikoishor)  
Just a guy exploring weird terminal app ideas with Rust. 🚀
