pub use prost::Message as MsgTrait;

#[macro_export]
macro_rules! dec {
    ($message:ident, $req:ident) => {
        match $message::decode($req) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to decode {}", stringify!($message));
                $message::default()
            }
        }
    };
}

pub mod pb {
    include!("../out/_.rs");
}

pub mod cmd {
    include!("../out/cmd.rs");
}
