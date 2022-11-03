use std::time::Duration;
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source, SkipDuration};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    
    for _ in 0..100 {
        let source = SineWave::new(880.0).take_duration(Duration::from_millis(20)).amplify(0.20).delay(Duration::from_millis(20));
        sink.append(source);
    }

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}
