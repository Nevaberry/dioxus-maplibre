import { type Demo } from "../types.ts";

declare const maplibregl: typeof import("maplibre-gl");

export const navigationDemos: Demo[] = [
  {
    id: "fly-to",
    title: "Fly To",
    description: "Animated flight-path transition to new location",
    category: "navigation",
    code: `map.flyTo({
  center: [-122.4194, 37.7749], // San Francisco
  zoom: 14,
  pitch: 45,
  bearing: 0,
  duration: 3000,
  essential: true
});`,
    run: (map, container) => {
      const locations = [
        { name: "New York", center: [-74.006, 40.7128] as [number, number], zoom: 14 },
        { name: "San Francisco", center: [-122.4194, 37.7749] as [number, number], zoom: 14 },
        { name: "London", center: [-0.1276, 51.5074] as [number, number], zoom: 13 },
        { name: "Tokyo", center: [139.6917, 35.6895] as [number, number], zoom: 13 },
      ];
      let currentIndex = 0;

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Fly To</h4>
        <div class="stat"><span class="label">Destination</span><span class="value" id="fly-dest">${locations[0].name}</span></div>
        <button id="fly-btn" style="margin-top: 0.5rem; padding: 0.5rem 1rem; background: #e94560; border: none; color: white; border-radius: 4px; cursor: pointer; width: 100%;">
          Fly to Next City
        </button>
      `;
      container.appendChild(info);

      const btn = document.getElementById("fly-btn")!;
      btn.addEventListener("click", () => {
        currentIndex = (currentIndex + 1) % locations.length;
        const loc = locations[currentIndex];
        document.getElementById("fly-dest")!.textContent = loc.name;

        map.flyTo({
          center: loc.center,
          zoom: loc.zoom,
          pitch: 45,
          bearing: Math.random() * 60 - 30,
          duration: 3000,
          essential: true,
        });
      });

      return () => info.remove();
    },
  },

  {
    id: "ease-to",
    title: "Ease To",
    description: "Smooth linear transition (shorter distance animations)",
    category: "navigation",
    code: `map.easeTo({
  center: [-74.01, 40.72],
  zoom: 14,
  pitch: 30,
  duration: 1500,
  easing: (t) => t * (2 - t) // ease-out quad
});`,
    run: (map, container) => {
      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Ease To</h4>
        <p style="color: #888; font-size: 0.75rem; margin-bottom: 0.5rem;">
          Click anywhere on the map to ease to that location.
        </p>
        <div class="stat"><span class="label">Last Click</span><span class="value" id="ease-last">-</span></div>
      `;
      container.appendChild(info);

      const handleClick = (e: maplibregl.MapMouseEvent) => {
        const { lng, lat } = e.lngLat;
        document.getElementById("ease-last")!.textContent = `${lng.toFixed(3)}, ${lat.toFixed(3)}`;

        map.easeTo({
          center: [lng, lat],
          zoom: map.getZoom() + 0.5,
          duration: 1500,
          easing: (t) => t * (2 - t),
        });
      };

      map.on("click", handleClick);

      return () => {
        map.off("click", handleClick);
        info.remove();
      };
    },
  },

  {
    id: "jump-to",
    title: "Jump To",
    description: "Instant camera change (no animation)",
    category: "navigation",
    code: `map.jumpTo({
  center: [-74.01, 40.72],
  zoom: 16,
  pitch: 60,
  bearing: 45
});`,
    run: (map, container) => {
      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Jump To</h4>
        <p style="color: #888; font-size: 0.75rem; margin-bottom: 0.5rem;">
          Instant transitions - no animation.
        </p>
        <button id="jump-btn" style="margin-top: 0.5rem; padding: 0.5rem 1rem; background: #e94560; border: none; color: white; border-radius: 4px; cursor: pointer; width: 100%;">
          Jump to Random Position
        </button>
      `;
      container.appendChild(info);

      const btn = document.getElementById("jump-btn")!;
      btn.addEventListener("click", () => {
        map.jumpTo({
          center: [-74.006 + (Math.random() - 0.5) * 0.1, 40.7128 + (Math.random() - 0.5) * 0.1],
          zoom: 10 + Math.random() * 6,
          pitch: Math.random() * 60,
          bearing: Math.random() * 360,
        });
      });

      return () => info.remove();
    },
  },

  {
    id: "fit-bounds",
    title: "Fit Bounds",
    description: "Fit view to contain geographic bounds",
    category: "navigation",
    code: `map.fitBounds([
  [-74.2591, 40.4774], // SW corner
  [-73.7002, 40.9162]  // NE corner
], {
  padding: { top: 50, bottom: 50, left: 50, right: 50 },
  maxZoom: 15,
  duration: 2000
});`,
    run: (map, container) => {
      const regions = [
        { name: "Manhattan", bounds: [[-74.0479, 40.6829], [-73.9067, 40.8820]] as [[number, number], [number, number]] },
        { name: "Brooklyn", bounds: [[-74.0419, 40.5707], [-73.8552, 40.7395]] as [[number, number], [number, number]] },
        { name: "Queens", bounds: [[-73.9626, 40.5431], [-73.7004, 40.8007]] as [[number, number], [number, number]] },
        { name: "All NYC", bounds: [[-74.2591, 40.4774], [-73.7002, 40.9162]] as [[number, number], [number, number]] },
      ];

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Fit Bounds</h4>
        <div style="display: flex; flex-direction: column; gap: 0.25rem; margin-top: 0.5rem;">
          ${regions
            .map(
              (r) => `
            <button class="fit-region" data-region="${r.name}" style="padding: 0.4rem 0.75rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px; cursor: pointer; text-align: left;">
              ${r.name}
            </button>
          `
            )
            .join("")}
        </div>
      `;
      container.appendChild(info);

      info.querySelectorAll(".fit-region").forEach((btn) => {
        btn.addEventListener("click", () => {
          const regionName = (btn as HTMLElement).dataset.region;
          const region = regions.find((r) => r.name === regionName);
          if (region) {
            map.fitBounds(region.bounds, {
              padding: 50,
              maxZoom: 15,
              duration: 2000,
            });
          }
        });
      });

      return () => info.remove();
    },
  },

  {
    id: "pan-by",
    title: "Pan By",
    description: "Pan by pixel offset",
    category: "navigation",
    code: `// Pan 100 pixels right and 50 pixels down
map.panBy([100, 50], {
  duration: 500,
  easing: (t) => t
});`,
    run: (map, container) => {
      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Pan By Pixels</h4>
        <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 0.25rem; margin-top: 0.5rem;">
          <div></div>
          <button class="pan-dir" data-dir="up" style="padding: 0.5rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px; cursor: pointer;">Up</button>
          <div></div>
          <button class="pan-dir" data-dir="left" style="padding: 0.5rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px; cursor: pointer;">Left</button>
          <button class="pan-dir" data-dir="center" style="padding: 0.5rem; background: #e94560; border: none; color: white; border-radius: 4px; cursor: pointer;">Reset</button>
          <button class="pan-dir" data-dir="right" style="padding: 0.5rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px; cursor: pointer;">Right</button>
          <div></div>
          <button class="pan-dir" data-dir="down" style="padding: 0.5rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px; cursor: pointer;">Down</button>
          <div></div>
        </div>
      `;
      container.appendChild(info);

      const offsets: Record<string, [number, number]> = {
        up: [0, -100],
        down: [0, 100],
        left: [-100, 0],
        right: [100, 0],
      };

      info.querySelectorAll(".pan-dir").forEach((btn) => {
        btn.addEventListener("click", () => {
          const dir = (btn as HTMLElement).dataset.dir;
          if (dir === "center") {
            map.easeTo({ center: [-74.006, 40.7128], zoom: 12, duration: 500 });
          } else if (dir && offsets[dir]) {
            map.panBy(offsets[dir], { duration: 500 });
          }
        });
      });

      return () => info.remove();
    },
  },

  {
    id: "zoom-methods",
    title: "Zoom Methods",
    description: "zoomTo, zoomIn, zoomOut",
    category: "navigation",
    code: `map.zoomTo(15, { duration: 1000 });
map.zoomIn({ duration: 500 });
map.zoomOut({ duration: 500 });`,
    run: (map, container) => {
      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Zoom Controls</h4>
        <div class="stat"><span class="label">Current Zoom</span><span class="value" id="zoom-level">${map.getZoom().toFixed(2)}</span></div>
        <div style="display: flex; gap: 0.25rem; margin-top: 0.5rem;">
          <button id="zoom-out" style="flex: 1; padding: 0.5rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px; cursor: pointer; font-size: 1.2rem;">-</button>
          <button id="zoom-reset" style="flex: 2; padding: 0.5rem; background: #e94560; border: none; color: white; border-radius: 4px; cursor: pointer;">12</button>
          <button id="zoom-in" style="flex: 1; padding: 0.5rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px; cursor: pointer; font-size: 1.2rem;">+</button>
        </div>
      `;
      container.appendChild(info);

      const updateZoom = () => {
        const el = document.getElementById("zoom-level");
        if (el) el.textContent = map.getZoom().toFixed(2);
      };

      map.on("zoom", updateZoom);

      document.getElementById("zoom-in")!.addEventListener("click", () => map.zoomIn({ duration: 500 }));
      document.getElementById("zoom-out")!.addEventListener("click", () => map.zoomOut({ duration: 500 }));
      document.getElementById("zoom-reset")!.addEventListener("click", () => map.zoomTo(12, { duration: 500 }));

      return () => {
        map.off("zoom", updateZoom);
        info.remove();
      };
    },
  },

  {
    id: "rotate-methods",
    title: "Rotate & Reset",
    description: "rotateTo, resetNorth, resetNorthPitch",
    category: "navigation",
    code: `map.rotateTo(45, { duration: 1000 });
map.resetNorth({ duration: 500 });
map.resetNorthPitch({ duration: 500 });`,
    run: (map, container) => {
      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Rotation</h4>
        <div class="stat"><span class="label">Bearing</span><span class="value" id="rot-bearing">${map.getBearing().toFixed(1)}°</span></div>
        <div class="stat"><span class="label">Pitch</span><span class="value" id="rot-pitch">${map.getPitch().toFixed(1)}°</span></div>
        <div style="display: flex; flex-direction: column; gap: 0.25rem; margin-top: 0.5rem;">
          <button id="rotate-cw" style="padding: 0.4rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px; cursor: pointer;">Rotate +45°</button>
          <button id="rotate-ccw" style="padding: 0.4rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px; cursor: pointer;">Rotate -45°</button>
          <button id="pitch-up" style="padding: 0.4rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px; cursor: pointer;">Pitch +20°</button>
          <button id="reset-north" style="padding: 0.4rem; background: #e94560; border: none; color: white; border-radius: 4px; cursor: pointer;">Reset North</button>
          <button id="reset-all" style="padding: 0.4rem; background: #e94560; border: none; color: white; border-radius: 4px; cursor: pointer;">Reset North & Pitch</button>
        </div>
      `;
      container.appendChild(info);

      const updateDisplay = () => {
        document.getElementById("rot-bearing")!.textContent = `${map.getBearing().toFixed(1)}°`;
        document.getElementById("rot-pitch")!.textContent = `${map.getPitch().toFixed(1)}°`;
      };

      map.on("rotate", updateDisplay);
      map.on("pitch", updateDisplay);

      document.getElementById("rotate-cw")!.addEventListener("click", () => {
        map.rotateTo(map.getBearing() + 45, { duration: 500 });
      });
      document.getElementById("rotate-ccw")!.addEventListener("click", () => {
        map.rotateTo(map.getBearing() - 45, { duration: 500 });
      });
      document.getElementById("pitch-up")!.addEventListener("click", () => {
        map.easeTo({ pitch: Math.min(85, map.getPitch() + 20), duration: 500 });
      });
      document.getElementById("reset-north")!.addEventListener("click", () => {
        map.resetNorth({ duration: 500 });
      });
      document.getElementById("reset-all")!.addEventListener("click", () => {
        map.resetNorthPitch({ duration: 500 });
      });

      return () => {
        map.off("rotate", updateDisplay);
        map.off("pitch", updateDisplay);
        info.remove();
      };
    },
  },

  {
    id: "stop-animation",
    title: "Stop Animation",
    description: "Cancel ongoing animations with stop()",
    category: "navigation",
    code: `// Start a long animation
map.flyTo({
  center: [139.6917, 35.6895], // Tokyo
  zoom: 14,
  duration: 10000
});

// Stop it mid-flight
setTimeout(() => map.stop(), 2000);`,
    run: (map, container) => {
      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Animation Control</h4>
        <div class="stat"><span class="label">Status</span><span class="value" id="anim-status">Ready</span></div>
        <div style="display: flex; gap: 0.25rem; margin-top: 0.5rem;">
          <button id="start-anim" style="flex: 1; padding: 0.5rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px; cursor: pointer;">Start Flight</button>
          <button id="stop-anim" style="flex: 1; padding: 0.5rem; background: #e94560; border: none; color: white; border-radius: 4px; cursor: pointer;">Stop</button>
        </div>
      `;
      container.appendChild(info);

      const statusEl = document.getElementById("anim-status")!;

      const updateStatus = () => {
        if (map.isMoving()) {
          statusEl.textContent = "Moving...";
          statusEl.style.color = "#79c0ff";
        } else {
          statusEl.textContent = "Idle";
          statusEl.style.color = "#8b949e";
        }
      };

      map.on("movestart", updateStatus);
      map.on("moveend", updateStatus);

      document.getElementById("start-anim")!.addEventListener("click", () => {
        statusEl.textContent = "Flying...";
        statusEl.style.color = "#79c0ff";
        map.flyTo({
          center: [139.6917, 35.6895],
          zoom: 14,
          duration: 10000,
        });
      });

      document.getElementById("stop-anim")!.addEventListener("click", () => {
        map.stop();
        statusEl.textContent = "Stopped";
        statusEl.style.color = "#e94560";
      });

      return () => {
        map.off("movestart", updateStatus);
        map.off("moveend", updateStatus);
        info.remove();
      };
    },
  },
];
