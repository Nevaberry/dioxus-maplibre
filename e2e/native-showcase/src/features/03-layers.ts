import { type Demo } from "../types.ts";

declare const maplibregl: typeof import("maplibre-gl");

export const layerDemos: Demo[] = [
  {
    id: "circle-layer",
    title: "Circle Layer",
    description: "Render points as circles with data-driven styling",
    category: "layers",
    code: `map.addSource('points', {
  type: 'geojson',
  data: geojsonData
});

map.addLayer({
  id: 'circles',
  type: 'circle',
  source: 'points',
  paint: {
    'circle-radius': ['get', 'size'],
    'circle-color': ['get', 'color'],
    'circle-opacity': 0.8
  }
});`,
    run: (map, container) => {
      // Generate random points
      const points = Array.from({ length: 50 }, () => ({
        type: "Feature" as const,
        geometry: {
          type: "Point" as const,
          coordinates: [
            -74.006 + (Math.random() - 0.5) * 0.1,
            40.7128 + (Math.random() - 0.5) * 0.1,
          ],
        },
        properties: {
          size: 5 + Math.random() * 20,
          color: `hsl(${Math.random() * 360}, 70%, 50%)`,
        },
      }));

      map.addSource("demo-points", {
        type: "geojson",
        data: { type: "FeatureCollection", features: points },
      });

      map.addLayer({
        id: "demo-circles",
        type: "circle",
        source: "demo-points",
        paint: {
          "circle-radius": ["get", "size"],
          "circle-color": ["get", "color"],
          "circle-opacity": 0.8,
          "circle-stroke-width": 2,
          "circle-stroke-color": "#fff",
        },
      });

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Circle Layer</h4>
        <div class="stat"><span class="label">Points</span><span class="value">50</span></div>
        <div class="stat"><span class="label">Radius</span><span class="value">5-25px (data-driven)</span></div>
        <div class="stat"><span class="label">Color</span><span class="value">Random HSL</span></div>
      `;
      container.appendChild(info);

      return () => {
        if (map.getLayer("demo-circles")) map.removeLayer("demo-circles");
        if (map.getSource("demo-points")) map.removeSource("demo-points");
        info.remove();
      };
    },
  },

  {
    id: "fill-layer",
    title: "Fill Layer",
    description: "Render polygons with fill and outline",
    category: "layers",
    code: `map.addLayer({
  id: 'polygon-fill',
  type: 'fill',
  source: 'polygons',
  paint: {
    'fill-color': '#088',
    'fill-opacity': 0.5,
    'fill-outline-color': '#000'
  }
});`,
    run: (map, container) => {
      // Create a polygon around Central Park
      const centralPark = {
        type: "Feature" as const,
        geometry: {
          type: "Polygon" as const,
          coordinates: [[
            [-73.9819, 40.7681],
            [-73.9580, 40.8006],
            [-73.9498, 40.7969],
            [-73.9734, 40.7644],
            [-73.9819, 40.7681],
          ]],
        },
        properties: { name: "Central Park" },
      };

      map.addSource("demo-polygon", {
        type: "geojson",
        data: centralPark,
      });

      map.addLayer({
        id: "demo-fill",
        type: "fill",
        source: "demo-polygon",
        paint: {
          "fill-color": "#2ecc71",
          "fill-opacity": 0.5,
        },
      });

      map.addLayer({
        id: "demo-outline",
        type: "line",
        source: "demo-polygon",
        paint: {
          "line-color": "#27ae60",
          "line-width": 3,
        },
      });

      map.fitBounds([[-73.9819, 40.7644], [-73.9498, 40.8006]], { padding: 50 });

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Fill Layer</h4>
        <div class="stat"><span class="label">Polygon</span><span class="value">Central Park</span></div>
        <div class="stat"><span class="label">Fill Color</span><span class="value">#2ecc71</span></div>
        <div class="stat"><span class="label">Opacity</span><span class="value">0.5</span></div>
      `;
      container.appendChild(info);

      return () => {
        if (map.getLayer("demo-outline")) map.removeLayer("demo-outline");
        if (map.getLayer("demo-fill")) map.removeLayer("demo-fill");
        if (map.getSource("demo-polygon")) map.removeSource("demo-polygon");
        info.remove();
      };
    },
  },

  {
    id: "line-layer",
    title: "Line Layer",
    description: "Render lines with various styles",
    category: "layers",
    code: `map.addLayer({
  id: 'route',
  type: 'line',
  source: 'route',
  layout: {
    'line-join': 'round',
    'line-cap': 'round'
  },
  paint: {
    'line-color': '#e74c3c',
    'line-width': 4,
    'line-dasharray': [2, 1]
  }
});`,
    run: (map, container) => {
      // Create a route line
      const route = {
        type: "Feature" as const,
        geometry: {
          type: "LineString" as const,
          coordinates: [
            [-74.006, 40.7128],
            [-73.99, 40.725],
            [-73.975, 40.73],
            [-73.96, 40.745],
            [-73.97, 40.758],
          ],
        },
        properties: {},
      };

      map.addSource("demo-route", {
        type: "geojson",
        data: route,
      });

      // Background line (wider, for glow effect)
      map.addLayer({
        id: "demo-route-bg",
        type: "line",
        source: "demo-route",
        layout: {
          "line-join": "round",
          "line-cap": "round",
        },
        paint: {
          "line-color": "#e74c3c",
          "line-width": 8,
          "line-opacity": 0.3,
        },
      });

      // Main line
      map.addLayer({
        id: "demo-route-line",
        type: "line",
        source: "demo-route",
        layout: {
          "line-join": "round",
          "line-cap": "round",
        },
        paint: {
          "line-color": "#e74c3c",
          "line-width": 4,
        },
      });

      // Dashed overlay
      map.addLayer({
        id: "demo-route-dash",
        type: "line",
        source: "demo-route",
        layout: {
          "line-join": "round",
          "line-cap": "round",
        },
        paint: {
          "line-color": "#fff",
          "line-width": 2,
          "line-dasharray": [2, 2],
        },
      });

      map.fitBounds([[-74.01, 40.71], [-73.95, 40.76]], { padding: 50 });

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Line Layer</h4>
        <div class="stat"><span class="label">Join</span><span class="value">round</span></div>
        <div class="stat"><span class="label">Cap</span><span class="value">round</span></div>
        <div class="stat"><span class="label">Width</span><span class="value">4px + 8px glow</span></div>
        <div class="stat"><span class="label">Dash</span><span class="value">[2, 2]</span></div>
      `;
      container.appendChild(info);

      return () => {
        ["demo-route-dash", "demo-route-line", "demo-route-bg"].forEach((id) => {
          if (map.getLayer(id)) map.removeLayer(id);
        });
        if (map.getSource("demo-route")) map.removeSource("demo-route");
        info.remove();
      };
    },
  },

  {
    id: "symbol-layer",
    title: "Symbol Layer",
    description: "Render text labels and icons",
    category: "layers",
    code: `map.addLayer({
  id: 'labels',
  type: 'symbol',
  source: 'places',
  layout: {
    'text-field': ['get', 'name'],
    'text-size': 14,
    'text-anchor': 'top',
    'text-offset': [0, 0.5]
  },
  paint: {
    'text-color': '#333',
    'text-halo-color': '#fff',
    'text-halo-width': 2
  }
});`,
    run: (map, container) => {
      const places = {
        type: "FeatureCollection" as const,
        features: [
          { type: "Feature" as const, geometry: { type: "Point" as const, coordinates: [-74.006, 40.7128] }, properties: { name: "New York", size: 18 } },
          { type: "Feature" as const, geometry: { type: "Point" as const, coordinates: [-73.9857, 40.7484] }, properties: { name: "Times Square", size: 14 } },
          { type: "Feature" as const, geometry: { type: "Point" as const, coordinates: [-73.9654, 40.7829] }, properties: { name: "Central Park", size: 14 } },
          { type: "Feature" as const, geometry: { type: "Point" as const, coordinates: [-74.0445, 40.6892] }, properties: { name: "Statue of Liberty", size: 12 } },
          { type: "Feature" as const, geometry: { type: "Point" as const, coordinates: [-73.9857, 40.7580] }, properties: { name: "Rockefeller", size: 12 } },
        ],
      };

      map.addSource("demo-places", {
        type: "geojson",
        data: places,
      });

      map.addLayer({
        id: "demo-labels",
        type: "symbol",
        source: "demo-places",
        layout: {
          "text-field": ["get", "name"],
          "text-size": ["get", "size"],
          "text-anchor": "center",
          "text-allow-overlap": true,
        },
        paint: {
          "text-color": "#1a1a2e",
          "text-halo-color": "#fff",
          "text-halo-width": 2,
        },
      });

      map.setZoom(11);

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Symbol Layer</h4>
        <div class="stat"><span class="label">Labels</span><span class="value">5 places</span></div>
        <div class="stat"><span class="label">Size</span><span class="value">12-18px (data-driven)</span></div>
        <div class="stat"><span class="label">Halo</span><span class="value">2px white</span></div>
      `;
      container.appendChild(info);

      return () => {
        if (map.getLayer("demo-labels")) map.removeLayer("demo-labels");
        if (map.getSource("demo-places")) map.removeSource("demo-places");
        info.remove();
      };
    },
  },

  {
    id: "heatmap-layer",
    title: "Heatmap Layer",
    description: "Visualize point density as a heatmap",
    category: "layers",
    code: `map.addLayer({
  id: 'heatmap',
  type: 'heatmap',
  source: 'points',
  paint: {
    'heatmap-weight': ['get', 'magnitude'],
    'heatmap-intensity': 1,
    'heatmap-radius': 20,
    'heatmap-opacity': 0.8,
    'heatmap-color': [
      'interpolate', ['linear'], ['heatmap-density'],
      0, 'rgba(0,0,255,0)',
      0.5, 'lime',
      1, 'red'
    ]
  }
});`,
    run: (map, container) => {
      // Generate random points for heatmap
      const points = Array.from({ length: 200 }, () => ({
        type: "Feature" as const,
        geometry: {
          type: "Point" as const,
          coordinates: [
            -74.006 + (Math.random() - 0.5) * 0.08,
            40.7128 + (Math.random() - 0.5) * 0.08,
          ],
        },
        properties: {
          magnitude: Math.random(),
        },
      }));

      map.addSource("demo-heatmap", {
        type: "geojson",
        data: { type: "FeatureCollection", features: points },
      });

      map.addLayer({
        id: "demo-heatmap-layer",
        type: "heatmap",
        source: "demo-heatmap",
        paint: {
          "heatmap-weight": ["get", "magnitude"],
          "heatmap-intensity": 1,
          "heatmap-radius": 25,
          "heatmap-opacity": 0.8,
          "heatmap-color": [
            "interpolate",
            ["linear"],
            ["heatmap-density"],
            0, "rgba(33,102,172,0)",
            0.2, "rgb(103,169,207)",
            0.4, "rgb(209,229,240)",
            0.6, "rgb(253,219,199)",
            0.8, "rgb(239,138,98)",
            1, "rgb(178,24,43)",
          ],
        },
      });

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Heatmap Layer</h4>
        <div class="stat"><span class="label">Points</span><span class="value">200</span></div>
        <div class="stat"><span class="label">Radius</span><span class="value">25px</span></div>
        <div class="stat"><span class="label">Weight</span><span class="value">0-1 (random)</span></div>
      `;
      container.appendChild(info);

      return () => {
        if (map.getLayer("demo-heatmap-layer")) map.removeLayer("demo-heatmap-layer");
        if (map.getSource("demo-heatmap")) map.removeSource("demo-heatmap");
        info.remove();
      };
    },
  },

  {
    id: "layer-ordering",
    title: "Layer Ordering",
    description: "Control layer z-order with moveLayer()",
    category: "layers",
    code: `// Move layer before another
map.moveLayer('circles', 'labels');

// Move to top
map.moveLayer('circles');

// Get layer order
const order = map.getLayersOrder();`,
    run: (map, container) => {
      // Create three overlapping squares
      const colors = ["#e74c3c", "#3498db", "#2ecc71"];
      const names = ["Red", "Blue", "Green"];
      const offsets = [0, 0.01, 0.02];

      colors.forEach((color, i) => {
        const center = [-74.006 + offsets[i], 40.7128 + offsets[i]];
        const size = 0.015;
        map.addSource(`demo-square-${i}`, {
          type: "geojson",
          data: {
            type: "Feature",
            geometry: {
              type: "Polygon",
              coordinates: [[
                [center[0] - size, center[1] - size],
                [center[0] + size, center[1] - size],
                [center[0] + size, center[1] + size],
                [center[0] - size, center[1] + size],
                [center[0] - size, center[1] - size],
              ]],
            },
            properties: {},
          },
        });

        map.addLayer({
          id: `demo-square-${i}`,
          type: "fill",
          source: `demo-square-${i}`,
          paint: {
            "fill-color": color,
            "fill-opacity": 0.8,
          },
        });
      });

      map.setZoom(13);

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Layer Ordering</h4>
        <p style="color: #888; font-size: 0.75rem; margin-bottom: 0.5rem;">Click to bring layer to top:</p>
        <div style="display: flex; flex-direction: column; gap: 0.25rem;">
          ${names.map((name, i) => `
            <button class="layer-btn" data-layer="${i}" style="padding: 0.4rem; background: ${colors[i]}; border: none; color: white; border-radius: 4px; cursor: pointer;">
              Bring ${name} to Top
            </button>
          `).join("")}
        </div>
      `;
      container.appendChild(info);

      info.querySelectorAll(".layer-btn").forEach((btn) => {
        btn.addEventListener("click", () => {
          const layerIndex = (btn as HTMLElement).dataset.layer;
          map.moveLayer(`demo-square-${layerIndex}`);
        });
      });

      return () => {
        [0, 1, 2].forEach((i) => {
          if (map.getLayer(`demo-square-${i}`)) map.removeLayer(`demo-square-${i}`);
          if (map.getSource(`demo-square-${i}`)) map.removeSource(`demo-square-${i}`);
        });
        info.remove();
      };
    },
  },

  {
    id: "paint-properties",
    title: "Dynamic Paint Properties",
    description: "Change paint properties at runtime",
    category: "layers",
    code: `// Set paint property
map.setPaintProperty('circles', 'circle-color', '#ff0000');

// Get paint property
const color = map.getPaintProperty('circles', 'circle-color');`,
    run: (map, container) => {
      const points = Array.from({ length: 30 }, () => ({
        type: "Feature" as const,
        geometry: {
          type: "Point" as const,
          coordinates: [
            -74.006 + (Math.random() - 0.5) * 0.08,
            40.7128 + (Math.random() - 0.5) * 0.08,
          ],
        },
        properties: {},
      }));

      map.addSource("demo-dynamic", {
        type: "geojson",
        data: { type: "FeatureCollection", features: points },
      });

      map.addLayer({
        id: "demo-dynamic-circles",
        type: "circle",
        source: "demo-dynamic",
        paint: {
          "circle-radius": 12,
          "circle-color": "#e94560",
          "circle-opacity": 0.8,
        },
      });

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Dynamic Paint</h4>
        <div style="margin-bottom: 0.5rem;">
          <label style="color: #888; font-size: 0.75rem;">Color:</label>
          <input type="color" id="paint-color" value="#e94560" style="width: 100%; height: 30px; border: none; cursor: pointer;">
        </div>
        <div style="margin-bottom: 0.5rem;">
          <label style="color: #888; font-size: 0.75rem;">Radius: <span id="radius-val">12</span>px</label>
          <input type="range" id="paint-radius" min="5" max="30" value="12" style="width: 100%;">
        </div>
        <div>
          <label style="color: #888; font-size: 0.75rem;">Opacity: <span id="opacity-val">0.8</span></label>
          <input type="range" id="paint-opacity" min="0" max="100" value="80" style="width: 100%;">
        </div>
      `;
      container.appendChild(info);

      document.getElementById("paint-color")!.addEventListener("input", (e) => {
        map.setPaintProperty("demo-dynamic-circles", "circle-color", (e.target as HTMLInputElement).value);
      });

      document.getElementById("paint-radius")!.addEventListener("input", (e) => {
        const val = parseInt((e.target as HTMLInputElement).value);
        document.getElementById("radius-val")!.textContent = val.toString();
        map.setPaintProperty("demo-dynamic-circles", "circle-radius", val);
      });

      document.getElementById("paint-opacity")!.addEventListener("input", (e) => {
        const val = parseInt((e.target as HTMLInputElement).value) / 100;
        document.getElementById("opacity-val")!.textContent = val.toFixed(2);
        map.setPaintProperty("demo-dynamic-circles", "circle-opacity", val);
      });

      return () => {
        if (map.getLayer("demo-dynamic-circles")) map.removeLayer("demo-dynamic-circles");
        if (map.getSource("demo-dynamic")) map.removeSource("demo-dynamic");
        info.remove();
      };
    },
  },

  {
    id: "layer-filter",
    title: "Layer Filters",
    description: "Filter features with expressions",
    category: "layers",
    code: `map.setFilter('circles', [
  'all',
  ['>=', ['get', 'size'], 10],
  ['==', ['get', 'type'], 'important']
]);`,
    run: (map, container) => {
      const categories = ["A", "B", "C"];
      const points = Array.from({ length: 100 }, () => ({
        type: "Feature" as const,
        geometry: {
          type: "Point" as const,
          coordinates: [
            -74.006 + (Math.random() - 0.5) * 0.1,
            40.7128 + (Math.random() - 0.5) * 0.1,
          ],
        },
        properties: {
          category: categories[Math.floor(Math.random() * 3)],
        },
      }));

      map.addSource("demo-filter", {
        type: "geojson",
        data: { type: "FeatureCollection", features: points },
      });

      map.addLayer({
        id: "demo-filter-circles",
        type: "circle",
        source: "demo-filter",
        paint: {
          "circle-radius": 8,
          "circle-color": [
            "match",
            ["get", "category"],
            "A", "#e74c3c",
            "B", "#3498db",
            "C", "#2ecc71",
            "#888",
          ],
          "circle-opacity": 0.8,
        },
      });

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Layer Filter</h4>
        <p style="color: #888; font-size: 0.75rem; margin-bottom: 0.5rem;">Toggle categories:</p>
        <div style="display: flex; flex-direction: column; gap: 0.25rem;">
          <label style="display: flex; align-items: center; gap: 0.5rem; cursor: pointer;">
            <input type="checkbox" class="cat-filter" data-cat="A" checked>
            <span style="color: #e74c3c;">Category A</span>
          </label>
          <label style="display: flex; align-items: center; gap: 0.5rem; cursor: pointer;">
            <input type="checkbox" class="cat-filter" data-cat="B" checked>
            <span style="color: #3498db;">Category B</span>
          </label>
          <label style="display: flex; align-items: center; gap: 0.5rem; cursor: pointer;">
            <input type="checkbox" class="cat-filter" data-cat="C" checked>
            <span style="color: #2ecc71;">Category C</span>
          </label>
        </div>
      `;
      container.appendChild(info);

      const updateFilter = () => {
        const checked: string[] = [];
        info.querySelectorAll(".cat-filter:checked").forEach((cb) => {
          checked.push((cb as HTMLInputElement).dataset.cat!);
        });

        if (checked.length === 3) {
          map.setFilter("demo-filter-circles", null);
        } else if (checked.length === 0) {
          map.setFilter("demo-filter-circles", ["==", "category", "NONE"]);
        } else {
          map.setFilter("demo-filter-circles", ["in", "category", ...checked]);
        }
      };

      info.querySelectorAll(".cat-filter").forEach((cb) => {
        cb.addEventListener("change", updateFilter);
      });

      return () => {
        if (map.getLayer("demo-filter-circles")) map.removeLayer("demo-filter-circles");
        if (map.getSource("demo-filter")) map.removeSource("demo-filter");
        info.remove();
      };
    },
  },
];
