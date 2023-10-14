use std::{sync::OnceLock, thread};

fn main() {
    // 子线程中调用
    let handle = thread::spawn(|| {
        let logger = App::global();
        logger.log("thread message".to_string());
    });

    // 主线程调用
    let logger = App::global();
    logger.log("some message".to_string());

    let logger2 = App::global();
    logger2.log("other message".to_string());

    handle.join().unwrap();
}

#[derive(Debug)]
struct App {
    name: String,
}

static APP: OnceLock<App> = OnceLock::new();

impl App {
    fn global() -> &'static App {
        // 获取或初始化
        APP.get_or_init(|| {
            println!("App is being created..."); // 初始化打印
            App {
                name: String::from("global"),
            }
        })
    }

    fn log(&self, message: String) {
        println!("{}: {}", self.name, message);
    }
}
