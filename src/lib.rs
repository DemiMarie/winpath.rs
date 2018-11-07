#![cfg_attr(not(windows), forbid(unsafe_code))]
mod dotdot;
pub use dotdot::*;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
