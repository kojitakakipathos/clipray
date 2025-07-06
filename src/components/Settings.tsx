import React, { useState } from "react";
import {
  AppConfig,
  ThemeConfig,
  THEME_ORDER,
  THEME_DISPLAY_NAMES,
  THEME_PREVIEW_COLORS,
} from "../types";

interface SettingsProps {
  config: AppConfig;
  onConfigChange: (config: AppConfig) => void;
  onSave: (config: AppConfig) => void;
  onCancel: () => void;
  onExit: () => void;
}

const Settings: React.FC<SettingsProps> = ({
  config,
  onConfigChange,
  onSave,
  onCancel,
  onExit,
}) => {
  const [isThemeExpanded, setIsThemeExpanded] = useState(false);

  const handleThemeChange = (preset: ThemeConfig["preset"]) => {
    onConfigChange({
      ...config,
      theme: {
        ...config.theme,
        preset,
      },
    });
  };

  const getPreviewStyle = (preset: ThemeConfig["preset"]) => {
    const colors = THEME_PREVIEW_COLORS[preset];

    return {
      background: `linear-gradient(135deg, ${colors.primary} 0%, ${colors.secondary} 100%)`,
      border: preset === "default" ? "1px solid #e0e0e0" : "none",
    };
  };

  return (
    <div className="settings-panel">
      <h3>Settings</h3>

      <div className="theme-section">
        <h4>Theme</h4>
        <div
          className="theme-section-header"
          onClick={() => setIsThemeExpanded(!isThemeExpanded)}
        >
          <span className="current-theme">
            {THEME_DISPLAY_NAMES[config.theme.preset]}
          </span>
          <span className={`expand-icon ${isThemeExpanded ? "expanded" : ""}`}>
            ▼
          </span>
        </div>
        {isThemeExpanded && (
          <div className="theme-presets">
            {THEME_ORDER.map((preset) => (
              <div
                key={preset}
                className={`theme-card ${
                  config.theme.preset === preset ? "selected" : ""
                }`}
                onClick={() => handleThemeChange(preset)}
              >
                <div
                  className="theme-preview"
                  style={getPreviewStyle(preset)}
                />
                <span>{THEME_DISPLAY_NAMES[preset]}</span>
              </div>
            ))}
          </div>
        )}
      </div>

      <div className="setting-item">
        <label>History Limit</label>
        <input
          type="number"
          value={config.max_history_count}
          onChange={(e) =>
            onConfigChange({
              ...config,
              max_history_count: parseInt(e.target.value) || 50,
            })
          }
          min="1"
          max="1000"
        />
      </div>
      <div className="setting-item">
        <label>Hotkey:</label>
        <input
          type="text"
          value={config.hotkey}
          onChange={(e) =>
            onConfigChange({ ...config, hotkey: e.target.value })
          }
          placeholder="CommandOrControl+Shift+V"
        />
        <small>Examples: CommandOrControl+Shift+V, Alt+V, Ctrl+Space</small>
      </div>
      <div className="setting-actions">
        <button onClick={() => onSave(config)} className="save-button">
          Save
        </button>
        <button onClick={onCancel} className="cancel-button">
          Cancel
        </button>
        <button onClick={onExit} className="exit-button">
          Exit App
        </button>
      </div>
    </div>
  );
};

export default Settings;
