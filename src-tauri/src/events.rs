#[derive(Clone, serde::Serialize)]
pub enum Event {
    PublishEnd(Result<i32, String>),
    ProgressUpdate(ProgressUpdateData),
}

pub trait EventEmitter {
    fn emit(&self, event: Event);
}

impl EventEmitter for tauri::Window {
    fn emit(&self, event: Event) {
        let _ = match event {
            Event::PublishEnd(result) => self.emit("publish_end", result),
            Event::ProgressUpdate(progress) => self.emit("progress_update", progress),
        };
    }
}

#[derive(Clone, serde::Serialize)]
pub struct ProgressUpdateData {
    pub published_count: i32,
    pub expected_speed: i32,
    pub actual_speed: i32,
}
