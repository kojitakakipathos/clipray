import {
  THEME_NAMES,
  THEME_COLORS,
  type ThemePreset,
  type ThemeColors,
} from "../types/theme";

/**
 * Apply theme CSS variables to the document root
 * @param theme - The theme to apply
 */
export const applyThemeVariables = (theme: ThemePreset): void => {
  const colors = THEME_COLORS[theme];
  const root = document.documentElement;

  // Set CSS variables dynamically
  root.style.setProperty("--primary-color", colors.primary);
  root.style.setProperty("--secondary-color", colors.secondary);
  root.style.setProperty("--background-color", colors.background);
  root.style.setProperty("--text-color", colors.text);
  root.style.setProperty("--text-secondary", colors.textSecondary);
  root.style.setProperty(
    "--background-alpha",
    colors.backgroundAlpha.toString()
  );
  root.style.setProperty("--border-color", colors.borderColor);
  root.style.setProperty("--accent-color", colors.accent);
  root.style.setProperty("--success-color", colors.success);
  root.style.setProperty("--danger-color", colors.danger);

  // Set data-theme attribute (for CSS selectors)
  document.body.setAttribute("data-theme", theme);

  // For default theme, remove data-theme attribute to use default styles
  if (theme === THEME_NAMES.DEFAULT) {
    document.body.removeAttribute("data-theme");
  }
};

/**
 * Check if a theme is valid
 * @param theme - The theme string to check
 * @returns Whether the theme is valid
 */
export const isValidTheme = (theme: string): theme is ThemePreset => {
  return Object.values(THEME_NAMES).includes(theme as ThemePreset);
};

/**
 * Get theme color information
 * @param theme - The theme
 * @returns The theme color information
 */
export const getThemeColors = (theme: ThemePreset): ThemeColors => {
  return THEME_COLORS[theme];
};

/**
 * Check if a theme is a dark theme
 * @param theme - The theme to check
 * @returns Whether the theme is a dark theme
 */
export const isDarkTheme = (theme: ThemePreset): boolean => {
  return (
    theme === THEME_NAMES.DEEP_PURPLE ||
    theme === THEME_NAMES.MIDNIGHT_BLUE ||
    theme === THEME_NAMES.PURPLE_GRADIENT
  );
};

/**
 * Check if a theme is a light theme
 * @param theme - The theme to check
 * @returns Whether the theme is a light theme
 */
export const isLightTheme = (theme: ThemePreset): boolean => {
  return theme === THEME_NAMES.DEFAULT;
};
