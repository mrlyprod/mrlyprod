import { cx } from "./lib";

export function Input({
  value,
  onChange,
  placeholder,
  type = "text",
  disabled,
  className,
}: {
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  type?: "text" | "email" | "password" | "number" | "tel" | "url";
  disabled?: boolean;
  className?: string;
}) {
  return (
    <input
      className={cx("input", className)}
      type={type}
      value={value}
      placeholder={placeholder}
      disabled={disabled}
      onChange={(event) => onChange(event.target.value)}
    />
  );
}

export function GraphemeInput({
  value,
  onChange,
}: {
  value: string;
  onChange: (value: string) => void;
}) {
  const handle = (raw: string) => {
    if (raw.length === 0) {
      onChange("");
      return;
    }
    const segments = [...new Intl.Segmenter().segment(raw)];
    const last = segments[segments.length - 1];
    onChange(last ? last.segment : "");
  };
  return (
    <input
      className="input glyph-input"
      type="text"
      inputMode="text"
      value={value}
      onChange={(event) => handle(event.target.value)}
    />
  );
}
