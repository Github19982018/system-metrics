use log::LevelFilter;

use log4rs::{
    append::console::{ConsoleAppender, Target},
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

use log4rs::append::rolling_file::{policy, RollingFileAppender};

pub fn initiate_logging(app_name: String) {
    // Logging to rolling file of size of 10 mb of count 1.

    let k: u64 = 1024;

    let m: u64 = k * k;

    let file_size: u64 = 10 * m;

    let file_count: u32 = 1;

    let app_name = app_name;

    let info_file_path = format!("log/{}_info.log", app_name);

    let error_file_path = format!("log/{}_error.log", app_name);

    let info_roll_pattern = format!("log/{}_info.{{}}.log", app_name);

    let error_roll_pattern = format!("log/{}_error.{{}}.log", app_name);

    let trigger = policy::compound::trigger::size::SizeTrigger::new(file_size);

    let info_roller = policy::compound::roll::fixed_window::FixedWindowRoller::builder()
        .build(&info_roll_pattern, file_count)
        .unwrap();

    let error_roller = policy::compound::roll::fixed_window::FixedWindowRoller::builder()
        .build(&error_roll_pattern, file_count)
        .unwrap();

    let info_policy =
        policy::compound::CompoundPolicy::new(Box::new(trigger.clone()), Box::new(info_roller));

    let error_policy =
        policy::compound::CompoundPolicy::new(Box::new(trigger), Box::new(error_roller));

    // Logging to info log file.

    let info_logfile = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%+)(utc)} [{f}:{L}] {h({l})} {M}:{m}{n}",
        )))
        .build(info_file_path, Box::new(info_policy))
        .unwrap();

    // Logging to error log file.

    let error_logfile = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%+)(utc)} [{f}:{L}] {h({l})} {M}:{m}{n}",
        )))
        .build(error_file_path, Box::new(error_policy))
        .unwrap();

    // Log configuration

    let config = Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Info)))
                .build("info_logfile", Box::new(info_logfile)),
        )
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Info)))
                .build("error_logfile", Box::new(error_logfile)),
        )
        .build(
            Root::builder()
                .appender("info_logfile")
                .appender("error_logfile")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    // Initialize logging

    log4rs::init_config(config).unwrap();
}
