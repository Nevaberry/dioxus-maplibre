import { type Demo } from "../types.ts";

declare const maplibregl: typeof import("maplibre-gl");

export const threeDDemos: Demo[] = [
  {
    id: "fill-extrusion",
    title: "3D Buildings",
    description: "Render extruded polygons for buildings",
    category: "3d",
    code: `map.addLayer({
  id: 'buildings',
  type: 'fill-extrusion',
  source: 'buildings',
  paint: {
    'fill-extrusion-color': '#aaa',
    'fill-extrusion-height': ['get', 'height'],
    'fill-extrusion-base': ['get', 'base'],
    'fill-extrusion-opacity': 0.8
  }
});`,
    run: (map, container) => {
      // Create some mock building polygons
      const buildings = {
        type: "FeatureCollection" as const,
        features: Array.from({ length: 20 }, (_, i) => {
          const x = -74.006 + (Math.random() - 0.5) * 0.02;
          const y = 40.7128 + (Math.random() - 0.5) * 0.02;
          const size = 0.0005 + Math.random() * 0.001;
          return {
            type: "Feature" as const,
            geometry: {
              type: "Polygon" as const,
              coordinates: [[
                [x - size, y - size],
                [x + size, y - size],
                [x + size, y + size],
                [x - size, y + size],
                [x - size, y - size],
              ]],
            },
            properties: {
              height: 50 + Math.random() * 200,
              base: 0,
              color: `hsl(${200 + Math.random() * 40}, 50%, ${40 + Math.random() * 20}%)`,
            },
          };
        }),
      };

      map.addSource("demo-buildings", {
        type: "geojson",
        data: buildings,
      });

      map.addLayer({
        id: "demo-buildings-3d",
        type: "fill-extrusion",
        source: "demo-buildings",
        paint: {
          "fill-extrusion-color": ["get", "color"],
          "fill-extrusion-height": ["get", "height"],
          "fill-extrusion-base": ["get", "base"],
          "fill-extrusion-opacity": 0.85,
        },
      });

      map.easeTo({ pitch: 60, bearing: -17.6, zoom: 15, duration: 1500 });

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>3D Buildings</h4>
        <div class="stat"><span class="label">Layer Type</span><span class="value">fill-extrusion</span></div>
        <div class="stat"><span class="label">Buildings</span><span class="value">20</span></div>
        <div class="stat"><span class="label">Height</span><span class="value">50-250m (data-driven)</span></div>
        <p style="margin-top: 0.5rem; color: #888; font-size: 0.75rem;">Drag to rotate and see the 3D effect!</p>
      `;
      container.appendChild(info);

      return () => {
        if (map.getLayer("demo-buildings-3d")) map.removeLayer("demo-buildings-3d");
        if (map.getSource("demo-buildings")) map.removeSource("demo-buildings");
        info.remove();
      };
    },
  },

  {
    id: "pitch-view",
    title: "Pitch & 3D View",
    description: "Control camera pitch for 3D perspective",
    category: "3d",
    code: `// Set pitch (0-85 degrees, up to 180 with globe)
map.setPitch(60);

// Animate pitch change
map.easeTo({ pitch: 45, duration: 1000 });

// Get current pitch
const pitch = map.getPitch();`,
    run: (map, container) => {
      map.easeTo({ pitch: 45, duration: 1000 });

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Pitch Control</h4>
        <div class="stat"><span class="label">Pitch</span><span class="value" id="pitch-val">${map.getPitch().toFixed(1)}°</span></div>
        <div style="margin-top: 0.5rem;">
          <input type="range" id="pitch-slider" min="0" max="85" value="${map.getPitch()}" style="width: 100%;">
        </div>
        <div style="display: flex; gap: 0.25rem; margin-top: 0.5rem;">
          <button class="pitch-btn" data-pitch="0" style="flex: 1; padding: 0.4rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px; cursor: pointer;">0°</button>
          <button class="pitch-btn" data-pitch="45" style="flex: 1; padding: 0.4rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px; cursor: pointer;">45°</button>
          <button class="pitch-btn" data-pitch="60" style="flex: 1; padding: 0.4rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px; cursor: pointer;">60°</button>
          <button class="pitch-btn" data-pitch="85" style="flex: 1; padding: 0.4rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px; cursor: pointer;">85°</button>
        </div>
      `;
      container.appendChild(info);

      const updatePitch = () => {
        document.getElementById("pitch-val")!.textContent = `${map.getPitch().toFixed(1)}°`;
        (document.getElementById("pitch-slider") as HTMLInputElement).value = map.getPitch().toString();
      };

      map.on("pitch", updatePitch);

      document.getElementById("pitch-slider")!.addEventListener("input", (e) => {
        map.setPitch(parseInt((e.target as HTMLInputElement).value));
      });

      info.querySelectorAll(".pitch-btn").forEach((btn) => {
        btn.addEventListener("click", () => {
          const pitch = parseInt((btn as HTMLElement).dataset.pitch!);
          map.easeTo({ pitch, duration: 500 });
        });
      });

      return () => {
        map.off("pitch", updatePitch);
        info.remove();
      };
    },
  },

  {
    id: "globe-projection",
    title: "Globe Projection",
    description: "View map as a 3D globe",
    category: "3d",
    code: `// Switch to globe projection
map.setProjection({ type: 'globe' });

// Switch back to mercator
map.setProjection({ type: 'mercator' });

// Check if using globe
const isGlobe = map.isGlobeProjection();`,
    run: (map, container) => {
      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Globe Projection</h4>
        <div class="stat"><span class="label">Projection</span><span class="value" id="proj-type">mercator</span></div>
        <div style="display: flex; flex-direction: column; gap: 0.25rem; margin-top: 0.5rem;">
          <button id="set-globe" style="padding: 0.4rem; background: #e94560; border: none; color: white; border-radius: 4px; cursor: pointer;">Switch to Globe</button>
          <button id="set-mercator" style="padding: 0.4rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px; cursor: pointer;">Switch to Mercator</button>
        </div>
        <p style="margin-top: 0.5rem; color: #888; font-size: 0.75rem;">Globe projection was added in MapLibre GL JS v5.0.</p>
      `;
      container.appendChild(info);

      const updateProjection = () => {
        const proj = map.getProjection();
        document.getElementById("proj-type")!.textContent = proj?.type || "mercator";
      };

      document.getElementById("set-globe")!.addEventListener("click", () => {
        map.setProjection({ type: "globe" });
        map.easeTo({ zoom: 2, pitch: 30, duration: 1500 });
        updateProjection();
      });

      document.getElementById("set-mercator")!.addEventListener("click", () => {
        map.setProjection({ type: "mercator" });
        map.easeTo({ zoom: 12, pitch: 0, center: [-74.006, 40.7128], duration: 1500 });
        updateProjection();
      });

      return () => {
        map.setProjection({ type: "mercator" });
        info.remove();
      };
    },
  },

  {
    id: "projections",
    title: "Map Projections",
    description: "Different map projection types",
    category: "3d",
    code: `// Available projections:
// mercator, globe, albers, equalEarth,
// equirectangular, lambertConformalConic,
// naturalEarth, winkelTripel, verticalPerspective

map.setProjection({ type: 'equalEarth' });`,
    run: (map, container) => {
      const projections = [
        { type: "mercator", name: "Mercator", zoom: 2 },
        { type: "globe", name: "Globe", zoom: 2 },
        { type: "equalEarth", name: "Equal Earth", zoom: 1 },
        { type: "naturalEarth", name: "Natural Earth", zoom: 1 },
        { type: "winkelTripel", name: "Winkel Tripel", zoom: 1 },
        { type: "equirectangular", name: "Equirectangular", zoom: 1 },
      ];

      map.easeTo({ center: [0, 20], zoom: 2, pitch: 0, bearing: 0, duration: 1000 });

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Map Projections</h4>
        <div class="stat"><span class="label">Current</span><span class="value" id="current-proj">mercator</span></div>
        <div style="display: flex; flex-direction: column; gap: 0.25rem; margin-top: 0.5rem; max-height: 200px; overflow-y: auto;">
          ${projections.map((p) => `
            <button class="proj-btn" data-proj="${p.type}" data-zoom="${p.zoom}" style="padding: 0.4rem; background: ${p.type === "mercator" ? "#e94560" : "#0f3460"}; border: ${p.type === "mercator" ? "none" : "1px solid #16213e"}; color: ${p.type === "mercator" ? "white" : "#ccc"}; border-radius: 4px; cursor: pointer; text-align: left;">
              ${p.name}
            </button>
          `).join("")}
        </div>
      `;
      container.appendChild(info);

      info.querySelectorAll(".proj-btn").forEach((btn) => {
        btn.addEventListener("click", () => {
          const projType = (btn as HTMLElement).dataset.proj!;
          const zoom = parseFloat((btn as HTMLElement).dataset.zoom!);

          map.setProjection({ type: projType as any });
          map.easeTo({ zoom, pitch: projType === "globe" ? 30 : 0, duration: 1000 });

          document.getElementById("current-proj")!.textContent = projType;

          info.querySelectorAll(".proj-btn").forEach((b) => {
            const isActive = b === btn;
            (b as HTMLElement).style.background = isActive ? "#e94560" : "#0f3460";
            (b as HTMLElement).style.border = isActive ? "none" : "1px solid #16213e";
            (b as HTMLElement).style.color = isActive ? "white" : "#ccc";
          });
        });
      });

      return () => {
        map.setProjection({ type: "mercator" });
        map.easeTo({ center: [-74.006, 40.7128], zoom: 12, duration: 500 });
        info.remove();
      };
    },
  },

  {
    id: "lighting",
    title: "3D Lighting",
    description: "Configure light source for 3D layers",
    category: "3d",
    code: `map.setLight({
  anchor: 'viewport', // or 'map'
  color: '#ffffff',
  intensity: 0.5,
  position: [1.15, 210, 30] // [radial, azimuthal, polar]
});`,
    run: (map, container) => {
      // Add buildings first
      const buildings = {
        type: "FeatureCollection" as const,
        features: Array.from({ length: 15 }, (_, i) => {
          const x = -74.006 + (Math.random() - 0.5) * 0.015;
          const y = 40.7128 + (Math.random() - 0.5) * 0.015;
          const size = 0.0006;
          return {
            type: "Feature" as const,
            geometry: {
              type: "Polygon" as const,
              coordinates: [[
                [x - size, y - size],
                [x + size, y - size],
                [x + size, y + size],
                [x - size, y + size],
                [x - size, y - size],
              ]],
            },
            properties: { height: 100 + Math.random() * 150 },
          };
        }),
      };

      map.addSource("demo-lit-buildings", {
        type: "geojson",
        data: buildings,
      });

      map.addLayer({
        id: "demo-lit-buildings-3d",
        type: "fill-extrusion",
        source: "demo-lit-buildings",
        paint: {
          "fill-extrusion-color": "#8ecae6",
          "fill-extrusion-height": ["get", "height"],
          "fill-extrusion-base": 0,
          "fill-extrusion-opacity": 0.9,
        },
      });

      map.easeTo({ pitch: 60, bearing: -30, zoom: 16, duration: 1500 });

      map.setLight({
        anchor: "viewport",
        color: "#ffffff",
        intensity: 0.5,
        position: [1.15, 210, 30],
      });

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>3D Lighting</h4>
        <div style="margin-bottom: 0.5rem;">
          <label style="color: #888; font-size: 0.75rem;">Light Color:</label>
          <input type="color" id="light-color" value="#ffffff" style="width: 100%; height: 25px; border: none; cursor: pointer;">
        </div>
        <div style="margin-bottom: 0.5rem;">
          <label style="color: #888; font-size: 0.75rem;">Intensity: <span id="intensity-val">0.5</span></label>
          <input type="range" id="light-intensity" min="0" max="100" value="50" style="width: 100%;">
        </div>
        <div style="margin-bottom: 0.5rem;">
          <label style="color: #888; font-size: 0.75rem;">Azimuth: <span id="azimuth-val">210°</span></label>
          <input type="range" id="light-azimuth" min="0" max="360" value="210" style="width: 100%;">
        </div>
        <div>
          <label style="color: #888; font-size: 0.75rem;">Polar: <span id="polar-val">30°</span></label>
          <input type="range" id="light-polar" min="0" max="90" value="30" style="width: 100%;">
        </div>
      `;
      container.appendChild(info);

      const updateLight = () => {
        const color = (document.getElementById("light-color") as HTMLInputElement).value;
        const intensity = parseInt((document.getElementById("light-intensity") as HTMLInputElement).value) / 100;
        const azimuth = parseInt((document.getElementById("light-azimuth") as HTMLInputElement).value);
        const polar = parseInt((document.getElementById("light-polar") as HTMLInputElement).value);

        document.getElementById("intensity-val")!.textContent = intensity.toFixed(2);
        document.getElementById("azimuth-val")!.textContent = `${azimuth}°`;
        document.getElementById("polar-val")!.textContent = `${polar}°`;

        map.setLight({
          anchor: "viewport",
          color,
          intensity,
          position: [1.15, azimuth, polar],
        });
      };

      ["light-color", "light-intensity", "light-azimuth", "light-polar"].forEach((id) => {
        document.getElementById(id)!.addEventListener("input", updateLight);
      });

      return () => {
        if (map.getLayer("demo-lit-buildings-3d")) map.removeLayer("demo-lit-buildings-3d");
        if (map.getSource("demo-lit-buildings")) map.removeSource("demo-lit-buildings");
        info.remove();
      };
    },
  },

  {
    id: "sky-atmosphere",
    title: "Sky & Atmosphere",
    description: "Configure sky appearance",
    category: "3d",
    code: `map.setSky({
  'sky-color': '#199EF3',
  'sky-horizon-blend': 0.8,
  'horizon-color': '#ffffff',
  'horizon-fog-blend': 0.5,
  'fog-color': '#ffffff',
  'fog-ground-blend': 0.5,
  'atmosphere-blend': 0.8
});`,
    run: (map, container) => {
      map.setProjection({ type: "globe" });
      map.easeTo({ center: [0, 30], zoom: 1.5, pitch: 30, duration: 1500 });

      map.setSky({
        "sky-color": "#199EF3",
        "sky-horizon-blend": 0.8,
        "horizon-color": "#ffffff",
        "horizon-fog-blend": 0.5,
        "fog-color": "#ffffff",
        "fog-ground-blend": 0.5,
        "atmosphere-blend": 0.8,
      });

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Sky & Atmosphere</h4>
        <div style="margin-bottom: 0.5rem;">
          <label style="color: #888; font-size: 0.75rem;">Sky Color:</label>
          <input type="color" id="sky-color" value="#199EF3" style="width: 100%; height: 25px; border: none; cursor: pointer;">
        </div>
        <div style="margin-bottom: 0.5rem;">
          <label style="color: #888; font-size: 0.75rem;">Horizon Color:</label>
          <input type="color" id="horizon-color" value="#ffffff" style="width: 100%; height: 25px; border: none; cursor: pointer;">
        </div>
        <div style="margin-bottom: 0.5rem;">
          <label style="color: #888; font-size: 0.75rem;">Atmosphere: <span id="atmo-val">0.8</span></label>
          <input type="range" id="atmo-blend" min="0" max="100" value="80" style="width: 100%;">
        </div>
        <p style="margin-top: 0.5rem; color: #888; font-size: 0.75rem;">Best viewed with globe projection.</p>
      `;
      container.appendChild(info);

      const updateSky = () => {
        const skyColor = (document.getElementById("sky-color") as HTMLInputElement).value;
        const horizonColor = (document.getElementById("horizon-color") as HTMLInputElement).value;
        const atmoBlend = parseInt((document.getElementById("atmo-blend") as HTMLInputElement).value) / 100;

        document.getElementById("atmo-val")!.textContent = atmoBlend.toFixed(2);

        map.setSky({
          "sky-color": skyColor,
          "sky-horizon-blend": 0.8,
          "horizon-color": horizonColor,
          "horizon-fog-blend": 0.5,
          "fog-color": horizonColor,
          "fog-ground-blend": 0.5,
          "atmosphere-blend": atmoBlend,
        });
      };

      ["sky-color", "horizon-color", "atmo-blend"].forEach((id) => {
        document.getElementById(id)!.addEventListener("input", updateSky);
      });

      return () => {
        map.setProjection({ type: "mercator" });
        map.easeTo({ center: [-74.006, 40.7128], zoom: 12, pitch: 0, duration: 500 });
        info.remove();
      };
    },
  },

  {
    id: "fog",
    title: "Fog Effect",
    description: "Add atmospheric fog to the map",
    category: "3d",
    code: `map.setFog({
  range: [0.5, 10],
  color: 'white',
  'horizon-blend': 0.1,
  'high-color': '#add8e6',
  'space-color': '#000033',
  'star-intensity': 0.5
});`,
    run: (map, container) => {
      map.easeTo({ pitch: 60, zoom: 14, duration: 1500 });

      map.setFog({
        range: [0.5, 10],
        color: "rgba(186, 210, 235, 0.8)",
        "horizon-blend": 0.1,
        "high-color": "#add8e6",
        "space-color": "#000033",
        "star-intensity": 0.15,
      });

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Fog Effect</h4>
        <div class="stat"><span class="label">Range</span><span class="value">[0.5, 10]</span></div>
        <div style="margin-top: 0.5rem;">
          <label style="color: #888; font-size: 0.75rem;">Fog Color:</label>
          <input type="color" id="fog-color" value="#bad2eb" style="width: 100%; height: 25px; border: none; cursor: pointer;">
        </div>
        <div style="margin-top: 0.5rem;">
          <label style="color: #888; font-size: 0.75rem;">Star Intensity: <span id="star-val">0.15</span></label>
          <input type="range" id="star-intensity" min="0" max="100" value="15" style="width: 100%;">
        </div>
        <p style="margin-top: 0.5rem; color: #888; font-size: 0.75rem;">Pitch the view to see fog effect.</p>
      `;
      container.appendChild(info);

      document.getElementById("fog-color")!.addEventListener("input", (e) => {
        map.setFog({
          ...map.getFog(),
          color: (e.target as HTMLInputElement).value,
        });
      });

      document.getElementById("star-intensity")!.addEventListener("input", (e) => {
        const val = parseInt((e.target as HTMLInputElement).value) / 100;
        document.getElementById("star-val")!.textContent = val.toFixed(2);
        map.setFog({
          ...map.getFog(),
          "star-intensity": val,
        });
      });

      return () => {
        map.setFog(null as any);
        info.remove();
      };
    },
  },

  {
    id: "free-camera",
    title: "Free Camera",
    description: "Advanced 3D camera control",
    category: "3d",
    code: `const camera = map.getFreeCameraOptions();

// Set camera position (LngLat + altitude)
camera.position = maplibregl.MercatorCoordinate
  .fromLngLat([-74.006, 40.7128], 1000);

// Set camera orientation
camera.lookAtPoint([-74.006, 40.7128]);

map.setFreeCameraOptions(camera);`,
    run: (map, container) => {
      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Free Camera</h4>
        <div class="stat"><span class="label">Altitude</span><span class="value" id="cam-alt">-</span></div>
        <div style="display: flex; flex-direction: column; gap: 0.25rem; margin-top: 0.5rem;">
          <button id="cam-high" style="padding: 0.4rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px; cursor: pointer;">Bird's Eye (10km)</button>
          <button id="cam-mid" style="padding: 0.4rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px; cursor: pointer;">City View (1km)</button>
          <button id="cam-low" style="padding: 0.4rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px; cursor: pointer;">Street Level (100m)</button>
          <button id="cam-orbit" style="padding: 0.4rem; background: #e94560; border: none; color: white; border-radius: 4px; cursor: pointer;">Start Orbit Animation</button>
        </div>
      `;
      container.appendChild(info);

      const setCameraAltitude = (altitude: number) => {
        const camera = map.getFreeCameraOptions();
        camera.position = maplibregl.MercatorCoordinate.fromLngLat([-74.006, 40.7128], altitude);
        camera.lookAtPoint([-74.006, 40.7128]);
        map.setFreeCameraOptions(camera);
        document.getElementById("cam-alt")!.textContent = `${altitude}m`;
      };

      document.getElementById("cam-high")!.addEventListener("click", () => setCameraAltitude(10000));
      document.getElementById("cam-mid")!.addEventListener("click", () => setCameraAltitude(1000));
      document.getElementById("cam-low")!.addEventListener("click", () => setCameraAltitude(100));

      let animationId: number | null = null;
      let angle = 0;

      document.getElementById("cam-orbit")!.addEventListener("click", () => {
        const btn = document.getElementById("cam-orbit")!;
        if (animationId !== null) {
          cancelAnimationFrame(animationId);
          animationId = null;
          btn.textContent = "Start Orbit Animation";
          return;
        }

        btn.textContent = "Stop Orbit Animation";
        const center = [-74.006, 40.7128] as [number, number];
        const radius = 0.01;
        const altitude = 500;

        const animate = () => {
          angle += 0.005;
          const x = center[0] + Math.cos(angle) * radius;
          const y = center[1] + Math.sin(angle) * radius;

          const camera = map.getFreeCameraOptions();
          camera.position = maplibregl.MercatorCoordinate.fromLngLat([x, y], altitude);
          camera.lookAtPoint(center);
          map.setFreeCameraOptions(camera);

          animationId = requestAnimationFrame(animate);
        };

        animate();
      });

      return () => {
        if (animationId !== null) {
          cancelAnimationFrame(animationId);
        }
        map.easeTo({ center: [-74.006, 40.7128], zoom: 12, pitch: 0, bearing: 0, duration: 500 });
        info.remove();
      };
    },
  },
];
