import React from "react";

interface FooterProps {
  hotkey: string;
  itemCount: number;
}

const Footer: React.FC<FooterProps> = ({ hotkey, itemCount }) => {
  return (
    <div className="footer">
      <span className="footer-text">
        {hotkey} で起動 | ↑↓で選択 | Enterでコピー&閉じる | Escで閉じる |{" "}
        {itemCount} 件
      </span>
    </div>
  );
};

export default Footer;
