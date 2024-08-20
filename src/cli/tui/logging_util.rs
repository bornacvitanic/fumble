use log::{debug, error, info, trace, warn};

pub fn make_trace() {
    trace!("Hey i am a trace!")
}

pub fn make_debug() {
    debug!("Hey i am a debug!")
}

pub fn make_log() {
    info!("Hey i am a log!")
}

pub fn make_warning() {
    warn!("Hey i am a warning!")
}

pub fn make_error() {
    error!("Hey i am a error!")
}
