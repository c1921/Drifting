export interface ThemeColors {
  foreground: number
  mutedForeground: number
  border: number
  card: number
  primary: number
  destructive: number
}

/** Extract theme colors from CSS custom properties */
export function getThemeColors(): ThemeColors {
  const s = getComputedStyle(document.documentElement)
  return {
    foreground:      parseCSSColor(s.getPropertyValue("--foreground")),
    mutedForeground: parseCSSColor(s.getPropertyValue("--muted-foreground")),
    border:          parseCSSColor(s.getPropertyValue("--border")),
    card:            parseCSSColor(s.getPropertyValue("--card")),
    primary:         parseCSSColor(s.getPropertyValue("--primary")),
    destructive:     parseCSSColor(s.getPropertyValue("--destructive")),
  }
}

/** Observe html[class] changes (dark mode toggle) */
export function onThemeChange(cb: () => void): () => void {
  const observer = new MutationObserver(cb)
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ["class"] })
  return () => observer.disconnect()
}

// ── Helpers ───────────────────────────────────────
function parseCSSColor(val: string): number {
  // oklch and other modern formats — fallback to 0x888888
  const trimmed = val.trim()
  // Try hex first
  if (trimmed.startsWith("#")) {
    const hex = trimmed.replace("#", "")
    return parseInt(hex.length === 3 ? hex.split("").map((c) => c + c).join("") : hex, 16)
  }
  // oklch() — approximate: just extract lightness and use gray
  if (trimmed.startsWith("oklch")) {
    const match = trimmed.match(/oklch\(([^)]+)\)/)
    if (match) {
      const parts = match[1].split(/\s+/).filter(Boolean)
      const l = parseFloat(parts[0])
      const gray = Math.round(l * 255)
      return (gray << 16) | (gray << 8) | gray
    }
  }
  // rgb/rgba
  if (trimmed.startsWith("rgb")) {
    const match = trimmed.match(/(\d+)/g)
    if (match) {
      return (parseInt(match[0]) << 16) | (parseInt(match[1]) << 8) | parseInt(match[2])
    }
  }
  return 0x888888
}
