
// use std::iter::Filter;

use log::LevelFilter;
use log4rs::{
    append::rolling_file::policy::compound::{
            roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
    },
    config::{Appender, Config, Logger, Root},
    encode::pattern::PatternEncoder, filter::threshold::ThresholdFilter,
};

pub fn log_init(appname:&str) {
    const TRIGGER_FILE_SIZE: u64 = 1024*1024;
    /// Number of archive log files to keep
    const LOG_FILE_COUNT: u32 = 10;
    // Location where logs will be written to
    println!("{}",appname.to_string().as_str().to_owned());
    let err_file_path: String  = format!("log/{}-error.log",appname);
    let info_file_path: String = format!("log/{appname}-info.log");
    
    // Location where log archives will be moved to
    // For Pattern info See:
    let err_archive_pattern: String = format!("log/archive/{appname}-error")+".{}.log";
    let info_archive_pattern: String = format!("log/archive/{appname}-info")+".{}.log";
    let info_level = log::LevelFilter::Info;
    let err_level = log::LevelFilter::Error;

    // Create a policy to use with the file logging
    let trigger = SizeTrigger::new(TRIGGER_FILE_SIZE);
    let err_roller = FixedWindowRoller::builder()
        .base(0) // Default Value (line not needed unless you want to change from 0 (only here for demo purposes)
        .build(&err_archive_pattern, LOG_FILE_COUNT) // Roll based on pattern and max 3 archive files
        .unwrap();
    let err_policy = CompoundPolicy::new(Box::new(trigger), Box::new(err_roller));
    // Logging to log file. (with rolling)
    let err_logfile = log4rs::append::rolling_file::RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {d(%Y-%m-%d %H:%M:%S)} - {m}\n")))
        .build(&err_file_path, Box::new(err_policy))
        .unwrap();

    let info_roller = FixedWindowRoller::builder()
        .base(0) // Default Value (line not needed unless you want to change from 0 (only here for demo purposes)
        .build(&info_archive_pattern, LOG_FILE_COUNT) // Roll based on pattern and max 3 archive files
        .unwrap();
    let info_policy = CompoundPolicy::new(Box::new(trigger), Box::new(info_roller));
    // Logging to log file. (with rolling)
    let info_logfile = log4rs::append::rolling_file::RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {d(%Y-%m-%d %H:%M:%S)} - {m}{n}")))
        .build(&info_file_path, Box::new(info_policy))
        .unwrap();


    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder()
        .filter(Box::new(ThresholdFilter::new(err_level)))
        .build("errlogfile", Box::new(err_logfile)))
        .appender(Appender::builder()
        .filter(Box::new(ThresholdFilter::new(info_level)))
        .build("infologfile", Box::new(info_logfile)))
        .logger(Logger::builder()
            .appender("infologfile")
            .additive(false)
            .build("info", info_level))
        .logger(Logger::builder()
            .appender("errlogfile")
            .additive(false)
            .build("error", err_level))
        .build(
            Root::builder()
                .appender("errlogfile")
                .appender("infologfile")
                .build(LevelFilter::Info),
        )
        .unwrap();
    let _handle = log4rs::init_config(config);

}