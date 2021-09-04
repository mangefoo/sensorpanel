pub trait LogExt {
    fn log(level: LogLevel, message: &str);
}

#[derive(Debug)]
pub enum LogLevel {
    TRACE,
    DEBUG,
    INFO,
//    WARN,
    ERROR
}

pub struct Log();

impl LogExt for Log {
    fn log(level: LogLevel, message: &str) {
        //let now: DateTime<Local> = Local::now();
        //println!("{} [{:?}]:  {}", now.format("%Y-%m-%d %H:%M:%S%.3f"), level, message);

        println!("[{:?}] {}", level, message);
    }
}