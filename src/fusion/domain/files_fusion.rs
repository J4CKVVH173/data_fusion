#[cfg(test)]
mod tests;

mod extended_file;
mod traits;
mod fusion;

pub use self::traits::FileAccess;
pub use self::extended_file::ExtendedFile;
