//! Semantic color scale and palettes.

use serde::{Deserialize, Serialize};

use crate::token::{ColorPalette, ColorToken, Hsl};

/// Complete color scale with semantic tokens and color palettes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorScale {
    // Semantic colors
    pub background: ColorToken,
    pub foreground: ColorToken,
    pub card: ColorToken,
    pub card_fg: ColorToken,
    pub popover: ColorToken,
    pub popover_fg: ColorToken,
    pub primary: ColorToken,
    pub primary_fg: ColorToken,
    pub secondary: ColorToken,
    pub secondary_fg: ColorToken,
    pub muted: ColorToken,
    pub muted_fg: ColorToken,
    pub accent: ColorToken,
    pub accent_fg: ColorToken,
    pub destructive: ColorToken,
    pub destructive_fg: ColorToken,
    pub border: ColorToken,
    pub input: ColorToken,
    pub ring: ColorToken,
    // Palettes
    pub gray: ColorPalette,
    pub red: ColorPalette,
    pub orange: ColorPalette,
    pub yellow: ColorPalette,
    pub green: ColorPalette,
    pub teal: ColorPalette,
    pub blue: ColorPalette,
    pub indigo: ColorPalette,
    pub violet: ColorPalette,
    pub pink: ColorPalette,
}

// ---------------------------------------------------------------------------
// Shared palette definitions
// ---------------------------------------------------------------------------

pub(crate) fn gray_palette() -> ColorPalette {
    ColorPalette {
        steps: [
            Hsl::new(0.0, 0.0, 95.0),
            Hsl::new(0.0, 0.0, 90.0),
            Hsl::new(0.0, 0.0, 83.0),
            Hsl::new(0.0, 0.0, 64.0),
            Hsl::new(0.0, 0.0, 45.0),
            Hsl::new(0.0, 0.0, 32.0),
            Hsl::new(0.0, 0.0, 25.0),
            Hsl::new(0.0, 0.0, 15.0),
            Hsl::new(0.0, 0.0, 9.0),
        ],
    }
}

pub(crate) fn red_palette() -> ColorPalette {
    ColorPalette {
        steps: [
            Hsl::new(0.0, 86.0, 97.0),
            Hsl::new(0.0, 93.0, 94.0),
            Hsl::new(0.0, 96.0, 89.0),
            Hsl::new(0.0, 94.0, 82.0),
            Hsl::new(0.0, 84.0, 60.0),
            Hsl::new(0.0, 72.0, 51.0),
            Hsl::new(0.0, 74.0, 42.0),
            Hsl::new(0.0, 70.0, 35.0),
            Hsl::new(0.0, 63.0, 31.0),
        ],
    }
}

pub(crate) fn orange_palette() -> ColorPalette {
    ColorPalette {
        steps: [
            Hsl::new(33.0, 100.0, 96.0),
            Hsl::new(34.0, 100.0, 92.0),
            Hsl::new(32.0, 98.0, 83.0),
            Hsl::new(31.0, 97.0, 72.0),
            Hsl::new(27.0, 96.0, 61.0),
            Hsl::new(25.0, 95.0, 53.0),
            Hsl::new(21.0, 90.0, 48.0),
            Hsl::new(17.0, 88.0, 40.0),
            Hsl::new(15.0, 75.0, 28.0),
        ],
    }
}

pub(crate) fn yellow_palette() -> ColorPalette {
    ColorPalette {
        steps: [
            Hsl::new(55.0, 92.0, 95.0),
            Hsl::new(55.0, 97.0, 88.0),
            Hsl::new(53.0, 98.0, 77.0),
            Hsl::new(50.0, 98.0, 64.0),
            Hsl::new(48.0, 96.0, 53.0),
            Hsl::new(45.0, 93.0, 47.0),
            Hsl::new(41.0, 96.0, 40.0),
            Hsl::new(35.0, 92.0, 33.0),
            Hsl::new(32.0, 81.0, 29.0),
        ],
    }
}

pub(crate) fn green_palette() -> ColorPalette {
    ColorPalette {
        steps: [
            Hsl::new(138.0, 76.0, 97.0),
            Hsl::new(141.0, 84.0, 93.0),
            Hsl::new(141.0, 79.0, 85.0),
            Hsl::new(142.0, 77.0, 73.0),
            Hsl::new(142.0, 69.0, 58.0),
            Hsl::new(142.0, 71.0, 45.0),
            Hsl::new(142.0, 76.0, 36.0),
            Hsl::new(142.0, 72.0, 29.0),
            Hsl::new(144.0, 61.0, 20.0),
        ],
    }
}

pub(crate) fn teal_palette() -> ColorPalette {
    ColorPalette {
        steps: [
            Hsl::new(166.0, 76.0, 97.0),
            Hsl::new(167.0, 85.0, 89.0),
            Hsl::new(168.0, 84.0, 78.0),
            Hsl::new(171.0, 77.0, 64.0),
            Hsl::new(172.0, 66.0, 50.0),
            Hsl::new(173.0, 80.0, 40.0),
            Hsl::new(175.0, 84.0, 32.0),
            Hsl::new(175.0, 77.0, 26.0),
            Hsl::new(176.0, 69.0, 22.0),
        ],
    }
}

pub(crate) fn blue_palette() -> ColorPalette {
    ColorPalette {
        steps: [
            Hsl::new(214.0, 100.0, 97.0),
            Hsl::new(214.0, 95.0, 93.0),
            Hsl::new(213.0, 97.0, 87.0),
            Hsl::new(212.0, 96.0, 78.0),
            Hsl::new(213.0, 94.0, 68.0),
            Hsl::new(217.0, 91.0, 60.0),
            Hsl::new(221.0, 83.0, 53.0),
            Hsl::new(224.0, 76.0, 48.0),
            Hsl::new(226.0, 71.0, 40.0),
        ],
    }
}

pub(crate) fn indigo_palette() -> ColorPalette {
    ColorPalette {
        steps: [
            Hsl::new(226.0, 100.0, 97.0),
            Hsl::new(226.0, 100.0, 94.0),
            Hsl::new(228.0, 96.0, 89.0),
            Hsl::new(230.0, 94.0, 82.0),
            Hsl::new(234.0, 89.0, 74.0),
            Hsl::new(239.0, 84.0, 67.0),
            Hsl::new(243.0, 75.0, 59.0),
            Hsl::new(245.0, 58.0, 51.0),
            Hsl::new(244.0, 55.0, 41.0),
        ],
    }
}

pub(crate) fn violet_palette() -> ColorPalette {
    ColorPalette {
        steps: [
            Hsl::new(250.0, 100.0, 98.0),
            Hsl::new(251.0, 91.0, 95.0),
            Hsl::new(251.0, 95.0, 92.0),
            Hsl::new(252.0, 95.0, 85.0),
            Hsl::new(255.0, 92.0, 76.0),
            Hsl::new(258.0, 90.0, 66.0),
            Hsl::new(262.0, 83.0, 58.0),
            Hsl::new(263.0, 70.0, 50.0),
            Hsl::new(263.0, 69.0, 42.0),
        ],
    }
}

pub(crate) fn pink_palette() -> ColorPalette {
    ColorPalette {
        steps: [
            Hsl::new(327.0, 73.0, 97.0),
            Hsl::new(326.0, 78.0, 95.0),
            Hsl::new(326.0, 85.0, 90.0),
            Hsl::new(327.0, 87.0, 82.0),
            Hsl::new(329.0, 86.0, 70.0),
            Hsl::new(330.0, 81.0, 60.0),
            Hsl::new(333.0, 71.0, 51.0),
            Hsl::new(335.0, 78.0, 42.0),
            Hsl::new(336.0, 74.0, 35.0),
        ],
    }
}

/// Shared palettes used by all built-in themes.
pub(crate) fn shared_palettes() -> (
    ColorPalette,
    ColorPalette,
    ColorPalette,
    ColorPalette,
    ColorPalette,
    ColorPalette,
    ColorPalette,
    ColorPalette,
    ColorPalette,
    ColorPalette,
) {
    (
        gray_palette(),
        red_palette(),
        orange_palette(),
        yellow_palette(),
        green_palette(),
        teal_palette(),
        blue_palette(),
        indigo_palette(),
        violet_palette(),
        pink_palette(),
    )
}

// ---------------------------------------------------------------------------
// Default (neutral) color scale  -- similar to shadcn/ui neutral theme
// ---------------------------------------------------------------------------

impl Default for ColorScale {
    fn default() -> Self {
        let (gray, red, orange, yellow, green, teal, blue, indigo, violet, pink) =
            shared_palettes();

        Self {
            background: ColorToken {
                light: Hsl::new(0.0, 0.0, 100.0),
                dark: Hsl::new(222.2, 84.0, 4.9),
            },
            foreground: ColorToken {
                light: Hsl::new(222.2, 84.0, 4.9),
                dark: Hsl::new(210.0, 40.0, 98.0),
            },
            card: ColorToken {
                light: Hsl::new(0.0, 0.0, 100.0),
                dark: Hsl::new(222.2, 84.0, 4.9),
            },
            card_fg: ColorToken {
                light: Hsl::new(222.2, 84.0, 4.9),
                dark: Hsl::new(210.0, 40.0, 98.0),
            },
            popover: ColorToken {
                light: Hsl::new(0.0, 0.0, 100.0),
                dark: Hsl::new(222.2, 84.0, 4.9),
            },
            popover_fg: ColorToken {
                light: Hsl::new(222.2, 84.0, 4.9),
                dark: Hsl::new(210.0, 40.0, 98.0),
            },
            primary: ColorToken {
                light: Hsl::new(222.2, 47.4, 11.2),
                dark: Hsl::new(210.0, 40.0, 98.0),
            },
            primary_fg: ColorToken {
                light: Hsl::new(210.0, 40.0, 98.0),
                dark: Hsl::new(222.2, 47.4, 11.2),
            },
            secondary: ColorToken {
                light: Hsl::new(210.0, 40.0, 96.1),
                dark: Hsl::new(217.2, 32.6, 17.5),
            },
            secondary_fg: ColorToken {
                light: Hsl::new(222.2, 47.4, 11.2),
                dark: Hsl::new(210.0, 40.0, 98.0),
            },
            muted: ColorToken {
                light: Hsl::new(210.0, 40.0, 96.1),
                dark: Hsl::new(217.2, 32.6, 17.5),
            },
            muted_fg: ColorToken {
                light: Hsl::new(215.4, 16.3, 46.9),
                dark: Hsl::new(215.0, 20.2, 65.1),
            },
            accent: ColorToken {
                light: Hsl::new(210.0, 40.0, 96.1),
                dark: Hsl::new(217.2, 32.6, 17.5),
            },
            accent_fg: ColorToken {
                light: Hsl::new(222.2, 47.4, 11.2),
                dark: Hsl::new(210.0, 40.0, 98.0),
            },
            destructive: ColorToken {
                light: Hsl::new(0.0, 84.2, 60.2),
                dark: Hsl::new(0.0, 62.8, 30.6),
            },
            destructive_fg: ColorToken {
                light: Hsl::new(210.0, 40.0, 98.0),
                dark: Hsl::new(0.0, 0.0, 98.0),
            },
            border: ColorToken {
                light: Hsl::new(214.3, 31.8, 91.4),
                dark: Hsl::new(217.2, 32.6, 17.5),
            },
            input: ColorToken {
                light: Hsl::new(214.3, 31.8, 91.4),
                dark: Hsl::new(217.2, 32.6, 17.5),
            },
            ring: ColorToken {
                light: Hsl::new(222.2, 84.0, 4.9),
                dark: Hsl::new(212.7, 26.8, 83.9),
            },
            gray,
            red,
            orange,
            yellow,
            green,
            teal,
            blue,
            indigo,
            violet,
            pink,
        }
    }
}
