// TypeScript declarations for MapLibre GL JS (loaded via CDN)
declare const maplibregl: typeof import("maplibre-gl");

export interface Demo {
  id: string;
  title: string;
  description: string;
  category: Category;
  code: string;
  run: (map: maplibregl.Map, container: HTMLElement) => void | (() => void);
}

export type Category =
  | "initialization"
  | "navigation"
  | "layers"
  | "sources"
  | "markers"
  | "events"
  | "controls"
  | "3d";

export interface DemoRegistry {
  demos: Map<string, Demo>;
  byCategory: Map<Category, Demo[]>;
}

// Helper to create demo code display
export function formatCode(code: string): string {
  return code
    .replace(/\/\/.*/g, '<span class="comment">$&</span>')
    .replace(
      /\b(const|let|var|function|return|if|else|for|while|new|await|async)\b/g,
      '<span class="keyword">$1</span>'
    )
    .replace(/'[^']*'|"[^"]*"|`[^`]*`/g, '<span class="string">$&</span>')
    .replace(/\b(\d+\.?\d*)\b/g, '<span class="number">$1</span>');
}

// Free tile style for demos (no API key required)
export const DEMO_STYLE =
  "https://demotiles.maplibre.org/style.json";

// Alternative styles
export const STYLES = {
  demo: "https://demotiles.maplibre.org/style.json",
  osm: "https://tiles.openfreemap.org/styles/liberty",
  positron:
    "https://basemaps.cartocdn.com/gl/positron-gl-style/style.json",
  darkMatter:
    "https://basemaps.cartocdn.com/gl/dark-matter-gl-style/style.json",
};

// Default map options
export const DEFAULT_CENTER: [number, number] = [-74.006, 40.7128]; // NYC
export const DEFAULT_ZOOM = 12;
