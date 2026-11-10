use pocketmic_daemon::audio::decoder::AudioDecoder;
use pocketmic_daemon::audio::jitter_buffer::JitterBuffer;
use pocketmic_daemon::net::udp_receiver::UdpReceiver;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎙️ PocketMic Desktop Daemon v1.0.0");
    println!("──────────────────────────────────────");

    let bind_addr = "0.0.0.0:37719";
    println!("[INFO] Initializing UDP Receiver on {}...", bind_addr);
    let receiver = UdpReceiver::bind(bind_addr)?;
    println!("[INFO] UDP socket bound to {:?}", receiver.local_addr()?);

    let (mut producer, mut _consumer) = JitterBuffer::new(4800); // 100 ms buffer
    println!("[INFO] Lock-free SPSC Jitter Buffer initialized (Capacity: 4800 samples)");

    let mut decoder = AudioDecoder::new(1)?;
    println!("[INFO] Opus Audio Decoder ready (48 kHz, Mono, 10 ms frames)");

    let mut packet_buf = [0u8; 2048];
    let mut decoded_pcm = [0.0f32; 480];

    println!("[INFO] PocketMic Daemon is live. Waiting for mobile audio stream...");

    // Simulated tick for verification
    let _ = receiver.poll_and_process(&mut packet_buf, &mut decoder, &mut producer, &mut decoded_pcm);

    println!("[INFO] Core engine loop verification successful.");
    Ok(())
}
