use serde::{Deserialize, Serialize};

/// Complete set of ARIA attributes for a component.
///
/// Every interactive component in NexusStratum produces an `AriaAttributes`
/// as part of its `RenderOutput`. Framework adapters apply these to the
/// rendered DOM elements.
///
/// All fields are `Option` — only set attributes are rendered to the DOM.
/// This prevents polluting the accessibility tree with unnecessary attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AriaAttributes {
    /// The ARIA role of the element.
    pub role: Option<AriaRole>,

    /// A human-readable label for the element.
    pub label: Option<String>,

    /// ID of the element that labels this element.
    pub labelledby: Option<String>,

    /// ID of the element that describes this element.
    pub describedby: Option<String>,

    /// Whether a section is expanded or collapsed.
    pub expanded: Option<bool>,

    /// Whether an option is selected.
    pub selected: Option<bool>,

    /// Checked state (supports indeterminate via TriState).
    pub checked: Option<TriState>,

    /// Whether the element is disabled.
    pub disabled: Option<bool>,

    /// Whether the element is required.
    pub required: Option<bool>,

    /// Whether the element's value is invalid.
    pub invalid: Option<bool>,

    /// Live region politeness setting.
    pub live: Option<AriaLive>,

    /// Whether the entire live region should be announced.
    pub atomic: Option<bool>,

    /// ID of the element this element controls.
    pub controls: Option<String>,

    /// ID of elements owned by this element (not DOM children).
    pub owns: Option<String>,

    /// Type of popup triggered by this element.
    pub haspopup: Option<AriaHasPopup>,

    /// Heading level (1-6).
    pub level: Option<u8>,

    /// Orientation of the element (horizontal or vertical).
    pub orientation: Option<Orientation>,

    /// Whether the element is read-only.
    pub readonly: Option<bool>,

    /// Whether multiple items can be selected.
    pub multiselectable: Option<bool>,

    /// Minimum value for range widgets.
    pub valuemin: Option<f64>,

    /// Maximum value for range widgets.
    pub valuemax: Option<f64>,

    /// Current value for range widgets.
    pub valuenow: Option<f64>,

    /// Human-readable text alternative for the current value.
    pub valuetext: Option<String>,

    /// Whether the element is hidden from the accessibility tree.
    pub hidden: Option<bool>,

    /// ID of the element that is the active descendant.
    pub activedescendant: Option<String>,

    /// Whether the element is busy (loading).
    pub busy: Option<bool>,

    /// Current item in a set (e.g., "3 of 10").
    pub posinset: Option<u32>,

    /// Total number of items in a set.
    pub setsize: Option<u32>,

    /// Column count for tables/grids.
    pub colcount: Option<u32>,

    /// Column index for table cells.
    pub colindex: Option<u32>,

    /// Column span for table cells.
    pub colspan: Option<u32>,

    /// Row count for tables/grids.
    pub rowcount: Option<u32>,

    /// Row index for table rows.
    pub rowindex: Option<u32>,

    /// Row span for table cells.
    pub rowspan: Option<u32>,

    /// Sort direction for sortable columns.
    pub sort: Option<AriaSort>,

    /// Whether autocomplete is available.
    pub autocomplete: Option<AriaAutocomplete>,

    /// Current state for widgets with multiple states.
    pub current: Option<AriaCurrent>,

    /// ID of the error message element.
    pub errormessage: Option<String>,

    /// Keyboard shortcut.
    pub keyshortcuts: Option<String>,

    /// Roledescription override.
    pub roledescription: Option<String>,

    /// Whether the element is modal.
    pub modal: Option<bool>,

    /// Placeholder text.
    pub placeholder: Option<String>,
}

impl AriaAttributes {
    /// Create a new empty AriaAttributes.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the role.
    pub fn with_role(mut self, role: AriaRole) -> Self {
        self.role = Some(role);
        self
    }

    /// Set the label.
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the labelledby reference.
    pub fn with_labelledby(mut self, id: impl Into<String>) -> Self {
        self.labelledby = Some(id.into());
        self
    }

    /// Set the describedby reference.
    pub fn with_describedby(mut self, id: impl Into<String>) -> Self {
        self.describedby = Some(id.into());
        self
    }

    /// Set the expanded state.
    pub fn with_expanded(mut self, expanded: bool) -> Self {
        self.expanded = Some(expanded);
        self
    }

    /// Set the selected state.
    pub fn with_selected(mut self, selected: bool) -> Self {
        self.selected = Some(selected);
        self
    }

    /// Set the checked state.
    pub fn with_checked(mut self, checked: TriState) -> Self {
        self.checked = Some(checked);
        self
    }

    /// Set the disabled state.
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    /// Set the controls reference.
    pub fn with_controls(mut self, id: impl Into<String>) -> Self {
        self.controls = Some(id.into());
        self
    }

    /// Set the modal state.
    pub fn with_modal(mut self, modal: bool) -> Self {
        self.modal = Some(modal);
        self
    }

    /// Set the haspopup type.
    pub fn with_haspopup(mut self, popup: AriaHasPopup) -> Self {
        self.haspopup = Some(popup);
        self
    }

    /// Set the orientation.
    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    /// Collect all set attributes as key-value string pairs for rendering.
    ///
    /// Only attributes with `Some` values are included. This is used by
    /// framework adapters to apply ARIA attributes to DOM elements.
    pub fn to_attr_pairs(&self) -> Vec<(String, String)> {
        let mut pairs = Vec::new();

        if let Some(ref role) = self.role {
            pairs.push(("role".to_string(), role.as_str().to_string()));
        }
        if let Some(ref label) = self.label {
            pairs.push(("aria-label".to_string(), label.clone()));
        }
        if let Some(ref id) = self.labelledby {
            pairs.push(("aria-labelledby".to_string(), id.clone()));
        }
        if let Some(ref id) = self.describedby {
            pairs.push(("aria-describedby".to_string(), id.clone()));
        }
        if let Some(expanded) = self.expanded {
            pairs.push(("aria-expanded".to_string(), expanded.to_string()));
        }
        if let Some(selected) = self.selected {
            pairs.push(("aria-selected".to_string(), selected.to_string()));
        }
        if let Some(ref checked) = self.checked {
            pairs.push(("aria-checked".to_string(), checked.as_str().to_string()));
        }
        if let Some(disabled) = self.disabled {
            pairs.push(("aria-disabled".to_string(), disabled.to_string()));
        }
        if let Some(required) = self.required {
            pairs.push(("aria-required".to_string(), required.to_string()));
        }
        if let Some(invalid) = self.invalid {
            pairs.push(("aria-invalid".to_string(), invalid.to_string()));
        }
        if let Some(ref live) = self.live {
            pairs.push(("aria-live".to_string(), live.as_str().to_string()));
        }
        if let Some(atomic) = self.atomic {
            pairs.push(("aria-atomic".to_string(), atomic.to_string()));
        }
        if let Some(ref controls) = self.controls {
            pairs.push(("aria-controls".to_string(), controls.clone()));
        }
        if let Some(ref owns) = self.owns {
            pairs.push(("aria-owns".to_string(), owns.clone()));
        }
        if let Some(ref popup) = self.haspopup {
            pairs.push(("aria-haspopup".to_string(), popup.as_str().to_string()));
        }
        if let Some(level) = self.level {
            // ARIA heading levels must be 1-6; clamp to valid range
            let clamped = level.clamp(1, 6);
            pairs.push(("aria-level".to_string(), clamped.to_string()));
        }
        if let Some(ref orientation) = self.orientation {
            pairs.push((
                "aria-orientation".to_string(),
                orientation.as_str().to_string(),
            ));
        }
        if let Some(readonly) = self.readonly {
            pairs.push(("aria-readonly".to_string(), readonly.to_string()));
        }
        if let Some(multi) = self.multiselectable {
            pairs.push(("aria-multiselectable".to_string(), multi.to_string()));
        }
        if let Some(min) = self.valuemin {
            pairs.push(("aria-valuemin".to_string(), min.to_string()));
        }
        if let Some(max) = self.valuemax {
            pairs.push(("aria-valuemax".to_string(), max.to_string()));
        }
        if let Some(now) = self.valuenow {
            pairs.push(("aria-valuenow".to_string(), now.to_string()));
        }
        if let Some(ref text) = self.valuetext {
            pairs.push(("aria-valuetext".to_string(), text.clone()));
        }
        if let Some(hidden) = self.hidden {
            pairs.push(("aria-hidden".to_string(), hidden.to_string()));
        }
        if let Some(ref id) = self.activedescendant {
            pairs.push(("aria-activedescendant".to_string(), id.clone()));
        }
        if let Some(busy) = self.busy {
            pairs.push(("aria-busy".to_string(), busy.to_string()));
        }
        if let Some(pos) = self.posinset {
            pairs.push(("aria-posinset".to_string(), pos.to_string()));
        }
        if let Some(size) = self.setsize {
            pairs.push(("aria-setsize".to_string(), size.to_string()));
        }
        if let Some(modal) = self.modal {
            pairs.push(("aria-modal".to_string(), modal.to_string()));
        }
        if let Some(col) = self.colcount {
            pairs.push(("aria-colcount".to_string(), col.to_string()));
        }
        if let Some(col) = self.colindex {
            pairs.push(("aria-colindex".to_string(), col.to_string()));
        }
        if let Some(col) = self.colspan {
            pairs.push(("aria-colspan".to_string(), col.to_string()));
        }
        if let Some(row) = self.rowcount {
            pairs.push(("aria-rowcount".to_string(), row.to_string()));
        }
        if let Some(row) = self.rowindex {
            pairs.push(("aria-rowindex".to_string(), row.to_string()));
        }
        if let Some(row) = self.rowspan {
            pairs.push(("aria-rowspan".to_string(), row.to_string()));
        }
        if let Some(ref sort) = self.sort {
            pairs.push(("aria-sort".to_string(), sort.as_str().to_string()));
        }
        if let Some(ref ac) = self.autocomplete {
            pairs.push(("aria-autocomplete".to_string(), ac.as_str().to_string()));
        }
        if let Some(ref cur) = self.current {
            pairs.push(("aria-current".to_string(), cur.as_str().to_string()));
        }
        if let Some(ref err) = self.errormessage {
            pairs.push(("aria-errormessage".to_string(), err.clone()));
        }
        if let Some(ref ks) = self.keyshortcuts {
            pairs.push(("aria-keyshortcuts".to_string(), ks.clone()));
        }
        if let Some(ref rd) = self.roledescription {
            pairs.push(("aria-roledescription".to_string(), rd.clone()));
        }
        if let Some(ref ph) = self.placeholder {
            pairs.push(("aria-placeholder".to_string(), ph.clone()));
        }

        pairs
    }
}

/// ARIA roles as defined in the WAI-ARIA specification.
///
/// Covers all roles needed for NexusStratum's 50+ component types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AriaRole {
    Alert,
    AlertDialog,
    Button,
    Checkbox,
    Combobox,
    Dialog,
    Feed,
    Grid,
    GridCell,
    Group,
    Heading,
    Img,
    Link,
    List,
    ListItem,
    ListBox,
    Log,
    Marquee,
    Menu,
    MenuBar,
    MenuItem,
    MenuItemCheckbox,
    MenuItemRadio,
    Navigation,
    None,
    Option,
    Presentation,
    ProgressBar,
    Radio,
    RadioGroup,
    Region,
    Row,
    RowGroup,
    RowHeader,
    ScrollBar,
    Search,
    SearchBox,
    Separator,
    Slider,
    SpinButton,
    Status,
    Switch,
    Tab,
    TabList,
    TabPanel,
    Table,
    TextBox,
    Timer,
    ToolBar,
    ToolTip,
    Tree,
    TreeGrid,
    TreeItem,
    ColumnHeader,
    Cell,
    Form,
    Main,
    Banner,
    Complementary,
    ContentInfo,
    Definition,
    Document,
    Figure,
    Note,
    Term,
    Application,
}

impl AriaRole {
    /// Get the string representation for use in the `role` HTML attribute.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Alert => "alert",
            Self::AlertDialog => "alertdialog",
            Self::Button => "button",
            Self::Checkbox => "checkbox",
            Self::Combobox => "combobox",
            Self::Dialog => "dialog",
            Self::Feed => "feed",
            Self::Grid => "grid",
            Self::GridCell => "gridcell",
            Self::Group => "group",
            Self::Heading => "heading",
            Self::Img => "img",
            Self::Link => "link",
            Self::List => "list",
            Self::ListItem => "listitem",
            Self::ListBox => "listbox",
            Self::Log => "log",
            Self::Marquee => "marquee",
            Self::Menu => "menu",
            Self::MenuBar => "menubar",
            Self::MenuItem => "menuitem",
            Self::MenuItemCheckbox => "menuitemcheckbox",
            Self::MenuItemRadio => "menuitemradio",
            Self::Navigation => "navigation",
            Self::None => "none",
            Self::Option => "option",
            Self::Presentation => "presentation",
            Self::ProgressBar => "progressbar",
            Self::Radio => "radio",
            Self::RadioGroup => "radiogroup",
            Self::Region => "region",
            Self::Row => "row",
            Self::RowGroup => "rowgroup",
            Self::RowHeader => "rowheader",
            Self::ScrollBar => "scrollbar",
            Self::Search => "search",
            Self::SearchBox => "searchbox",
            Self::Separator => "separator",
            Self::Slider => "slider",
            Self::SpinButton => "spinbutton",
            Self::Status => "status",
            Self::Switch => "switch",
            Self::Tab => "tab",
            Self::TabList => "tablist",
            Self::TabPanel => "tabpanel",
            Self::Table => "table",
            Self::TextBox => "textbox",
            Self::Timer => "timer",
            Self::ToolBar => "toolbar",
            Self::ToolTip => "tooltip",
            Self::Tree => "tree",
            Self::TreeGrid => "treegrid",
            Self::TreeItem => "treeitem",
            Self::ColumnHeader => "columnheader",
            Self::Cell => "cell",
            Self::Form => "form",
            Self::Main => "main",
            Self::Banner => "banner",
            Self::Complementary => "complementary",
            Self::ContentInfo => "contentinfo",
            Self::Definition => "definition",
            Self::Document => "document",
            Self::Figure => "figure",
            Self::Note => "note",
            Self::Term => "term",
            Self::Application => "application",
        }
    }
}

/// Tri-state value for checkboxes supporting indeterminate state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TriState {
    True,
    False,
    Mixed,
}

impl TriState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::True => "true",
            Self::False => "false",
            Self::Mixed => "mixed",
        }
    }

    pub fn is_checked(&self) -> bool {
        matches!(self, Self::True)
    }

    pub fn toggle(&self) -> Self {
        match self {
            Self::True => Self::False,
            Self::False => Self::True,
            Self::Mixed => Self::True,
        }
    }
}

impl From<bool> for TriState {
    fn from(value: bool) -> Self {
        if value { Self::True } else { Self::False }
    }
}

/// ARIA live region politeness setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AriaLive {
    Off,
    Polite,
    Assertive,
}

impl AriaLive {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Polite => "polite",
            Self::Assertive => "assertive",
        }
    }
}

/// Type of popup an element triggers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AriaHasPopup {
    True,
    Menu,
    ListBox,
    Tree,
    Grid,
    Dialog,
}

impl AriaHasPopup {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::True => "true",
            Self::Menu => "menu",
            Self::ListBox => "listbox",
            Self::Tree => "tree",
            Self::Grid => "grid",
            Self::Dialog => "dialog",
        }
    }
}

/// Orientation of a widget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// Sort direction for sortable table columns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AriaSort {
    None,
    Ascending,
    Descending,
    Other,
}

impl AriaSort {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Ascending => "ascending",
            Self::Descending => "descending",
            Self::Other => "other",
        }
    }
}

/// Autocomplete behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AriaAutocomplete {
    None,
    Inline,
    List,
    Both,
}

impl AriaAutocomplete {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Inline => "inline",
            Self::List => "list",
            Self::Both => "both",
        }
    }
}

/// Current item state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AriaCurrent {
    True,
    Page,
    Step,
    Location,
    Date,
    Time,
}

impl AriaCurrent {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::True => "true",
            Self::Page => "page",
            Self::Step => "step",
            Self::Location => "location",
            Self::Date => "date",
            Self::Time => "time",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aria_attributes_default_is_empty() {
        let attrs = AriaAttributes::default();
        assert!(attrs.role.is_none());
        assert!(attrs.label.is_none());
        assert_eq!(attrs.to_attr_pairs().len(), 0);
    }

    #[test]
    fn aria_attributes_builder() {
        let attrs = AriaAttributes::new()
            .with_role(AriaRole::Button)
            .with_label("Click me")
            .with_disabled(false)
            .with_expanded(true);

        assert_eq!(attrs.role, Some(AriaRole::Button));
        assert_eq!(attrs.label, Some("Click me".to_string()));
        assert_eq!(attrs.disabled, Some(false));
        assert_eq!(attrs.expanded, Some(true));
    }

    #[test]
    fn aria_attributes_to_attr_pairs() {
        let attrs = AriaAttributes::new()
            .with_role(AriaRole::Button)
            .with_label("Save")
            .with_expanded(false);

        let pairs = attrs.to_attr_pairs();
        assert!(pairs.contains(&("role".to_string(), "button".to_string())));
        assert!(pairs.contains(&("aria-label".to_string(), "Save".to_string())));
        assert!(pairs.contains(&("aria-expanded".to_string(), "false".to_string())));
        assert_eq!(pairs.len(), 3);
    }

    #[test]
    fn aria_role_as_str() {
        assert_eq!(AriaRole::Button.as_str(), "button");
        assert_eq!(AriaRole::Dialog.as_str(), "dialog");
        assert_eq!(AriaRole::TabList.as_str(), "tablist");
        assert_eq!(AriaRole::TreeItem.as_str(), "treeitem");
        assert_eq!(AriaRole::AlertDialog.as_str(), "alertdialog");
    }

    #[test]
    fn tri_state_toggle() {
        assert_eq!(TriState::False.toggle(), TriState::True);
        assert_eq!(TriState::True.toggle(), TriState::False);
        assert_eq!(TriState::Mixed.toggle(), TriState::True);
    }

    #[test]
    fn tri_state_from_bool() {
        assert_eq!(TriState::from(true), TriState::True);
        assert_eq!(TriState::from(false), TriState::False);
    }

    #[test]
    fn aria_live_as_str() {
        assert_eq!(AriaLive::Polite.as_str(), "polite");
        assert_eq!(AriaLive::Assertive.as_str(), "assertive");
        assert_eq!(AriaLive::Off.as_str(), "off");
    }

    #[test]
    fn orientation_as_str() {
        assert_eq!(Orientation::Horizontal.as_str(), "horizontal");
        assert_eq!(Orientation::Vertical.as_str(), "vertical");
    }

    #[test]
    fn aria_attributes_serialization() {
        let attrs = AriaAttributes::new()
            .with_role(AriaRole::Checkbox)
            .with_checked(TriState::Mixed);

        let json = serde_json::to_string(&attrs).unwrap();
        let deserialized: AriaAttributes = serde_json::from_str(&json).unwrap();
        assert_eq!(attrs, deserialized);
    }

    #[test]
    fn all_aria_roles_have_str() {
        let roles = vec![
            AriaRole::Alert,
            AriaRole::AlertDialog,
            AriaRole::Button,
            AriaRole::Checkbox,
            AriaRole::Combobox,
            AriaRole::Dialog,
            AriaRole::Grid,
            AriaRole::Group,
            AriaRole::Heading,
            AriaRole::Link,
            AriaRole::List,
            AriaRole::ListBox,
            AriaRole::Menu,
            AriaRole::MenuBar,
            AriaRole::MenuItem,
            AriaRole::Navigation,
            AriaRole::ProgressBar,
            AriaRole::Radio,
            AriaRole::RadioGroup,
            AriaRole::Separator,
            AriaRole::Slider,
            AriaRole::SpinButton,
            AriaRole::Status,
            AriaRole::Switch,
            AriaRole::Tab,
            AriaRole::TabList,
            AriaRole::TabPanel,
            AriaRole::Table,
            AriaRole::TextBox,
            AriaRole::ToolTip,
            AriaRole::Tree,
            AriaRole::TreeItem,
        ];
        for role in roles {
            assert!(!role.as_str().is_empty());
        }
    }
}
