pub use prost::Message;

include!("../out/_.rs");

pub mod cmd {
    include!("../out/cmd.rs");
}
