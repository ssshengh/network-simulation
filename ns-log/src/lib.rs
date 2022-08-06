use env_logger::filter::{Builder, Filter};
use env_logger::Target;
use log::{info, trace, LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::fmt;

type NsLogMessage = Option<Box<dyn Fn(&str, log::Level, &str) + Send + Sync>>;

struct NsLogger {
    callback: NsLogMessage,
    inner: Filter,
}

impl NsLogger {
    fn log_message(&self, category: &str, level: log::Level, msg: &str) {
        if let Some(cb) = self.callback.as_ref() {
            cb(category, level, msg);
        } else {
            eprint!("Callback logger should be set!");
        }
    }
}

impl Log for NsLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.inner.enabled(metadata)
    }
    /// 将日志的记录进行项目风格的处理, 对于收集到的每一条日志都是一个 record
    fn log(&self, record: &Record) {
        let category = "ns";

        let mut message = if let Some(file) = record.file() {
            format!("[{}] [{}]", record.target(), file)
        } else {
            format!("[{}]", record.target())
        };

        if let Err(e) = fmt::write(&mut message, *record.args()) {
            eprint!("Can not format log, error: {}", e);
        }

        self.log_message(&category, record.level(), &message);
    }

    fn flush(&self) {}
}

fn create_logger_filter() -> Filter {
    // 用于分析一段指令以生成 Filter
    let mut builder = Builder::new();
    // 给定环境变量的情况下, 通过环境变量搞定, 否则使用默认的 log level
    if let Ok(var) = std::env::var("NS_LOG") {
        builder.parse(&var);
    } else {
        builder.filter(Some("ns"), LevelFilter::Info);
        builder.filter(Some("ns_core"), LevelFilter::Info);
        builder.filter(Some("ns_algorithm"), LevelFilter::Info);

        builder.filter(None, LevelFilter::Warn);
    }

    builder.build()
}

fn init_env_logger() -> Result<(), SetLoggerError> {
    let mut builder = env_logger::Builder::new();
    builder.target(Target::Stdout);
    if let Ok(var) = std::env::var("NS_LOG") {
        builder.parse_filters(&var);
    } else {
        builder.filter(Some("ns"), LevelFilter::Info);
        builder.filter(Some("ns_core"), LevelFilter::Info);
        builder.filter(Some("ns_algorithm"), LevelFilter::Info);

        builder.filter(None, LevelFilter::Warn);
    }

    builder.try_init()
}

pub fn init(callback: NsLogMessage) -> Result<(), SetLoggerError> {
    let force_stdout = std::env::var("NS_LOG_STD_OUT").is_ok();
    match (&callback, force_stdout) {
        (Some(_), false) => {
            let log_backend = Box::new(NsLogger {
                callback,
                inner: create_logger_filter(),
            });
            log::set_max_level(log_backend.inner.filter());
            log::set_boxed_logger(log_backend)?;
            info!("Using custom callback logger!");
        }
        _ => {
            init_env_logger()?;
            info!("Using env_logger as default!");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{init, NsLogMessage};
    use log::{error, info, trace};

    #[test]
    fn logger_base_env() -> Result<(), Box<dyn std::error::Error>> {
        std::env::set_var("NS_LOG", "ns_log=trace");
        init(None)?;
        // std::env::set_var("NS_LOG", "ns_log=info");
        info!("Hello info!");
        error!("Hello Error!");
        trace!("Hello debug!");

        Ok(())
    }
    #[test]
    fn test_with_callback() -> Result<(), Box<dyn std::error::Error>> {
        std::env::set_var("NS_LOG", "ns_log=trace");
        let cb: NsLogMessage = Some(Box::new(|category: &str, level: log::Level, msg: &str| {
            println!("----{}----", msg);
        }));
        init(cb)?;
        info!("hello info!");
        trace!("hello msg!");

        Ok(())
    }
}
