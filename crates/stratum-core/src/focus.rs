use serde::{Deserialize, Serialize};

/// Manages focus behavior for interactive components.
///
/// Different component types require different focus management strategies:
/// - Dialogs trap focus within their boundary
/// - Popovers restore focus to the trigger on close
/// - Menus focus the first item on open
///
/// `FocusManager` encapsulates these strategies and provides methods
/// for programmatic focus control.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FocusManager {
    /// The focus strategy to use.
    pub strategy: FocusStrategy,
    /// The ID of the container element to manage focus within.
    container_id: Option<String>,
    /// The ID of the element that had focus before trapping.
    restore_target: Option<String>,
}

/// Focus management strategies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FocusStrategy {
    /// Browser default focus behavior.
    Auto,
    /// Trap focus within the component (dialogs, modals).
    /// Tab and Shift+Tab cycle through focusable children.
    Trap,
    /// Restore focus to the previously focused element on close.
    Restore,
    /// Focus the first focusable child on mount.
    Initial,
    /// Combination: trap focus AND restore on close (dialogs).
    TrapAndRestore,
}

impl FocusManager {
    /// Create a new FocusManager with the given strategy.
    pub fn new(strategy: FocusStrategy) -> Self {
        Self {
            strategy,
            container_id: None,
            restore_target: None,
        }
    }

    /// Create a FocusManager for a dialog (trap + restore).
    pub fn dialog() -> Self {
        Self::new(FocusStrategy::TrapAndRestore)
    }

    /// Create a FocusManager for a menu (focus first item).
    pub fn menu() -> Self {
        Self::new(FocusStrategy::Initial)
    }

    /// Create a FocusManager for a popover (restore on close).
    pub fn popover() -> Self {
        Self::new(FocusStrategy::Restore)
    }

    /// Set the container element ID.
    pub fn with_container(mut self, id: impl Into<String>) -> Self {
        self.container_id = Some(id.into());
        self
    }

    /// Get the container ID.
    pub fn container_id(&self) -> Option<&str> {
        self.container_id.as_deref()
    }

    /// Record the currently focused element for later restoration.
    pub fn save_restore_target(&mut self, element_id: impl Into<String>) {
        self.restore_target = Some(element_id.into());
    }

    /// Get the element ID to restore focus to.
    pub fn restore_target(&self) -> Option<&str> {
        self.restore_target.as_deref()
    }

    /// Begin trapping focus within the container.
    ///
    /// Returns focus instructions that the framework adapter should execute.
    pub fn trap(&self, container_id: &str) -> FocusInstruction {
        FocusInstruction::Trap {
            container_id: container_id.to_string(),
        }
    }

    /// Release the focus trap.
    pub fn release(&self) -> FocusInstruction {
        FocusInstruction::Release
    }

    /// Restore focus to the previously focused element.
    pub fn restore(&self) -> FocusInstruction {
        match &self.restore_target {
            Some(id) => FocusInstruction::FocusElement {
                element_id: id.clone(),
            },
            None => FocusInstruction::Release,
        }
    }

    /// Focus the first focusable child of a container.
    pub fn focus_first(&self, container_id: &str) -> FocusInstruction {
        FocusInstruction::FocusFirst {
            container_id: container_id.to_string(),
        }
    }

    /// Focus the last focusable child of a container.
    pub fn focus_last(&self, container_id: &str) -> FocusInstruction {
        FocusInstruction::FocusLast {
            container_id: container_id.to_string(),
        }
    }

    /// Focus the next focusable element.
    pub fn focus_next(&self) -> FocusInstruction {
        FocusInstruction::FocusNext
    }

    /// Focus the previous focusable element.
    pub fn focus_prev(&self) -> FocusInstruction {
        FocusInstruction::FocusPrev
    }

    /// Whether this manager uses focus trapping.
    pub fn is_trapping(&self) -> bool {
        matches!(
            self.strategy,
            FocusStrategy::Trap | FocusStrategy::TrapAndRestore
        )
    }

    /// Whether this manager restores focus on close.
    pub fn should_restore(&self) -> bool {
        matches!(
            self.strategy,
            FocusStrategy::Restore | FocusStrategy::TrapAndRestore
        )
    }
}

/// Instructions for framework adapters to execute focus operations.
///
/// Since `stratum-core` is framework-agnostic, it cannot directly manipulate
/// the DOM. Instead, it produces `FocusInstruction`s that the framework
/// adapter translates to actual DOM focus operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FocusInstruction {
    /// Trap focus within a container element.
    Trap { container_id: String },
    /// Release the focus trap.
    Release,
    /// Focus a specific element by ID.
    FocusElement { element_id: String },
    /// Focus the first focusable child of a container.
    FocusFirst { container_id: String },
    /// Focus the last focusable child of a container.
    FocusLast { container_id: String },
    /// Focus the next focusable element in tab order.
    FocusNext,
    /// Focus the previous focusable element in tab order.
    FocusPrev,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_manager_dialog() {
        let fm = FocusManager::dialog();
        assert_eq!(fm.strategy, FocusStrategy::TrapAndRestore);
        assert!(fm.is_trapping());
        assert!(fm.should_restore());
    }

    #[test]
    fn focus_manager_menu() {
        let fm = FocusManager::menu();
        assert_eq!(fm.strategy, FocusStrategy::Initial);
        assert!(!fm.is_trapping());
        assert!(!fm.should_restore());
    }

    #[test]
    fn focus_manager_popover() {
        let fm = FocusManager::popover();
        assert_eq!(fm.strategy, FocusStrategy::Restore);
        assert!(!fm.is_trapping());
        assert!(fm.should_restore());
    }

    #[test]
    fn focus_manager_save_restore_target() {
        let mut fm = FocusManager::dialog();
        assert!(fm.restore_target().is_none());
        fm.save_restore_target("btn-trigger");
        assert_eq!(fm.restore_target(), Some("btn-trigger"));
    }

    #[test]
    fn focus_manager_with_container() {
        let fm = FocusManager::dialog().with_container("dialog-1");
        assert_eq!(fm.container_id(), Some("dialog-1"));
    }

    #[test]
    fn focus_instruction_trap() {
        let fm = FocusManager::dialog();
        let instruction = fm.trap("container-1");
        assert_eq!(
            instruction,
            FocusInstruction::Trap {
                container_id: "container-1".to_string()
            }
        );
    }

    #[test]
    fn focus_instruction_restore() {
        let mut fm = FocusManager::dialog();
        fm.save_restore_target("trigger-btn");
        let instruction = fm.restore();
        assert_eq!(
            instruction,
            FocusInstruction::FocusElement {
                element_id: "trigger-btn".to_string()
            }
        );
    }

    #[test]
    fn focus_instruction_restore_no_target() {
        let fm = FocusManager::dialog();
        let instruction = fm.restore();
        assert_eq!(instruction, FocusInstruction::Release);
    }
}
