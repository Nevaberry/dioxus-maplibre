import { type Demo } from "../types.ts";

declare const maplibregl: typeof import("maplibre-gl");

export const controlDemos: Demo[] = [
  {
    id: "navigation-control",
    title: "Navigation Control",
    description: "Add zoom and compass controls",
    category: "controls",
    code: `map.addControl(new maplibregl.NavigationControl({
  showCompass: true,
  showZoom: true,
  visualizePitch: true
}), 'top-right');`,
    run: (map, container) => {
      const navControl = new maplibregl.NavigationControl({
        showCompass: true,
        showZoom: true,
        visualizePitch: true,
      });

      map.addControl(navControl, "top-right");

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Navigation Control</h4>
        <div class="stat"><span class="label">Compass</span><span class="value">Enabled</span></div>
        <div class="stat"><span class="label">Zoom Buttons</span><span class="value">Enabled</span></div>
        <div class="stat"><span class="label">Pitch Visual</span><span class="value">Enabled</span></div>
        <p style="margin-top: 0.5rem; color: #888; font-size: 0.75rem;">Drag compass to rotate, click to reset north.</p>
      `;
      container.appendChild(info);

      return () => {
        map.removeControl(navControl);
        info.remove();
      };
    },
  },

  {
    id: "scale-control",
    title: "Scale Control",
    description: "Show map scale indicator",
    category: "controls",
    code: `map.addControl(new maplibregl.ScaleControl({
  maxWidth: 100,
  unit: 'metric' // or 'imperial', 'nautical'
}), 'bottom-left');`,
    run: (map, container) => {
      const scaleControl = new maplibregl.ScaleControl({
        maxWidth: 100,
        unit: "metric",
      });

      map.addControl(scaleControl, "bottom-left");

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Scale Control</h4>
        <div class="stat"><span class="label">Unit</span><span class="value" id="scale-unit">metric</span></div>
        <div style="display: flex; flex-direction: column; gap: 0.25rem; margin-top: 0.5rem;">
          <button class="scale-btn" data-unit="metric" style="padding: 0.4rem; background: #e94560; border: none; color: white; border-radius: 4px; cursor: pointer;">Metric</button>
          <button class="scale-btn" data-unit="imperial" style="padding: 0.4rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px; cursor: pointer;">Imperial</button>
          <button class="scale-btn" data-unit="nautical" style="padding: 0.4rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px; cursor: pointer;">Nautical</button>
        </div>
      `;
      container.appendChild(info);

      info.querySelectorAll(".scale-btn").forEach((btn) => {
        btn.addEventListener("click", () => {
          const unit = (btn as HTMLElement).dataset.unit as "metric" | "imperial" | "nautical";
          scaleControl.setUnit(unit);
          document.getElementById("scale-unit")!.textContent = unit;

          info.querySelectorAll(".scale-btn").forEach((b) => {
            (b as HTMLElement).style.background = b === btn ? "#e94560" : "#0f3460";
            (b as HTMLElement).style.border = b === btn ? "none" : "1px solid #16213e";
            (b as HTMLElement).style.color = b === btn ? "white" : "#ccc";
          });
        });
      });

      return () => {
        map.removeControl(scaleControl);
        info.remove();
      };
    },
  },

  {
    id: "fullscreen-control",
    title: "Fullscreen Control",
    description: "Toggle fullscreen mode",
    category: "controls",
    code: `map.addControl(new maplibregl.FullscreenControl({
  container: document.querySelector('#map-container')
}), 'top-right');`,
    run: (map, container) => {
      const fullscreenControl = new maplibregl.FullscreenControl();
      map.addControl(fullscreenControl, "top-right");

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Fullscreen Control</h4>
        <p style="color: #888; font-size: 0.75rem;">Click the fullscreen button in the top-right corner.</p>
        <div class="stat"><span class="label">Shortcut</span><span class="value">Ctrl/Cmd + Shift + F</span></div>
      `;
      container.appendChild(info);

      return () => {
        map.removeControl(fullscreenControl);
        info.remove();
      };
    },
  },

  {
    id: "geolocate-control",
    title: "Geolocate Control",
    description: "Find user's location",
    category: "controls",
    code: `const geolocate = new maplibregl.GeolocateControl({
  positionOptions: { enableHighAccuracy: true },
  trackUserLocation: true,
  showUserHeading: true,
  showAccuracyCircle: true
});

map.addControl(geolocate);

geolocate.on('geolocate', (e) => {
  console.log('Location:', e.coords);
});`,
    run: (map, container) => {
      const geolocateControl = new maplibregl.GeolocateControl({
        positionOptions: { enableHighAccuracy: true },
        trackUserLocation: true,
        showUserHeading: true,
        showAccuracyCircle: true,
      });

      map.addControl(geolocateControl, "top-right");

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Geolocate Control</h4>
        <div class="stat"><span class="label">Status</span><span class="value" id="geo-status">Ready</span></div>
        <div class="stat"><span class="label">Tracking</span><span class="value">Enabled</span></div>
        <p style="margin-top: 0.5rem; color: #888; font-size: 0.75rem;">Click the location button to find your position.</p>
      `;
      container.appendChild(info);

      geolocateControl.on("geolocate", () => {
        document.getElementById("geo-status")!.textContent = "Located";
      });

      geolocateControl.on("error", () => {
        document.getElementById("geo-status")!.textContent = "Error";
      });

      geolocateControl.on("trackuserlocationstart", () => {
        document.getElementById("geo-status")!.textContent = "Tracking...";
      });

      return () => {
        map.removeControl(geolocateControl);
        info.remove();
      };
    },
  },

  {
    id: "attribution-control",
    title: "Attribution Control",
    description: "Show data attribution",
    category: "controls",
    code: `map.addControl(new maplibregl.AttributionControl({
  compact: false,
  customAttribution: 'Custom attribution text'
}), 'bottom-right');`,
    run: (map, container) => {
      // Remove default attribution first
      const defaultAttribution = map._controls.find(
        (c) => c instanceof maplibregl.AttributionControl
      );
      if (defaultAttribution) {
        map.removeControl(defaultAttribution);
      }

      const attributionControl = new maplibregl.AttributionControl({
        compact: false,
        customAttribution: "Demo by dioxus-maplibre",
      });

      map.addControl(attributionControl, "bottom-right");

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Attribution Control</h4>
        <div class="stat"><span class="label">Mode</span><span class="value">Expanded</span></div>
        <div class="stat"><span class="label">Custom Text</span><span class="value">Yes</span></div>
        <p style="margin-top: 0.5rem; color: #888; font-size: 0.75rem;">See attribution in bottom-right corner.</p>
      `;
      container.appendChild(info);

      return () => {
        map.removeControl(attributionControl);
        // Restore default attribution
        map.addControl(new maplibregl.AttributionControl({ compact: true }));
        info.remove();
      };
    },
  },

  {
    id: "terrain-control",
    title: "Terrain Control",
    description: "Toggle 3D terrain",
    category: "controls",
    code: `// First add a DEM source
map.addSource('dem', {
  type: 'raster-dem',
  url: 'https://demotiles.maplibre.org/terrain-tiles/tiles.json'
});

// Then add terrain control
map.addControl(new maplibregl.TerrainControl({
  source: 'dem',
  exaggeration: 1.5
}));`,
    run: (map, container) => {
      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Terrain Control</h4>
        <p style="color: #888; font-size: 0.75rem;">
          Terrain control requires a raster-dem source. This demo shows the concept.
        </p>
        <div class="stat"><span class="label">Source Type</span><span class="value">raster-dem</span></div>
        <div class="stat"><span class="label">Exaggeration</span><span class="value">1.5x (adjustable)</span></div>
        <code style="display: block; background: #0d1117; padding: 0.5rem; border-radius: 3px; font-size: 0.7rem; margin-top: 0.5rem; white-space: pre-wrap;">new TerrainControl({
  source: 'dem',
  exaggeration: 1.5
})</code>
        <p style="margin-top: 0.5rem; color: #888; font-size: 0.75rem;">See 3D Features section for full terrain demo.</p>
      `;
      container.appendChild(info);

      return () => info.remove();
    },
  },

  {
    id: "control-positions",
    title: "Control Positions",
    description: "Place controls in different corners",
    category: "controls",
    code: `map.addControl(control, 'top-left');
map.addControl(control, 'top-right');
map.addControl(control, 'bottom-left');
map.addControl(control, 'bottom-right');`,
    run: (map, container) => {
      const positions: maplibregl.ControlPosition[] = ["top-left", "top-right", "bottom-left", "bottom-right"];
      const controls: maplibregl.NavigationControl[] = [];

      positions.forEach((position) => {
        const control = new maplibregl.NavigationControl({ showCompass: false, showZoom: true });
        map.addControl(control, position);
        controls.push(control);
      });

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Control Positions</h4>
        <p style="color: #888; font-size: 0.75rem;">Navigation controls placed in all four corners.</p>
        <div class="stat"><span class="label">top-left</span><span class="value">Nav</span></div>
        <div class="stat"><span class="label">top-right</span><span class="value">Nav</span></div>
        <div class="stat"><span class="label">bottom-left</span><span class="value">Nav</span></div>
        <div class="stat"><span class="label">bottom-right</span><span class="value">Nav</span></div>
      `;
      container.appendChild(info);

      return () => {
        controls.forEach((c) => map.removeControl(c));
        info.remove();
      };
    },
  },

  {
    id: "custom-control",
    title: "Custom Control",
    description: "Create a custom control class",
    category: "controls",
    code: `class CustomControl {
  onAdd(map) {
    this._map = map;
    this._container = document.createElement('div');
    this._container.className = 'maplibregl-ctrl';
    this._container.innerHTML = '<button>Custom</button>';
    return this._container;
  }

  onRemove() {
    this._container.remove();
    this._map = undefined;
  }
}

map.addControl(new CustomControl(), 'top-left');`,
    run: (map, container) => {
      class CustomControl implements maplibregl.IControl {
        _map?: maplibregl.Map;
        _container?: HTMLDivElement;

        onAdd(map: maplibregl.Map) {
          this._map = map;
          this._container = document.createElement("div");
          this._container.className = "maplibregl-ctrl maplibregl-ctrl-group";
          this._container.innerHTML = `
            <button style="
              padding: 8px 16px;
              background: linear-gradient(135deg, #e94560, #0f3460);
              border: none;
              color: white;
              font-weight: bold;
              cursor: pointer;
              border-radius: 4px;
            ">
              Custom
            </button>
          `;

          this._container.querySelector("button")!.addEventListener("click", () => {
            alert("Custom control clicked!");
          });

          return this._container;
        }

        onRemove() {
          this._container?.remove();
          this._map = undefined;
        }
      }

      const customControl = new CustomControl();
      map.addControl(customControl, "top-left");

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Custom Control</h4>
        <p style="color: #888; font-size: 0.75rem;">Implements IControl interface with onAdd() and onRemove().</p>
        <div class="stat"><span class="label">Position</span><span class="value">top-left</span></div>
        <p style="margin-top: 0.5rem; color: #888; font-size: 0.75rem;">Click the custom button!</p>
      `;
      container.appendChild(info);

      return () => {
        map.removeControl(customControl);
        info.remove();
      };
    },
  },
];
