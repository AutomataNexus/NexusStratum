use clap::{Parser, Subcommand};

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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { framework } => {
            println!("Initializing NexusStratum with framework: {}", framework);
            // Implementation will be added
        }
        Commands::Add { components, framework, crate_dep, all } => {
            if all {
                println!("Adding all components for framework: {}", framework);
            } else {
                println!("Adding components {:?} for framework: {}", components, framework);
            }
            if crate_dep {
                println!("  (as crate dependency)");
            }
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
            println!("NexusStratum Components:");
            println!("  Layout: Box, Stack, HStack, VStack, Grid, Center, Container, Divider, AspectRatio, ScrollArea, Resizable");
            println!("  Typography: Text, Heading, Code, Kbd, Blockquote, Link");
            println!("  Forms: Button, Checkbox, Radio, Switch, Slider, Input, Textarea, NumberInput, Select, Combobox, DatePicker, Form");
            println!("  Overlay: Dialog, AlertDialog, Drawer, Sheet, Popover, Tooltip, ContextMenu, HoverCard, Toast");
            println!("  Navigation: Tabs, Accordion, NavigationMenu, Breadcrumb, Pagination, Menu, DropdownMenu");
            println!("  Data Display: Table, DataTable, VirtualList, Tree, Badge, Tag, Avatar, Card, Progress, Skeleton, Spinner");
            println!("  Feedback: Alert, Banner, EmptyState, ErrorBoundary");
            println!("  Utility: Portal, VisuallyHidden, FocusScope, Separator");
        }
        Commands::Docs { component } => {
            println!("Opening docs for component: {}", component);
        }
        Commands::Explorer => {
            println!("Starting NexusStratum Component Explorer...");
        }
    }
}
