import React from "react";

interface EmptyStateProps {
  activeTab?: "pinned" | "history";
}

const EmptyState: React.FC<EmptyStateProps> = ({ activeTab = "history" }) => {
  const message =
    activeTab === "pinned" ? "No pinned items" : "No clipboard history";

  return (
    <div className="empty-state">
      <p>{message}</p>
    </div>
  );
};

export default EmptyState;
