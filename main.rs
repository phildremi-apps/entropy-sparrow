use espa_codecs;

fn main() {
    println!("Supported codecs:");
    println!("{}", espa_codecs::list());
}
