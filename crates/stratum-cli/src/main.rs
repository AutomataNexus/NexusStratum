use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "stratum", about = "NexusStratum CLI — scaffolding, themes, and component management")]
#[command(version, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize NexusStratum in a project
    Init {
        /// Target framework
        #[arg(long, default_value = "leptos")]
        framework: String,
    },
    /// Add a component to your project
    Add {
        /// Component names to add
        components: Vec<String>,
        /// Target framework
        #[arg(long, default_value = "leptos")]
        framework: String,
        /// Add as crate dependency instead of copying source
        #[arg(long)]
        crate_dep: bool,
        /// Add all components
        #[arg(long)]
        all: bool,
    },
    /// Theme management
    Theme {
        #[command(subcommand)]
        command: ThemeCommands,
    },
    /// Show component diff between versions
    Diff {
        /// Component name
        component: String,
        /// From version
        #[arg(long)]
        from: String,
        /// To version
        #[arg(long)]
        to: String,
    },
    /// List all available components
    List,
    /// Open component documentation
    Docs {
        /// Component name
        component: String,
    },
    /// Start the component explorer
    Explorer,
}

#[derive(Subcommand)]
enum ThemeCommands {
    /// List available themes
    List,
    /// Create a new theme
    Create {
        /// Theme name
        name: String,
        /// Base theme
        #[arg(long, default_value = "default")]
        base: String,
    },
    /// Apply a theme to the project
    Apply {
        /// Theme name
        name: String,
    },
}

/// Built-in component registry, organized by category.
const COMPONENT_REGISTRY: &[(&str, &[&str])] = &[
    ("Layout", &["Box", "Stack", "HStack", "VStack", "Grid", "Center", "Container", "Divider", "AspectRatio", "ScrollArea", "Resizable"]),
    ("Typography", &["Text", "Heading", "Code", "Kbd", "Blockquote", "Link"]),
    ("Forms", &["Button", "Checkbox", "Radio", "Switch", "Slider", "Input", "Textarea", "NumberInput", "Select", "Combobox", "DatePicker", "Form"]),
    ("Overlay", &["Dialog", "AlertDialog", "Drawer", "Sheet", "Popover", "Tooltip", "ContextMenu", "HoverCard", "Toast"]),
    ("Navigation", &["Tabs", "Accordion", "NavigationMenu", "Breadcrumb", "Pagination", "Menu", "DropdownMenu"]),
    ("Data Display", &["Table", "DataTable", "VirtualList", "Tree", "Badge", "Tag", "Avatar", "Card", "Progress", "Skeleton", "Spinner"]),
    ("Feedback", &["Alert", "Banner", "EmptyState", "ErrorBoundary"]),
    ("Utility", &["Portal", "VisuallyHidden", "FocusScope", "Separator"]),
];

/// Check if a component name exists in the registry (case-insensitive).
fn find_component(name: &str) -> Option<(&'static str, &'static str)> {
    let lower = name.to_lowercase();
    for (category, components) in COMPONENT_REGISTRY {
        for comp in *components {
            if comp.to_lowercase() == lower {
                return Some((category, comp));
            }
        }
    }
    None
}

fn init_project(framework: &str) {
    println!("Initializing NexusStratum with framework: {}", framework);

    // Create stratum.toml
    let stratum_toml = format!(
        r#"[project]
framework = "{}"

[components]
# Components will be listed here as they are added.

[theme]
name = "default"
"#,
        framework
    );

    if Path::new("stratum.toml").exists() {
        println!("  stratum.toml already exists, skipping.");
    } else {
        fs::write("stratum.toml", stratum_toml).expect("Failed to write stratum.toml");
        println!("  Created stratum.toml");
    }

    // Add a comment to Cargo.toml if it exists
    if Path::new("Cargo.toml").exists() {
        let contents = fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");
        if !contents.contains("# NexusStratum") {
            let updated = format!(
                "{}\n# NexusStratum: run `stratum add <component>` to add components.\n",
                contents.trim_end()
            );
            fs::write("Cargo.toml", updated).expect("Failed to write Cargo.toml");
            println!("  Added NexusStratum comment to Cargo.toml");
        } else {
            println!("  Cargo.toml already has NexusStratum marker, skipping.");
        }
    }

    // Create src/components/mod.rs
    let components_dir = Path::new("src/components");
    if components_dir.exists() {
        println!("  src/components/ already exists, skipping.");
    } else {
        fs::create_dir_all(components_dir).expect("Failed to create src/components/");
        fs::write(
            components_dir.join("mod.rs"),
            "//! NexusStratum components for this project.\n",
        )
        .expect("Failed to write src/components/mod.rs");
        println!("  Created src/components/mod.rs");
    }

    println!("NexusStratum initialized successfully!");
}

fn add_components(components: &[String], framework: &str, crate_dep: bool, all: bool) {
    if all {
        println!("Adding all components for framework: {}", framework);
        for (category, comps) in COMPONENT_REGISTRY {
            for comp in *comps {
                println!("  Added {} ({})", comp, category);
            }
        }
        if crate_dep {
            println!("  (as crate dependencies)");
        }
        return;
    }

    if components.is_empty() {
        eprintln!("Error: no component names provided. Use `stratum add <name>` or `stratum add --all`.");
        std::process::exit(1);
    }

    for name in components {
        match find_component(name) {
            Some((category, canonical)) => {
                println!("Adding component {} ({}) for framework: {}", canonical, category, framework);
                if crate_dep {
                    println!("  (as crate dependency)");
                }
            }
            None => {
                eprintln!(
                    "Error: unknown component '{}'. Run `stratum list` to see available components.",
                    name
                );
                std::process::exit(1);
            }
        }
    }
}

fn list_components() {
    println!("NexusStratum Components:");
    for (category, components) in COMPONENT_REGISTRY {
        println!("  {}: {}", category, components.join(", "));
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { framework } => {
            init_project(&framework);
        }
        Commands::Add { components, framework, crate_dep, all } => {
            add_components(&components, &framework, crate_dep, all);
        }
        Commands::Theme { command } => match command {
            ThemeCommands::List => println!("Available themes: default, slate, zinc, rose, blue, green, orange"),
            ThemeCommands::Create { name, base } => println!("Creating theme '{}' based on '{}'", name, base),
            ThemeCommands::Apply { name } => println!("Applying theme '{}'", name),
        },
        Commands::Diff { component, from, to } => {
            println!("Showing diff for {} from {} to {}", component, from, to);
        }
        Commands::List => {
            list_components();
        }
        Commands::Docs { component } => {
            println!("Opening docs for component: {}", component);
        }
        Commands::Explorer => {
            println!("Starting NexusStratum Component Explorer...");
        }
    }
}
