use std::fmt;
use std::sync::Arc;

/// A type-erased, cloneable callback.
///
/// Used by components to accept event handler functions from consumers
/// without coupling to a specific framework's callback type.
///
/// Framework adapters convert framework-specific callbacks (e.g., Leptos
/// `Callback`, Dioxus closures) to and from this type.
pub struct Callback<T: 'static> {
    f: Arc<dyn Fn(T) + Send + Sync>,
}

impl<T: 'static> Callback<T> {
    /// Create a new callback from a function.
    pub fn new(f: impl Fn(T) + Send + Sync + 'static) -> Self {
        Self { f: Arc::new(f) }
    }

    /// Invoke the callback with the given argument.
    pub fn call(&self, arg: T) {
        (self.f)(arg);
    }
}

impl<T: 'static> Clone for Callback<T> {
    fn clone(&self) -> Self {
        Self {
            f: Arc::clone(&self.f),
        }
    }
}

impl<T: 'static> fmt::Debug for Callback<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Callback(..)")
    }
}

impl<T: 'static> PartialEq for Callback<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.f, &other.f)
    }
}

/// A callback that takes no arguments.
pub type ActionCallback = Callback<()>;

/// A callback that receives a boolean value (toggle, checkbox, switch).
pub type BoolCallback = Callback<bool>;

/// A callback that receives a string value (input change, select).
pub type StringCallback = Callback<String>;

/// A callback that receives an index (navigation, selection).
pub type IndexCallback = Callback<usize>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn callback_call() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = Arc::clone(&called);

        let cb = Callback::new(move |_: ()| {
            called_clone.store(true, Ordering::SeqCst);
        });

        cb.call(());
        assert!(called.load(Ordering::SeqCst));
    }

    #[test]
    fn callback_clone() {
        let cb = Callback::new(|x: i32| {
            let _ = x;
        });
        let cb2 = cb.clone();
        assert_eq!(cb, cb2);
    }

    #[test]
    fn callback_debug() {
        let cb = Callback::new(|_: ()| {});
        assert_eq!(format!("{:?}", cb), "Callback(..)");
    }

    #[test]
    fn callback_with_value() {
        let received = Arc::new(std::sync::Mutex::new(String::new()));
        let received_clone = Arc::clone(&received);

        let cb = StringCallback::new(move |val: String| {
            *received_clone.lock().unwrap() = val;
        });

        cb.call("hello".to_string());
        assert_eq!(*received.lock().unwrap(), "hello");
    }
}
