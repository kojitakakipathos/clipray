// Theme constant definitions
export const THEME_NAMES = {
  DEFAULT: "default",
  PURPLE_GRADIENT: "purple-gradient",
  DEEP_PURPLE: "deep-purple",
  MIDNIGHT_BLUE: "midnight-blue",
} as const;

// Theme ENUM
export type ThemePreset = (typeof THEME_NAMES)[keyof typeof THEME_NAMES];

// Theme display names
export const THEME_DISPLAY_NAMES: Record<ThemePreset, string> = {
  [THEME_NAMES.DEFAULT]: "Default",
  [THEME_NAMES.PURPLE_GRADIENT]: "Purple Gradient",
  [THEME_NAMES.DEEP_PURPLE]: "Deep Purple",
  [THEME_NAMES.MIDNIGHT_BLUE]: "Midnight Blue",
};

// Theme CSS variable definitions
export interface ThemeColors {
  primary: string;
  secondary: string;
  background: string;
  text: string;
  textSecondary: string;
  backgroundAlpha: number;
  borderColor: string;
  accent: string;
  success: string;
  danger: string;
}

// Theme color definitions
export const THEME_COLORS: Record<ThemePreset, ThemeColors> = {
  [THEME_NAMES.DEFAULT]: {
    primary: "#ffffff",
    secondary: "#f8f9fa",
    background: "#ffffff",
    text: "#2c3e50",
    textSecondary: "rgba(44, 62, 80, 0.7)",
    backgroundAlpha: 0.95,
    borderColor: "rgba(44, 62, 80, 0.15)",
    accent: "#64b5f6",
    success: "#28a745",
    danger: "#dc3545",
  },
  [THEME_NAMES.PURPLE_GRADIENT]: {
    primary: "#667eea",
    secondary: "#764ba2",
    background: "#f5f5f7",
    text: "#ffffff",
    textSecondary: "rgba(255, 255, 255, 0.7)",
    backgroundAlpha: 0.1,
    borderColor: "rgba(255, 255, 255, 0.1)",
    accent: "#ffd700",
    success: "#2ed573",
    danger: "#ff4757",
  },
  [THEME_NAMES.DEEP_PURPLE]: {
    primary: "#0f0f23",
    secondary: "#1a1a2e",
    background: "#0a0a0a",
    text: "#ffffff",
    textSecondary: "rgba(255, 255, 255, 0.7)",
    backgroundAlpha: 0.3,
    borderColor: "rgba(255, 255, 255, 0.1)",
    accent: "#bb86fc",
    success: "#4caf50",
    danger: "#f44336",
  },
  [THEME_NAMES.MIDNIGHT_BLUE]: {
    primary: "#0c1427",
    secondary: "#1e3a5f",
    background: "#0a0a0a",
    text: "#ffffff",
    textSecondary: "rgba(255, 255, 255, 0.7)",
    backgroundAlpha: 0.3,
    borderColor: "rgba(255, 255, 255, 0.1)",
    accent: "#64b5f6",
    success: "#4caf50",
    danger: "#f44336",
  },
};

// Theme preview colors (for settings screen)
export const THEME_PREVIEW_COLORS: Record<
  ThemePreset,
  { primary: string; secondary: string }
> = {
  [THEME_NAMES.DEFAULT]: { primary: "#ffffff", secondary: "#f8f9fa" },
  [THEME_NAMES.PURPLE_GRADIENT]: { primary: "#667eea", secondary: "#764ba2" },
  [THEME_NAMES.DEEP_PURPLE]: { primary: "#0f0f23", secondary: "#1a1a2e" },
  [THEME_NAMES.MIDNIGHT_BLUE]: { primary: "#0c1427", secondary: "#1e3a5f" },
};

// Theme order
export const THEME_ORDER: ThemePreset[] = [
  THEME_NAMES.DEFAULT,
  THEME_NAMES.PURPLE_GRADIENT,
  THEME_NAMES.DEEP_PURPLE,
  THEME_NAMES.MIDNIGHT_BLUE,
];
