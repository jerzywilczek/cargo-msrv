#![allow(unused)]

use std::io::Stderr;
use std::sync::{Arc, Mutex, RwLock};
use std::{io, marker};
use storyteller::EventHandler;

pub trait SendWriter: io::Write + Send + 'static {}

pub struct JsonHandler<W: SendWriter> {
    writer: Arc<Mutex<W>>,
}

impl<W: SendWriter> JsonHandler<W> {
    const LOCK_FAILURE_MSG: &'static str =
        "{ \"panic\": true, \"cause\": \"Unable to lock writer for JsonHandle\" }";
    const SERIALIZE_FAILURE_MSG: &'static str =
        "{ \"panic\": true, \"cause\": \"Unable to serialize event for JsonHandle\" }";
    const WRITE_FAILURE_MSG: &'static str =
        "{ \"panic\": true, \"cause\": \"Unable to write serialized event for JsonHandle\" }";
}

impl SendWriter for Stderr {}

impl JsonHandler<Stderr> {
    pub fn stderr() -> Self {
        Self {
            writer: Arc::new(Mutex::new(io::stderr())),
        }
    }
}

impl<W: SendWriter> EventHandler for JsonHandler<W> {
    type Event = super::Event;

    fn handle(&self, event: Self::Event) {
        let mut w = self.writer.lock().expect(Self::LOCK_FAILURE_MSG);
        let serialized_event = serde_json::to_string(&event).expect(Self::SERIALIZE_FAILURE_MSG);

        writeln!(&mut w, "->. {}", &serialized_event).expect(Self::WRITE_FAILURE_MSG);
    }
}

pub struct HumanProgressHandler {
    bar: indicatif::ProgressBar,
}

impl HumanProgressHandler {
    pub fn new() -> Self {
        Self {
            bar: indicatif::ProgressBar::hidden(),
        }
    }
}

impl EventHandler for HumanProgressHandler {
    type Event = super::Event;

    fn handle(&self, _event: Self::Event) {
        if self.bar.is_hidden() {
            self.bar
                .set_draw_target(indicatif::ProgressDrawTarget::stderr());
            self.bar.set_length(9000);
        }

        self.bar.inc(1);
        self.bar.println("event!");
    }
}
