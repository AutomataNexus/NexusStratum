//! Live region utilities for dynamic content announcements.

use stratum_core::AriaLive;

/// Describes how to announce dynamic content changes to assistive technology.
#[derive(Debug, Clone)]
pub struct LiveRegion {
    /// The politeness level of announcements.
    pub politeness: AriaLive,
    /// Whether the entire region should be announced as a whole.
    pub atomic: bool,
}

impl LiveRegion {
    /// Create a polite live region (waits for the user to be idle).
    pub fn polite() -> Self {
        Self {
            politeness: AriaLive::Polite,
            atomic: true,
        }
    }

    /// Create an assertive live region (interrupts the user immediately).
    pub fn assertive() -> Self {
        Self {
            politeness: AriaLive::Assertive,
            atomic: true,
        }
    }

    /// Create an announcement instruction for the given message.
    pub fn announce(&self, message: &str) -> Announcement {
        Announcement {
            message: message.to_string(),
            politeness: self.politeness,
            atomic: self.atomic,
        }
    }
}

/// An announcement instruction to be delivered to assistive technology.
#[derive(Debug, Clone, PartialEq)]
pub struct Announcement {
    /// The text content to announce.
    pub message: String,
    /// The politeness level of the announcement.
    pub politeness: AriaLive,
    /// Whether the region is atomic (announce all content together).
    pub atomic: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polite_region_defaults() {
        let region = LiveRegion::polite();
        assert_eq!(region.politeness, AriaLive::Polite);
        assert!(region.atomic);
    }

    #[test]
    fn assertive_region_defaults() {
        let region = LiveRegion::assertive();
        assert_eq!(region.politeness, AriaLive::Assertive);
        assert!(region.atomic);
    }

    #[test]
    fn announce_creates_correct_announcement() {
        let region = LiveRegion::polite();
        let announcement = region.announce("Item added to cart");

        assert_eq!(announcement.message, "Item added to cart");
        assert_eq!(announcement.politeness, AriaLive::Polite);
        assert!(announcement.atomic);
    }

    #[test]
    fn assertive_announce() {
        let region = LiveRegion::assertive();
        let announcement = region.announce("Error: form is invalid");

        assert_eq!(announcement.message, "Error: form is invalid");
        assert_eq!(announcement.politeness, AriaLive::Assertive);
        assert!(announcement.atomic);
    }

    #[test]
    fn non_atomic_region() {
        let mut region = LiveRegion::polite();
        region.atomic = false;
        let announcement = region.announce("Updated");

        assert!(!announcement.atomic);
    }

    #[test]
    fn empty_message() {
        let region = LiveRegion::polite();
        let announcement = region.announce("");
        assert_eq!(announcement.message, "");
    }
}
