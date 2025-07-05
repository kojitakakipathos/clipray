import React from "react";

const EmptyState: React.FC = () => {
  return (
    <div className="empty-state">
      <p>クリップボード履歴がありません</p>
      <p className="empty-hint">
        何かをコピーすると、ここに表示されます
        <br />
        テキストや画像に対応しています
      </p>
    </div>
  );
};

export default EmptyState;
