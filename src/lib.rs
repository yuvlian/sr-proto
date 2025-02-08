pub use prost::Message;

pub fn decode<T: Message + Default>(data: &[u8]) -> T {
    T::decode(data).unwrap_or_else(|_| {
        println!("!! failed decoding to msg, defaulting !!");
        T::default()
    })
}

include!("../out/_.rs");

pub mod cmd {
    include!("../out/cmd.rs");
}
