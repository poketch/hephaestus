pub mod log {

    #[derive(Debug)]

    pub enum LogLevel {
        Error,
        Warning,
        Info
    }

    impl std::fmt::Display for LogLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{:?}]", self)
        }
    }


    pub fn log(level: LogLevel, msg: impl std::fmt::Display) -> () {

        println!("{level}: {msg}")

    }

}

pub use log::*; 