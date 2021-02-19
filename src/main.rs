use rodio::Source;

fn main() {
    let (a, b) = rodio::OutputStream::try_default().unwrap();
    let buf = std::fs::read("music1.ogg").ok().unwrap();

    let s = rodio::Decoder::new(std::io::Cursor::new(buf)).unwrap();
    b.play_raw(s.convert_samples()).unwrap();
    loop {}
}
