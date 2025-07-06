import { useEffect } from "react";
import { ThemeConfig } from "../types";
import { applyThemeVariables } from "../utils/theme";

export const useTheme = (theme: ThemeConfig) => {
  useEffect(() => {
    // Apply theme variables and data-theme attribute
    applyThemeVariables(theme.preset);
  }, [theme.preset]);
};
