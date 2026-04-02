use std::sync::atomic::{AtomicU64, Ordering};

/// Thread-safe unique ID generator for ARIA attribute cross-references.
///
/// Every interactive component needs unique, stable IDs for ARIA attributes
/// like `aria-controls`, `aria-labelledby`, and `aria-describedby`.
///
/// IDs are generated with a configurable prefix and an atomic counter,
/// producing values like `"stratum-btn-001"`, `"stratum-dialog-002"`.
pub struct IdGenerator {
    prefix: String,
    counter: AtomicU64,
}

impl IdGenerator {
    /// Create a new ID generator with the given prefix.
    ///
    /// # Example
    /// ```
    /// use stratum_core::IdGenerator;
    /// let id_gen = IdGenerator::new("btn");
    /// let id = id_gen.next();
    /// assert!(id.starts_with("stratum-btn-"));
    /// ```
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            counter: AtomicU64::new(0),
        }
    }

    /// Generate the next unique ID.
    pub fn next(&self) -> String {
        let n = self.counter.fetch_add(1, Ordering::Relaxed);
        format!("stratum-{}-{:03}", self.prefix, n)
    }

    /// Generate a paired set of IDs for label + target relationships.
    ///
    /// Returns `(label_id, target_id)` — useful for `aria-labelledby`
    /// and `aria-controls` relationships.
    pub fn paired(&self) -> (String, String) {
        let n = self.counter.fetch_add(1, Ordering::Relaxed);
        (
            format!("stratum-{}-{:03}-label", self.prefix, n),
            format!("stratum-{}-{:03}-target", self.prefix, n),
        )
    }

    /// Generate a set of related IDs for multi-part components.
    ///
    /// For example, a Disclosure needs a trigger ID and content ID.
    pub fn group(&self, suffixes: &[&str]) -> Vec<String> {
        let n = self.counter.fetch_add(1, Ordering::Relaxed);
        suffixes
            .iter()
            .map(|suffix| format!("stratum-{}-{:03}-{}", self.prefix, n, suffix))
            .collect()
    }

    /// Get the current counter value (for testing).
    pub fn current_count(&self) -> u64 {
        self.counter.load(Ordering::Relaxed)
    }
}

/// Global ID generators for each component type.
///
/// Using separate generators per component type keeps IDs readable
/// and predictable in tests.
pub mod generators {
    use super::IdGenerator;
    use std::sync::LazyLock;

    pub static BUTTON: LazyLock<IdGenerator> = LazyLock::new(|| IdGenerator::new("btn"));
    pub static CHECKBOX: LazyLock<IdGenerator> = LazyLock::new(|| IdGenerator::new("chk"));
    pub static RADIO: LazyLock<IdGenerator> = LazyLock::new(|| IdGenerator::new("radio"));
    pub static SWITCH: LazyLock<IdGenerator> = LazyLock::new(|| IdGenerator::new("switch"));
    pub static SLIDER: LazyLock<IdGenerator> = LazyLock::new(|| IdGenerator::new("slider"));
    pub static DISCLOSURE: LazyLock<IdGenerator> =
        LazyLock::new(|| IdGenerator::new("disclosure"));
    pub static DIALOG: LazyLock<IdGenerator> = LazyLock::new(|| IdGenerator::new("dialog"));
    pub static POPOVER: LazyLock<IdGenerator> = LazyLock::new(|| IdGenerator::new("popover"));
    pub static TOOLTIP: LazyLock<IdGenerator> = LazyLock::new(|| IdGenerator::new("tooltip"));
    pub static MENU: LazyLock<IdGenerator> = LazyLock::new(|| IdGenerator::new("menu"));
    pub static SELECT: LazyLock<IdGenerator> = LazyLock::new(|| IdGenerator::new("select"));
    pub static TABS: LazyLock<IdGenerator> = LazyLock::new(|| IdGenerator::new("tabs"));
    pub static ACCORDION: LazyLock<IdGenerator> =
        LazyLock::new(|| IdGenerator::new("accordion"));
    pub static FORM: LazyLock<IdGenerator> = LazyLock::new(|| IdGenerator::new("form"));
    pub static INPUT: LazyLock<IdGenerator> = LazyLock::new(|| IdGenerator::new("input"));
    pub static TOAST: LazyLock<IdGenerator> = LazyLock::new(|| IdGenerator::new("toast"));
    pub static TABLE: LazyLock<IdGenerator> = LazyLock::new(|| IdGenerator::new("table"));
    pub static TREE: LazyLock<IdGenerator> = LazyLock::new(|| IdGenerator::new("tree"));
    pub static TOGGLE: LazyLock<IdGenerator> = LazyLock::new(|| IdGenerator::new("toggle"));
    pub static PRESSABLE: LazyLock<IdGenerator> =
        LazyLock::new(|| IdGenerator::new("pressable"));
    pub static FOCUS_SCOPE: LazyLock<IdGenerator> =
        LazyLock::new(|| IdGenerator::new("focus-scope"));
    pub static SEPARATOR: LazyLock<IdGenerator> =
        LazyLock::new(|| IdGenerator::new("separator"));
    pub static PORTAL: LazyLock<IdGenerator> = LazyLock::new(|| IdGenerator::new("portal"));
    pub static GENERIC: LazyLock<IdGenerator> = LazyLock::new(|| IdGenerator::new("id"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_generator_sequential() {
        let id_gen = IdGenerator::new("test");
        assert_eq!(id_gen.next(), "stratum-test-000");
        assert_eq!(id_gen.next(), "stratum-test-001");
        assert_eq!(id_gen.next(), "stratum-test-002");
    }

    #[test]
    fn id_generator_paired() {
        let id_gen = IdGenerator::new("field");
        let (label, target) = id_gen.paired();
        assert_eq!(label, "stratum-field-000-label");
        assert_eq!(target, "stratum-field-000-target");
    }

    #[test]
    fn id_generator_group() {
        let id_gen = IdGenerator::new("disclosure");
        let ids = id_gen.group(&["trigger", "content"]);
        assert_eq!(ids.len(), 2);
        assert_eq!(ids[0], "stratum-disclosure-000-trigger");
        assert_eq!(ids[1], "stratum-disclosure-000-content");
    }

    #[test]
    fn id_generator_thread_safe() {
        use std::thread;

        let id_gen = IdGenerator::new("thread");
        let id_ref = &id_gen;

        thread::scope(|s| {
            let mut handles = vec![];
            for _ in 0..10 {
                handles.push(s.spawn(|| id_ref.next()));
            }
            let ids: Vec<String> = handles.into_iter().map(|h| h.join().unwrap()).collect();
            // All IDs should be unique
            let mut sorted = ids.clone();
            sorted.sort();
            sorted.dedup();
            assert_eq!(sorted.len(), ids.len());
        });
    }

    #[test]
    fn global_generators_unique() {
        let btn_id = generators::BUTTON.next();
        let chk_id = generators::CHECKBOX.next();
        assert!(btn_id.starts_with("stratum-btn-"));
        assert!(chk_id.starts_with("stratum-chk-"));
        assert_ne!(btn_id, chk_id);
    }

    #[test]
    fn current_count() {
        let id_gen = IdGenerator::new("count");
        assert_eq!(id_gen.current_count(), 0);
        id_gen.next();
        assert_eq!(id_gen.current_count(), 1);
        id_gen.next();
        assert_eq!(id_gen.current_count(), 2);
    }
}
