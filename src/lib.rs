extern crate env_logger;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate clap;
#[cfg(feature = "color")]
extern crate ansi_term;

#[cfg(feature = "color")]
use ansi_term::Colour::{Blue, Green, Red, White, Yellow};
use clap::ArgMatches;
use env_logger::LogBuilder;
use log::{LogLevel, LogLevelFilter, LogRecord, SetLoggerError};
use std::sync::atomic::{AtomicUsize, Ordering};

lazy_static! {
    static ref HACKLOG_INDENT:AtomicUsize = {
        AtomicUsize::new(0)
    };
}


pub fn init(opts: Option<&ArgMatches>, level: Option<LogLevelFilter>) -> Result<(), SetLoggerError> {
    let mut builder = LogBuilder::new();
    init_format(&mut builder);
    if let Some(opts) = opts {
        init_clap(opts, &mut builder);
    }
    if let Some(level) = level {
        builder.filter(None, level);
    }
    builder.init()
}

fn init_format(builder: &mut LogBuilder) {
    #[cfg(not(feature = "color"))]
    builder.format(|record: &LogRecord| {
        format!("{:>ident$} {}",
                match record.level() {
                    LogLevel::Error => "[-]",
                    LogLevel::Warn => "[*]",
                    LogLevel::Info => "[+]",
                    LogLevel::Debug => "[#]",
                    LogLevel::Trace => "[%]",
                },
                record.args(),
                ident=HACKLOG_INDENT.load(Ordering::SeqCst) * 4)
    });
    #[cfg(feature = "color")]
    builder.format(|record: &LogRecord| {
        format!("{holder:>ident$}{} {}",
                match record.level() {
                    LogLevel::Error => Red.paint("[-]"),
                    LogLevel::Warn => Yellow.paint("[*]"),
                    LogLevel::Info => Green.paint("[+]"),
                    LogLevel::Debug => Blue.paint("[#]"),
                    LogLevel::Trace => White.paint("[%]"),
                },
                record.args(),
                holder="",
                ident=HACKLOG_INDENT.load(Ordering::SeqCst) * 4)
    });
}

fn init_clap(opts: &ArgMatches, builder: &mut LogBuilder) {
    builder.filter(None,
                   match opts.occurrences_of("verbose") {
                       n if n >= 3 => LogLevelFilter::Trace,
                       n if n == 2 => LogLevelFilter::Debug,
                       n if n == 1 => LogLevelFilter::Info,
                       _ => LogLevelFilter::Warn,
                   });
}

fn indent() -> usize {
    HACKLOG_INDENT.fetch_add(1, Ordering::SeqCst)
}

fn unindent() -> usize {
    HACKLOG_INDENT.fetch_sub(1, Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::LogLevelFilter;

    #[test]
    fn test() {
        init(None, Some(LogLevelFilter::Trace)).unwrap();
        error!("level 1 - error message 1");
        indent();
        error!("level 2 - error message 1");
        indent();
        error!("level 3 - error message 1");
        unindent();
        error!("level 2 - error message 2");
        unindent();
        error!("level 1 - error message 1");
        warn!("a warning message");
        info!("a info message");
        debug!("a debug message");
        trace!("a trace message");
    }
}
