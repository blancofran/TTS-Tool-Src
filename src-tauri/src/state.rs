use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// Tracks the cancellation flag for the currently running transcription or
/// model download, if any. Only one job runs at a time in this phase.
#[derive(Default)]
pub struct TranscriptionState {
    cancel_flag: Mutex<Option<Arc<AtomicBool>>>,
}

impl TranscriptionState {
    /// Registers a new job and returns its cancellation flag.
    pub fn begin_job(&self) -> Arc<AtomicBool> {
        let flag = Arc::new(AtomicBool::new(false));
        *self.cancel_flag.lock().unwrap() = Some(flag.clone());
        flag
    }

    pub fn end_job(&self) {
        *self.cancel_flag.lock().unwrap() = None;
    }

    /// Signals cancellation for the running job, if any. Returns `true` if a
    /// job was actually running.
    pub fn request_cancel(&self) -> bool {
        match self.cancel_flag.lock().unwrap().as_ref() {
            Some(flag) => {
                flag.store(true, Ordering::Relaxed);
                true
            }
            None => false,
        }
    }
}

pub fn is_cancelled(flag: &AtomicBool) -> bool {
    flag.load(Ordering::Relaxed)
}
