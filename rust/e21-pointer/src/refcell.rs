pub trait Messenger {
    fn send(&self, message: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        if self.value > self.max {
            self.messenger
                .send(&format!("Limit reached: {}", self.value));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        pub fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: Vec::new(),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(message.to_string());
        }
    }

    #[test]
    fn test_refcell_pointer() {
        let messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&messenger, 10);
        limit_tracker.set_value(5);
        assert_eq!(messenger.sent_messages.len(), 0);
        limit_tracker.set_value(10);
        assert_eq!(messenger.sent_messages.len(), 1);
    }
}
