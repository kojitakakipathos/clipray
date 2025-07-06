import React from "react";
import { AppConfig } from "../types";

interface SettingsProps {
  config: AppConfig;
  onConfigChange: (config: AppConfig) => void;
  onSave: (config: AppConfig) => void;
  onCancel: () => void;
}

const Settings: React.FC<SettingsProps> = ({
  config,
  onConfigChange,
  onSave,
  onCancel,
}) => {
  return (
    <div className="settings-panel">
      <h3>Settings</h3>
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
      </div>
    </div>
  );
};

export default Settings;
