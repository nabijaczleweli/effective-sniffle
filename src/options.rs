use clap::{App, AppSettings};

/// Representation of the application's all configurable values
#[derive(Debug, Clone, Hash)]
pub struct Options {
    /// Source file
    pub input: String,
    /// Output file
    pub output: String,
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        static USAGE: &'static str = "<input>  'Source file'
                                      <output> 'Output file'";

        let matches = App::new("effective-sniffle-c")
                          .setting(AppSettings::ColoredHelp)
                          .version(env!("CARGO_PKG_VERSION"))
                          .author("nabijaczleweli <nabijaczleweli@gmail.com>")
                          .about("A compiler for a language with a compiler")
                          .args_from_usage(USAGE)
                          .get_matches();

        Options {
            input: matches.value_of("input").unwrap().to_string(),
            output: matches.value_of("output").unwrap().to_string(),
        }
    }
}
