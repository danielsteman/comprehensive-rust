#[allow(dead_code)]
pub trait Logger {
    /// Log a message at the given verbosity level.
    fn log(&self, verbosity: u8, message: &str);
}

#[allow(dead_code)]
struct StdoutLogger;

impl Logger for StdoutLogger {
    fn log(&self, verbosity: u8, message: &str) {
        println!("verbosity={verbosity}: {message}");
    }
}

// TODO: Define and implement `VerbosityFilter`.

#[allow(dead_code)]
struct VerbosityFilter {
    max_verbosity: usize,
    inner: StdoutLogger,
}

impl Logger for VerbosityFilter {
    fn log(&self, verbosity: u8, message: &str) {
        if message.len() <= self.max_verbosity {
            self.inner.log(verbosity, message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger() {
        let logger = VerbosityFilter {
            max_verbosity: 3,
            inner: StdoutLogger,
        };
        logger.log(5, "FYI");
        logger.log(2, "Uhoh");
    }
}
