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

/**
 * 解析 CSS 颜色值为 PixiJS 可用的 number (0xRRGGBB)
 * 支持: #hex, rgb(), oklch()
 */
function parseCSSColor(val: string): number {
  const trimmed = val.trim()

  // Try hex first
  if (trimmed.startsWith("#")) {
    const hex = trimmed.replace("#", "")
    return parseInt(hex.length === 3 ? hex.split("").map((c) => c + c).join("") : hex, 16)
  }

  // rgb/rgba
  if (trimmed.startsWith("rgb")) {
    const match = trimmed.match(/(\d+)/g)
    if (match) {
      return (parseInt(match[0]) << 16) | (parseInt(match[1]) << 8) | parseInt(match[2])
    }
  }

  // oklch(L C H) / oklch(L C H / alpha)
  if (trimmed.startsWith("oklch")) {
    return parseOklch(trimmed)
  }

  return 0x888888
}

/**
 * OKLCH → sRGB 颜色转换
 * 格式: oklch(L C H) 或 oklch(L C H / alpha)
 * L: 0~1, C: 0~0.4+, H: 0~360
 */
function parseOklch(val: string): number {
  const match = val.match(/oklch\(\s*([^)]+)\s*\)/)
  if (!match) return 0x888888

  const inner = match[1].split("/")[0].trim() // 去掉 alpha 部分
  const parts = inner.split(/\s+/).filter(Boolean)
  if (parts.length < 3) return 0x888888

  const L = parseFloat(parts[0])
  const C = parseFloat(parts[1])
  const H = parseFloat(parts[2])

  // OKLCH → OKLab
  const hRad = H * (Math.PI / 180)
  const a = C * Math.cos(hRad)
  const b = C * Math.sin(hRad)

  // OKLab → linear sRGB (CSS Color Level 4 矩阵)
  const l_ = L + 0.3963377774 * a + 0.2158037573 * b
  const m_ = L - 0.1055613458 * a - 0.0638541728 * b
  const s_ = L - 0.0894841775 * a - 1.2914855480 * b

  let r = +4.0767416621 * l_ - 3.3077115913 * m_ + 0.2309699292 * s_
  let g = -1.2684380046 * l_ + 2.6097574011 * m_ - 0.3413193965 * s_
  let b_ = -0.0041960863 * l_ - 0.7034186147 * m_ + 1.7076147010 * s_

  // linear → sRGB gamma
  const gamma = (c: number): number => {
    const clamped = Math.max(0, Math.min(1, c))
    return clamped <= 0.0031308
      ? clamped * 12.92
      : 1.055 * Math.pow(clamped, 1 / 2.4) - 0.055
  }

  const R = Math.round(gamma(r) * 255)
  const G = Math.round(gamma(g) * 255)
  const B = Math.round(gamma(b_) * 255)

  return (R << 16) | (G << 8) | B
}
