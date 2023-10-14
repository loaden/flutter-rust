use std::{sync::OnceLock, thread};

static APP: OnceLock<App> = OnceLock::new();
static mut APP2: OnceLock<App2> = OnceLock::new();

fn main() {
    // 子线程中调用
    let handle = thread::spawn(|| {
        let logger = App::global();
        logger.log("thread message".to_string());
    });

    // 主线程调用
    let logger = App::global();
    logger.log("some message".to_string());
    // logger.save("message".to_string());

    App2::global();
    unsafe {
        let logger2 = APP2.get_mut().unwrap();
        logger2.save("mut_".to_string());
        logger2.save("mut_".to_string());
    }

    handle.join().unwrap();
}

#[derive(Debug, Default)]
struct App {
    name: String,
}

impl App {
    fn global() -> &'static App {
        // 获取或初始化
        APP.get_or_init(|| {
            println!("App is being created..."); // 初始化打印
            App::default()
        })
    }

    fn log(&self, message: String) {
        println!("{}: {}", self.name, message);
    }
}

#[derive(Debug, Default)]
struct App2 {
    msgs: String,
}

impl App2 {
    fn global() -> &'static App2 {
        unsafe {
            // 获取或初始化
            APP2.get_or_init(|| {
                println!("App2 is being created..."); // 初始化打印
                App2::default()
            })
        }
    }

    fn save(&mut self, message: String) {
        self.msgs += message.as_str();
        println!("self.msgs: {}", self.msgs);
    }
}
