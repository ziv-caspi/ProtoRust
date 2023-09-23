#[derive(Clone, serde::Serialize)]
pub enum Event {
    PublishEnd(Result<i32, String>),
    ProgressUpdate(ProgressUpdate),
}

pub trait EventEmitter {
    fn emit(&self, event: Event);
}

impl EventEmitter for tauri::Window {
    fn emit(&self, event: Event) {
        match event {
            Event::PublishEnd(result) => self.emit("publish_end", result),
            Event::ProgressUpdate(progress) => self.emit("progress_update", progress),
        };
    }
}

#[derive(Clone, serde::Serialize)]
pub struct ProgressUpdate {
    published_count: i32,
    expected_speed: i32,
    actual_speed: i32,
}
