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
      <h3>設定</h3>
      <div className="setting-item">
        <label>最大履歴数:</label>
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
        <label>ホットキー:</label>
        <input
          type="text"
          value={config.hotkey}
          onChange={(e) =>
            onConfigChange({ ...config, hotkey: e.target.value })
          }
          placeholder="CommandOrControl+Shift+V"
        />
        <small>例: CommandOrControl+Shift+V, Alt+V, Ctrl+Space</small>
      </div>
      <div className="setting-actions">
        <button onClick={() => onSave(config)} className="save-button">
          保存
        </button>
        <button onClick={onCancel} className="cancel-button">
          キャンセル
        </button>
      </div>
    </div>
  );
};

export default Settings;
