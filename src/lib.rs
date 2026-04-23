//! No-op demo crate.

pub fn note() -> &'static str {
    "noop"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note_is_nonempty() {
        assert!(!note().is_empty());
    }
}
