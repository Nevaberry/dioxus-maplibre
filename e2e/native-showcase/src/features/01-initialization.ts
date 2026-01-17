import { type Demo, STYLES } from "../types.ts";

declare const maplibregl: typeof import("maplibre-gl");

export const initializationDemos: Demo[] = [
  {
    id: "basic-map",
    title: "Basic Map",
    description: "Minimal map initialization with center and zoom",
    category: "initialization",
    code: `const map = new maplibregl.Map({
  container: 'map',
  style: 'https://demotiles.maplibre.org/style.json',
  center: [-74.006, 40.7128],
  zoom: 12
});`,
    run: (map, container) => {
      // Map is already created by main.ts, just show info
      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Map Info</h4>
        <div class="stat"><span class="label">Center</span><span class="value" id="info-center">-74.006, 40.713</span></div>
        <div class="stat"><span class="label">Zoom</span><span class="value" id="info-zoom">12.00</span></div>
        <div class="stat"><span class="label">Bearing</span><span class="value" id="info-bearing">0.00</span></div>
        <div class="stat"><span class="label">Pitch</span><span class="value" id="info-pitch">0.00</span></div>
      `;
      container.appendChild(info);

      const updateInfo = () => {
        const center = map.getCenter();
        document.getElementById("info-center")!.textContent = `${center.lng.toFixed(3)}, ${center.lat.toFixed(3)}`;
        document.getElementById("info-zoom")!.textContent = map.getZoom().toFixed(2);
        document.getElementById("info-bearing")!.textContent = map.getBearing().toFixed(2);
        document.getElementById("info-pitch")!.textContent = map.getPitch().toFixed(2);
      };

      map.on("move", updateInfo);
      updateInfo();

      return () => {
        info.remove();
      };
    },
  },

  {
    id: "initial-bounds",
    title: "Initial Bounds",
    description: "Initialize map to fit specific bounds",
    category: "initialization",
    code: `const map = new maplibregl.Map({
  container: 'map',
  style: 'https://demotiles.maplibre.org/style.json',
  bounds: [
    [-74.2591, 40.4774], // SW
    [-73.7002, 40.9162]  // NE
  ],
  fitBoundsOptions: {
    padding: 50
  }
});`,
    run: (map, container) => {
      // Fit to NYC bounds
      const bounds: maplibregl.LngLatBoundsLike = [
        [-74.2591, 40.4774],
        [-73.7002, 40.9162],
      ];
      map.fitBounds(bounds, { padding: 50, duration: 1000 });

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Bounds</h4>
        <div class="stat"><span class="label">SW</span><span class="value">-74.259, 40.477</span></div>
        <div class="stat"><span class="label">NE</span><span class="value">-73.700, 40.916</span></div>
        <div class="stat"><span class="label">Padding</span><span class="value">50px</span></div>
      `;
      container.appendChild(info);

      return () => info.remove();
    },
  },

  {
    id: "pitch-bearing",
    title: "Pitch & Bearing",
    description: "Initialize map with 3D pitch and rotation",
    category: "initialization",
    code: `const map = new maplibregl.Map({
  container: 'map',
  style: 'https://demotiles.maplibre.org/style.json',
  center: [-74.006, 40.7128],
  zoom: 15,
  pitch: 60,
  bearing: -17.6
});`,
    run: (map, container) => {
      map.easeTo({
        pitch: 60,
        bearing: -17.6,
        zoom: 15,
        duration: 1500,
      });

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>3D View</h4>
        <div class="stat"><span class="label">Pitch</span><span class="value">60°</span></div>
        <div class="stat"><span class="label">Bearing</span><span class="value">-17.6°</span></div>
        <div class="stat"><span class="label">Zoom</span><span class="value">15</span></div>
      `;
      container.appendChild(info);

      return () => info.remove();
    },
  },

  {
    id: "interaction-options",
    title: "Interaction Options",
    description: "Disable specific map interactions",
    category: "initialization",
    code: `const map = new maplibregl.Map({
  container: 'map',
  style: 'https://demotiles.maplibre.org/style.json',
  center: [-74.006, 40.7128],
  zoom: 12,
  scrollZoom: false,
  doubleClickZoom: false,
  dragRotate: false
});`,
    run: (map, container) => {
      map.scrollZoom.disable();
      map.doubleClickZoom.disable();
      map.dragRotate.disable();

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Disabled Interactions</h4>
        <div class="stat"><span class="label">Scroll Zoom</span><span class="value">OFF</span></div>
        <div class="stat"><span class="label">Double-click Zoom</span><span class="value">OFF</span></div>
        <div class="stat"><span class="label">Drag Rotate</span><span class="value">OFF</span></div>
        <p style="margin-top: 0.5rem; color: #888; font-size: 0.75rem;">Try scrolling or double-clicking - nothing happens!</p>
      `;
      container.appendChild(info);

      return () => {
        map.scrollZoom.enable();
        map.doubleClickZoom.enable();
        map.dragRotate.enable();
        info.remove();
      };
    },
  },

  {
    id: "zoom-constraints",
    title: "Zoom Constraints",
    description: "Limit min/max zoom levels",
    category: "initialization",
    code: `const map = new maplibregl.Map({
  container: 'map',
  style: 'https://demotiles.maplibre.org/style.json',
  center: [-74.006, 40.7128],
  zoom: 12,
  minZoom: 10,
  maxZoom: 15
});`,
    run: (map, container) => {
      map.setMinZoom(10);
      map.setMaxZoom(15);

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Zoom Constraints</h4>
        <div class="stat"><span class="label">Min Zoom</span><span class="value">10</span></div>
        <div class="stat"><span class="label">Max Zoom</span><span class="value">15</span></div>
        <div class="stat"><span class="label">Current</span><span class="value" id="current-zoom">12.00</span></div>
        <p style="margin-top: 0.5rem; color: #888; font-size: 0.75rem;">Try zooming beyond the limits!</p>
      `;
      container.appendChild(info);

      map.on("zoom", () => {
        const el = document.getElementById("current-zoom");
        if (el) el.textContent = map.getZoom().toFixed(2);
      });

      return () => {
        map.setMinZoom(0);
        map.setMaxZoom(22);
        info.remove();
      };
    },
  },

  {
    id: "max-bounds",
    title: "Max Bounds",
    description: "Restrict panning to specific area",
    category: "initialization",
    code: `const map = new maplibregl.Map({
  container: 'map',
  style: 'https://demotiles.maplibre.org/style.json',
  center: [-74.006, 40.7128],
  zoom: 11,
  maxBounds: [
    [-74.3, 40.5],  // SW
    [-73.7, 40.95]  // NE
  ]
});`,
    run: (map, container) => {
      const bounds: maplibregl.LngLatBoundsLike = [
        [-74.3, 40.5],
        [-73.7, 40.95],
      ];
      map.setMaxBounds(bounds);
      map.setZoom(11);

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Max Bounds (NYC)</h4>
        <div class="stat"><span class="label">SW</span><span class="value">-74.3, 40.5</span></div>
        <div class="stat"><span class="label">NE</span><span class="value">-73.7, 40.95</span></div>
        <p style="margin-top: 0.5rem; color: #888; font-size: 0.75rem;">Try panning outside NYC - you can't!</p>
      `;
      container.appendChild(info);

      return () => {
        map.setMaxBounds(null);
        info.remove();
      };
    },
  },

  {
    id: "style-switching",
    title: "Style Switching",
    description: "Change map style dynamically",
    category: "initialization",
    code: `// Switch styles dynamically
map.setStyle('https://tiles.openfreemap.org/styles/liberty');

// Or with transition options
map.setStyle(newStyle, {
  diff: true,
  localIdeographFontFamily: 'sans-serif'
});`,
    run: (map, container) => {
      const styles = [
        { name: "Demo Tiles", url: STYLES.demo },
        { name: "Positron", url: STYLES.positron },
        { name: "Dark Matter", url: STYLES.darkMatter },
      ];
      let currentIndex = 0;

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Style Switcher</h4>
        <div class="stat"><span class="label">Current</span><span class="value" id="current-style">${styles[0].name}</span></div>
        <button id="switch-style" style="margin-top: 0.5rem; padding: 0.5rem 1rem; background: #e94560; border: none; color: white; border-radius: 4px; cursor: pointer;">
          Switch Style
        </button>
      `;
      container.appendChild(info);

      const btn = document.getElementById("switch-style")!;
      btn.addEventListener("click", () => {
        currentIndex = (currentIndex + 1) % styles.length;
        const style = styles[currentIndex];
        map.setStyle(style.url);
        document.getElementById("current-style")!.textContent = style.name;
      });

      return () => info.remove();
    },
  },

  {
    id: "hash-navigation",
    title: "Hash Navigation",
    description: "Sync map position with URL hash",
    category: "initialization",
    code: `const map = new maplibregl.Map({
  container: 'map',
  style: 'https://demotiles.maplibre.org/style.json',
  center: [-74.006, 40.7128],
  zoom: 12,
  hash: true  // or 'map' for named parameter
});`,
    run: (map, container) => {
      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Hash Navigation</h4>
        <p style="color: #888; font-size: 0.75rem; margin-bottom: 0.5rem;">
          Move the map and watch the URL change! The hash format is:
        </p>
        <code style="background: #0d1117; padding: 0.25rem 0.5rem; border-radius: 3px; font-size: 0.75rem;">
          #zoom/lat/lng/bearing/pitch
        </code>
        <p style="color: #888; font-size: 0.75rem; margin-top: 0.5rem;">
          Note: Hash is simulated here (map already created).
        </p>
      `;
      container.appendChild(info);

      return () => info.remove();
    },
  },
];
