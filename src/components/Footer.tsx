import React from "react";

interface FooterProps {
  hotkey: string;
  itemCount: number;
}

const Footer: React.FC<FooterProps> = ({ hotkey, itemCount }) => {
  return (
    <div className="footer">
      <span className="footer-text">
        {hotkey} to launch | ↑↓ to select | Enter to copy & close | Esc to close
        | {itemCount} items
      </span>
    </div>
  );
};

export default Footer;
