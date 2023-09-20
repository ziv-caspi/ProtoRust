pub trait EventEmitter {
    fn emit<S>(&self, name: &str, message: S)
    where
        S: Clone + serde::Serialize;
}

pub struct TauriEventEmitter {
    pub window_handle: tauri::Window,
}

impl EventEmitter for TauriEventEmitter {
    fn emit<S>(&self, name: &str, message: S)
    where
        S: Clone + serde::Serialize,
    {
        self.window_handle
            .emit(name, message)
            .expect("cannot emit events, something is wrong!");
    }
}

#[derive(Clone, serde::Serialize)]
pub struct PublishEnd(pub Result<i32, String>);

pub fn emit_publish_end_event(emitter: impl EventEmitter, publish_end: PublishEnd) {
    println!("emitting event");
    emitter.emit("publish_end", publish_end);
}
