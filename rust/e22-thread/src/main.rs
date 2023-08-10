mod join;
mod r#move;
mod channel;

fn main() {
    // crate::join::thread_join();
    // crate::r#move::thread_move();
    crate::channel::thread_channel();
}
