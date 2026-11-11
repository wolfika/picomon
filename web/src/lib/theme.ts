// Keep in sync with THEME_PRESETS in src/config.rs and the selectors in app.css.
export const THEMES = [
  'glass',
  'dark',
  'light',
  'nord',
  'catppuccin-latte',
  'catppuccin-frappe',
  'catppuccin-macchiato',
  'catppuccin-mocha',
] as const;

const LABELS: Record<string, string> = {
  glass: 'Glass',
  dark: 'Dark',
  light: 'Light',
  nord: 'Nord',
  'catppuccin-latte': 'Catppuccin Latte',
  'catppuccin-frappe': 'Catppuccin Frappé',
  'catppuccin-macchiato': 'Catppuccin Macchiato',
  'catppuccin-mocha': 'Catppuccin Mocha',
};

/** Display name for a preset id. */
export function themeLabel(id: string): string {
  return LABELS[id] ?? id;
}

const KEY = 'picomon-theme';
const root = document.documentElement;

/** The preset chosen in the server config, injected into index.html. */
export const serverTheme = root.dataset.theme ?? 'glass';

/** Applies the theme this browser picked earlier, if any. */
export function restoreTheme() {
  try {
    const saved = localStorage.getItem(KEY);
    if (saved && (THEMES as readonly string[]).includes(saved)) root.dataset.theme = saved;
  } catch {
    // Storage can be unavailable (private mode, blocked cookies); use the server default.
  }
}

export function setTheme(theme: string) {
  root.dataset.theme = theme;
  try {
    if (theme === serverTheme) localStorage.removeItem(KEY);
    else localStorage.setItem(KEY, theme);
  } catch {
    // See restoreTheme.
  }
}
