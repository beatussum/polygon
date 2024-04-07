pub mod generate;
pub use generate::generate;

pub mod process;

#[cfg(feature = "dac")] pub use process::process_dac;
#[cfg(feature = "frames")] pub use process::process_frames;
#[cfg(feature = "naive")] pub use process::process_naive;
