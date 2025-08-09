use crate::{constance::AUDIO_FORMATS, engine::AudioEngine, music::TermiMusic};
use std::{fs, path::PathBuf};

#[derive(PartialEq, Eq)]
pub enum PlayerState {
    Empty,
    Loading,
    Ready,
    Play,
    Pause,
}

pub struct TermiPlayer {
    pub engine: AudioEngine,
    pub playlist: Vec<PathBuf>,
    pub current_index: usize,
    pub music: TermiMusic,

    pub state: PlayerState,

    //
    pub status: Option<String>,
}

impl Default for TermiPlayer {
    fn default() -> Self {
        Self {
            engine: AudioEngine::new(100.00),
            playlist: Vec::new(),
            current_index: 0,
            music: TermiMusic::default(),

            state: PlayerState::Empty,

            status: None,
        }
    }
}

impl TermiPlayer {
    pub fn prepare(&mut self) {
        // self.engine.set_volume(self.volume as f32 / 100.0);
    }

    /// Open File
    ///
    /// Opens file/folder and add them to playlist
    pub fn open(&mut self, path: &str) {
        self.state = PlayerState::Loading;
        self.status = Some("Loading...".to_string());

        let path = PathBuf::from(path);

        // TODO: check path.extist()

        if path.is_file() && is_audio_file(&path) {
            self.playlist = vec![path];
        } else if path.is_dir() {
            self.playlist = fs::read_dir(path)
                .expect("open_file: read_dir")
                .flatten()
                .fold(Vec::<PathBuf>::new(), |mut acc, entry| {
                    // TODO: nested dir indexing

                    let entry_path = entry.path();

                    if entry_path.is_file() && is_audio_file(&entry_path) {
                        acc.push(entry_path);
                    }

                    acc
                });
        } else {
            // TODO: check !(file | dir)
            // TODO: show status
            self.playlist = vec![];
        }

        if self.playlist.is_empty() {
            self.state = PlayerState::Empty;
            self.status = Some("Error: Failed to open.".to_string());
            return;
        }

        self.state = PlayerState::Ready;
        self.status = None;

        self.pause();
        
        self.update_music();

        self.play();
    }

    pub fn update_music(&mut self) {
        self.status = Some("Loading Music".to_string());

        if let Some(item) = self.playlist.get(self.current_index) {
            self.music = TermiMusic {
                title: item
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    .to_string(),
            };

            // Load the audio file into the engine
            if let Err(e) = self.engine.load_audio(item) {
                self.status = Some(format!("Error loading audio: {}", e));
                return;
            }
        } else {
            self.music = TermiMusic::default()
        }

        self.status = Some("Loaded Music".to_string());
    }

    pub fn toggle_play(&mut self) {
        if self.state == PlayerState::Play {
            self.pause();
        } else {
            self.play();
        }
    }

    pub fn play(&mut self) {
        self.state = PlayerState::Play;
        self.status = Some("Playing".to_string());
        self.engine.play();
    }

    pub fn pause(&mut self) {
        self.state = PlayerState::Pause;
        self.status = Some("Pause".to_string());
        self.engine.pause();
    }

    pub fn next(&mut self) {
        self.status = Some("Next Music".to_string());

        let mut current_index = self.current_index + 1;

        if current_index >= self.playlist.len() {
            current_index = 0;
        }

        self.current_index = current_index;

        self.update_music();

        self.play();
    }

    pub fn previous(&mut self) {
        self.status = Some("Previous Music".to_string());

        let mut current_index = self.current_index as isize - 1;

        if current_index <= -1 {
            current_index = self.playlist.len() as isize - 1;
        }

        if self.playlist.is_empty() {
            current_index = 0;
        }

        self.current_index = current_index as usize;

        self.update_music();

        self.play();
    }

    pub fn volume_up(&mut self) {
        self.engine.volume_up();
    }

    pub fn volume_down(&mut self) {
        self.engine.volume_down();
    }

    pub fn current_position(&self) -> (u64, u64) {
        self.engine.get_position_seconds()
    }
}

///
///
///
fn is_audio_file(path: &PathBuf) -> bool {
    if let Some(ext) = path.extension() {
        return AUDIO_FORMATS.contains(&ext.to_str().unwrap());
    }

    false
}
