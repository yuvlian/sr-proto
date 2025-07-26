#![deprecated(note = "this is deprecated. however, you can still get raw protos from qingque-sr, if you need them")]

pub use prost;

include!("../out/_.rs");

pub mod cmd {
    include!("../out/cmd.rs");
}
