import { type Demo, type Category, formatCode, DEMO_STYLE } from "./types.ts";

// Import all feature demos
import { initializationDemos } from "./features/01-initialization.ts";
import { navigationDemos } from "./features/02-navigation.ts";
import { layerDemos } from "./features/03-layers.ts";
import { sourceDemos } from "./features/04-sources.ts";
import { markerDemos } from "./features/05-markers-popups.ts";
import { eventDemos } from "./features/06-events.ts";
import { controlDemos } from "./features/07-controls.ts";
import { threeDDemos } from "./features/08-3d-features.ts";

declare const maplibregl: typeof import("maplibre-gl");

// Registry of all demos
const allDemos: Demo[] = [
  ...initializationDemos,
  ...navigationDemos,
  ...layerDemos,
  ...sourceDemos,
  ...markerDemos,
  ...eventDemos,
  ...controlDemos,
  ...threeDDemos,
];

// Group demos by category
const demosByCategory = new Map<Category, Demo[]>();
for (const demo of allDemos) {
  const list = demosByCategory.get(demo.category) || [];
  list.push(demo);
  demosByCategory.set(demo.category, list);
}

// DOM elements
const categoryNav = document.getElementById("category-nav")!;
const demoList = document.getElementById("demo-list")!;
const demoTitle = document.getElementById("demo-title")!;
const demoDescription = document.getElementById("demo-description")!;
const mapContainer = document.getElementById("map")!;
const codeDisplay = document.getElementById("code-display")!;

// State
let currentCategory: Category = "initialization";
let currentDemo: Demo | null = null;
let currentMap: maplibregl.Map | null = null;
let currentCleanup: (() => void) | null = null;

// Initialize category navigation
function initCategoryNav() {
  const buttons = categoryNav.querySelectorAll("button");
  buttons.forEach((btn) => {
    btn.addEventListener("click", () => {
      const category = btn.dataset.category as Category;
      if (category) {
        selectCategory(category);
      }
    });
  });
}

// Select a category
function selectCategory(category: Category) {
  currentCategory = category;

  // Update nav buttons
  categoryNav.querySelectorAll("button").forEach((btn) => {
    btn.classList.toggle("active", btn.dataset.category === category);
  });

  // Update demo list
  renderDemoList(category);

  // Auto-select first demo
  const demos = demosByCategory.get(category) || [];
  if (demos.length > 0) {
    selectDemo(demos[0]);
  }
}

// Render demo list for category
function renderDemoList(category: Category) {
  const demos = demosByCategory.get(category) || [];
  demoList.innerHTML = demos
    .map(
      (demo) => `
    <li>
      <button data-demo="${demo.id}" ${demo === currentDemo ? 'class="active"' : ""}>
        ${demo.title}
      </button>
    </li>
  `
    )
    .join("");

  // Add click handlers
  demoList.querySelectorAll("button").forEach((btn) => {
    btn.addEventListener("click", () => {
      const demoId = btn.dataset.demo;
      const demo = allDemos.find((d) => d.id === demoId);
      if (demo) {
        selectDemo(demo);
      }
    });
  });
}

// Select and run a demo
function selectDemo(demo: Demo) {
  // Clean up previous demo
  if (currentCleanup) {
    currentCleanup();
    currentCleanup = null;
  }
  if (currentMap) {
    currentMap.remove();
    currentMap = null;
  }

  currentDemo = demo;

  // Update UI
  demoTitle.textContent = demo.title;
  demoDescription.textContent = demo.description;
  codeDisplay.innerHTML = formatCode(demo.code);

  // Update demo list selection
  demoList.querySelectorAll("button").forEach((btn) => {
    btn.classList.toggle("active", btn.dataset.demo === demo.id);
  });

  // Clear any existing overlays
  const existingOverlays = mapContainer.parentElement?.querySelectorAll(
    ".info-panel, #event-log"
  );
  existingOverlays?.forEach((el) => el.remove());

  // Create new map
  currentMap = new maplibregl.Map({
    container: mapContainer,
    style: DEMO_STYLE,
    center: [-74.006, 40.7128],
    zoom: 12,
  });

  // Run demo when map loads
  currentMap.on("load", () => {
    if (currentMap && currentDemo === demo) {
      const result = demo.run(currentMap, mapContainer.parentElement!);
      if (typeof result === "function") {
        currentCleanup = result;
      }
    }
  });
}

// Handle URL hash for deep linking
function handleHashChange() {
  const hash = window.location.hash.slice(1);
  if (hash) {
    const demo = allDemos.find((d) => d.id === hash);
    if (demo) {
      selectCategory(demo.category);
      selectDemo(demo);
      return;
    }
  }
  // Default to first demo
  selectCategory("initialization");
}

// Initialize
document.addEventListener("DOMContentLoaded", () => {
  initCategoryNav();
  handleHashChange();
  window.addEventListener("hashchange", handleHashChange);
});
