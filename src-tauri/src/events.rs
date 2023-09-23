pub trait EventEmitter {
    fn emit<S>(&self, name: &str, message: S)
    where
        S: Clone + serde::Serialize;

    fn emit_publish_end_event(&self, publish_end: PublishEnd);
}

impl EventEmitter for tauri::Window {
    fn emit<S>(&self, name: &str, message: S)
    where
        S: Clone + serde::Serialize,
    {
        let _ = self.emit(name, message);
    }

    fn emit_publish_end_event(&self, publish_end: PublishEnd) {
        println!("emitting publish end event");
        let _ = self.emit("publish_end", publish_end);
    }
}

#[derive(Clone, serde::Serialize)]
pub struct PublishEnd(pub Result<i32, String>);
