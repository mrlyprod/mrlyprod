import { Symbol } from "./Glyphs";

export function Search({
  value,
  onChange,
  placeholder = "Search",
  onClear,
}: {
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  onClear?: () => void;
}) {
  return (
    <div className="search" role="search">
      <span className="search-icon" aria-hidden="true">
        <Symbol name="search" size="var(--font-lg)" />
      </span>
      <input
        className="search-input"
        type="search"
        value={value}
        placeholder={placeholder}
        aria-label={placeholder}
        onChange={(event) => onChange(event.target.value)}
      />
      {value && (
        <button
          type="button"
          className="search-clear"
          aria-label="Clear"
          onClick={() => {
            onChange("");
            onClear?.();
          }}
        >
          <Symbol name="close" size="var(--font-size)" />
        </button>
      )}
    </div>
  );
}
