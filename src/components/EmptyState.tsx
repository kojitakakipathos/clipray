import React from "react";

const EmptyState: React.FC = () => {
  return (
    <div className="empty-state">
      <p>No clipboard history</p>
      <p className="empty-hint">
        Copy something and it will appear here
        <br />
        Supports text and images
      </p>
    </div>
  );
};

export default EmptyState;
