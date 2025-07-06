import React from "react";
import { Search, Settings, X } from "lucide-react";

interface HeaderProps {
  searchQuery: string;
  onSearchChange: (query: string) => void;
  onSettingsToggle: () => void;
  onHide: () => void;
  searchInputRef: React.RefObject<HTMLInputElement>;
}

const Header: React.FC<HeaderProps> = ({
  searchQuery,
  onSearchChange,
  onSettingsToggle,
  onHide,
  searchInputRef,
}) => {
  return (
    <div className="header">
      <div className="search-container">
        <Search className="search-icon" size={16} />
        <input
          ref={searchInputRef}
          type="text"
          placeholder="Search clipboard history..."
          value={searchQuery}
          onChange={(e) => onSearchChange(e.target.value)}
          className="search-input"
        />
      </div>
      <div className="header-actions">
        <button
          onClick={onSettingsToggle}
          className="icon-button"
          title="Settings (Ctrl+I)"
        >
          <Settings size={16} />
        </button>
        <button onClick={onHide} className="icon-button" title="Close (Esc)">
          <X size={16} />
        </button>
      </div>
    </div>
  );
};

export default Header;
