use rodio::{Decoder, OutputStream, Sink, Source};
use std::{
    fs::File,
    io::BufReader,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{Duration, Instant}
};

pub struct AudioEngine {
    _stream: OutputStream,
    sink: Sink,
    position: Arc<Mutex<PositionTracker>>,
    pub volume: f32
}

#[derive(Clone)]
struct PositionTracker {
    start_time: Option<Instant>,
    total_duration: Option<Duration>,
    is_playing: bool,
    paused_time: Duration,
    last_pause_time: Option<Instant>,
}

impl Default for PositionTracker {
    fn default() -> Self {
        Self {
            start_time: None,
            total_duration: None,
            is_playing: false,
            paused_time: Duration::ZERO,
            last_pause_time: None,
        }
    }
}

impl AudioEngine {
    pub fn new(volume: f32) -> Self {
        let (stream, stream_handle) = OutputStream::try_default().expect("Failed to create audio output stream");
        let sink = Sink::try_new(&stream_handle).expect("Failed to create audio sink");

        sink.set_volume(volume);
        
        Self {
            _stream: stream,
            sink,
            position: Arc::new(Mutex::new(PositionTracker::default())),
            volume
        }
    }

    pub fn load_audio(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let source = Decoder::new(BufReader::new(file))?;
        
        // Try to get the total duration before consuming the source
        let total_duration = source.total_duration();
        
        // If we couldn't get duration directly, try with a buffered source
        let buffered_source = source.buffered();
        let duration_from_buffered = if total_duration.is_none() {
            buffered_source.total_duration()
        } else {
            total_duration
        };
        
        self.sink.stop();
        self.sink.append(buffered_source);
        
        // Update position tracker with new file info
        let mut tracker = self.position.lock().unwrap();
        *tracker = PositionTracker::default();
        tracker.total_duration = duration_from_buffered;
        
        Ok(())
    }

    pub fn play(&self) {
        let mut tracker = self.position.lock().unwrap();
        
        if tracker.start_time.is_none() {
            // First time playing
            tracker.start_time = Some(Instant::now());
        } else if let Some(pause_time) = tracker.last_pause_time {
            // Resuming from pause
            tracker.paused_time += pause_time.elapsed();
            tracker.last_pause_time = None;
        }
        
        tracker.is_playing = true;
        drop(tracker);
        
        self.sink.play();
    }

    pub fn pause(&self) {
        let mut tracker = self.position.lock().unwrap();
        tracker.is_playing = false;
        tracker.last_pause_time = Some(Instant::now());
        drop(tracker);
        
        self.sink.pause();
    }

    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }

    pub fn is_empty(&self) -> bool {
        self.sink.empty()
    }

    pub fn get_total_duration(&self) -> Option<Duration> {
        let tracker = self.position.lock().unwrap();
        tracker.total_duration
    }

    pub fn get_current_position(&self) -> Duration {
        let tracker = self.position.lock().unwrap();
        
        if let Some(start_time) = tracker.start_time {
            if tracker.is_playing {
                // Currently playing
                let elapsed = start_time.elapsed();
                elapsed.saturating_sub(tracker.paused_time)
            } else if let Some(pause_time) = tracker.last_pause_time {
                // Currently paused
                let elapsed_until_pause = pause_time.duration_since(start_time);
                elapsed_until_pause.saturating_sub(tracker.paused_time)
            } else {
                // Not started or stopped
                Duration::ZERO
            }
        } else {
            Duration::ZERO
        }
    }

    pub fn get_position_seconds(&self) -> (u64, u64) {
        let current_pos = self.get_current_position();
        let total_dur = self.get_total_duration().unwrap_or(Duration::ZERO);
        
        (current_pos.as_secs(), total_dur.as_secs())
    }


    pub fn volume_up(&mut self) {
        let mut volume = self.volume + 10.00;

        if volume >= 100.00 {
            volume = 100.00;
        }

        self.volume = volume;
        self.set_volume(self.volume as f32 / 100.0);
    }

    pub fn volume_down(&mut self) {
        let mut volume = self.volume  - 10.00;

        if volume <= 0.0 {
            volume = 0.0;
        }

        self.volume = volume;
        self.set_volume(self.volume / 100.0);
    }

}