//! Static site generator for the component explorer.

use stratum_sdk::metadata::{ComponentMeta, RegistryMeta};
use std::fs;
use std::path::Path;

/// The component explorer generator.
pub struct Explorer {
    registry: RegistryMeta,
}

impl Explorer {
    /// Create an explorer from the built-in component registry.
    pub fn new() -> Self {
        Self {
            registry: RegistryMeta::builtin(),
        }
    }

    /// Create an explorer from a custom registry.
    pub fn from_registry(registry: RegistryMeta) -> Self {
        Self { registry }
    }

    /// Generate the explorer static site into the given directory.
    pub fn generate(&self, output_dir: &str) {
        let out = Path::new(output_dir);
        fs::create_dir_all(out).expect("Failed to create output directory");

        // Generate index page
        let index_html = self.render_index();
        fs::write(out.join("index.html"), index_html).expect("Failed to write index.html");

        // Generate individual component pages
        for comp in &self.registry.components {
            let page = self.render_component(comp);
            let filename = format!("{}.html", comp.name.to_lowercase());
            fs::write(out.join(&filename), page).expect("Failed to write component page");
        }
    }

    /// Get the registry metadata as JSON.
    pub fn to_json(&self) -> String {
        self.registry.to_json()
    }

    /// Get the number of components in the registry.
    pub fn component_count(&self) -> usize {
        self.registry.components.len()
    }

    fn render_index(&self) -> String {
        let mut components_html = String::new();
        for comp in &self.registry.components {
            components_html.push_str(&format!(
                r#"<div class="comp-card">
  <h3><a href="{filename}.html">{name}</a></h3>
  <p>{desc}</p>
  <span class="category">{cat}</span>
</div>
"#,
                filename = comp.name.to_lowercase(),
                name = comp.name,
                desc = comp.description,
                cat = comp.category,
            ));
        }

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>NexusStratum Explorer</title>
<style>
body {{ font-family: Inter, sans-serif; max-width: 900px; margin: 0 auto; padding: 24px; color: #1a1a2e; }}
h1 {{ font-size: 28px; margin-bottom: 8px; }}
p.subtitle {{ color: #495057; margin-bottom: 32px; }}
.grid {{ display: grid; grid-template-columns: repeat(auto-fill, minmax(260px, 1fr)); gap: 16px; }}
.comp-card {{ border: 1px solid #e9ecef; border-radius: 10px; padding: 20px; }}
.comp-card h3 {{ margin: 0 0 6px; font-size: 16px; }}
.comp-card h3 a {{ color: #4263eb; text-decoration: none; }}
.comp-card p {{ margin: 0 0 8px; font-size: 13px; color: #495057; }}
.category {{ font-size: 11px; padding: 2px 8px; background: #f1f3f5; border-radius: 100px; color: #868e96; }}
</style>
</head>
<body>
<h1>NexusStratum Explorer</h1>
<p class="subtitle">{count} components — browse props, ARIA roles, and keyboard patterns.</p>
<div class="grid">
{components}
</div>
</body>
</html>"#,
            count = self.registry.components.len(),
            components = components_html,
        )
    }

    fn render_component(&self, comp: &ComponentMeta) -> String {
        let mut props_html = String::new();
        for prop in &comp.props {
            let values = if prop.values.is_empty() {
                String::new()
            } else {
                format!(" ({})", prop.values.join(", "))
            };
            props_html.push_str(&format!(
                "<tr><td><code>{}</code></td><td>{}</td><td>{}</td><td>{}{}</td></tr>\n",
                prop.name,
                prop.prop_type,
                prop.default.as_deref().unwrap_or("—"),
                prop.description,
                values,
            ));
        }

        let mut keyboard_html = String::new();
        for kb in &comp.keyboard {
            keyboard_html.push_str(&format!(
                "<tr><td><kbd>{}</kbd></td><td>{}</td></tr>\n",
                kb.key, kb.action
            ));
        }

        let aria = comp
            .aria_role
            .as_deref()
            .map(|r| format!("<code>role=\"{}\"</code>", r))
            .unwrap_or_else(|| "None".to_string());

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{name} — NexusStratum Explorer</title>
<style>
body {{ font-family: Inter, sans-serif; max-width: 800px; margin: 0 auto; padding: 24px; color: #1a1a2e; }}
h1 {{ font-size: 28px; margin-bottom: 4px; }}
.desc {{ color: #495057; margin-bottom: 24px; }}
.meta {{ display: flex; gap: 8px; margin-bottom: 24px; }}
.tag {{ font-size: 11px; padding: 3px 8px; background: #f1f3f5; border-radius: 100px; color: #495057; border: 1px solid #e9ecef; }}
h2 {{ font-size: 18px; margin: 32px 0 12px; border-bottom: 1px solid #e9ecef; padding-bottom: 6px; }}
table {{ width: 100%; border-collapse: collapse; font-size: 14px; }}
th {{ text-align: left; padding: 8px; border-bottom: 2px solid #e9ecef; font-size: 13px; }}
td {{ padding: 8px; border-bottom: 1px solid #e9ecef; }}
td:first-child {{ font-family: 'JetBrains Mono', monospace; color: #4263eb; }}
kbd {{ font-family: 'JetBrains Mono', monospace; font-size: 12px; padding: 2px 6px; background: #f1f3f5; border: 1px solid #e9ecef; border-radius: 4px; }}
a {{ color: #4263eb; text-decoration: none; }}
</style>
</head>
<body>
<p><a href="index.html">&larr; All Components</a></p>
<h1>{name}</h1>
<p class="desc">{desc}</p>
<div class="meta">
<span class="tag">{cat}</span>
<span class="tag">ARIA: {aria}</span>
</div>

<h2>Props</h2>
<table>
<thead><tr><th>Prop</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
{props}
</tbody>
</table>

<h2>Keyboard</h2>
<table>
<thead><tr><th>Key</th><th>Action</th></tr></thead>
<tbody>
{keyboard}
</tbody>
</table>
</body>
</html>"#,
            name = comp.name,
            desc = comp.description,
            cat = comp.category,
            aria = aria,
            props = props_html,
            keyboard = keyboard_html,
        )
    }
}

impl Default for Explorer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explorer_new() {
        let explorer = Explorer::new();
        assert!(explorer.component_count() >= 3);
    }

    #[test]
    fn explorer_to_json() {
        let explorer = Explorer::new();
        let json = explorer.to_json();
        assert!(json.contains("Button"));
        assert!(json.contains("Dialog"));
    }

    #[test]
    fn render_index_contains_components() {
        let explorer = Explorer::new();
        let html = explorer.render_index();
        assert!(html.contains("NexusStratum Explorer"));
        assert!(html.contains("Button"));
        assert!(html.contains("button.html"));
    }

    #[test]
    fn render_component_page() {
        let reg = RegistryMeta::builtin();
        let explorer = Explorer::from_registry(reg);
        let button = explorer.registry.find("Button").unwrap();
        let html = explorer.render_component(button);
        assert!(html.contains("<h1>Button</h1>"));
        assert!(html.contains("variant"));
        assert!(html.contains("Enter"));
        assert!(html.contains("role=\"button\""));
    }

    #[test]
    fn generate_creates_files() {
        let explorer = Explorer::new();
        let dir = "/tmp/stratum-explorer-test";
        let _ = std::fs::remove_dir_all(dir);
        explorer.generate(dir);
        assert!(Path::new(dir).join("index.html").exists());
        assert!(Path::new(dir).join("button.html").exists());
        assert!(Path::new(dir).join("dialog.html").exists());
        let _ = std::fs::remove_dir_all(dir);
    }
}
