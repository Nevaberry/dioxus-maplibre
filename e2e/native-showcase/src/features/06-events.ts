import { type Demo } from "../types.ts";

declare const maplibregl: typeof import("maplibre-gl");

export const eventDemos: Demo[] = [
  {
    id: "click-event",
    title: "Click Events",
    description: "Handle map click with coordinates",
    category: "events",
    code: `map.on('click', (e) => {
  console.log('Clicked at:', e.lngLat);
  console.log('Pixel:', e.point);
});`,
    run: (map, container) => {
      const eventLog = document.createElement("div");
      eventLog.id = "event-log";
      eventLog.innerHTML = `<h4>Click Events</h4>`;
      container.appendChild(eventLog);

      const logEvent = (e: maplibregl.MapMouseEvent) => {
        const entry = document.createElement("div");
        entry.className = "event";
        entry.innerHTML = `
          <span class="event-type">click</span>
          lng: ${e.lngLat.lng.toFixed(4)}, lat: ${e.lngLat.lat.toFixed(4)}
          <span class="event-time">${new Date().toLocaleTimeString()}</span>
        `;
        eventLog.insertBefore(entry, eventLog.children[1]);

        // Keep only last 10 events
        while (eventLog.children.length > 11) {
          eventLog.removeChild(eventLog.lastChild!);
        }
      };

      map.on("click", logEvent);

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Click Event</h4>
        <p style="color: #888; font-size: 0.75rem;">Click anywhere on the map to log coordinates.</p>
        <div class="stat"><span class="label">Event</span><span class="value">MapMouseEvent</span></div>
        <div class="stat"><span class="label">Properties</span><span class="value">lngLat, point</span></div>
      `;
      container.appendChild(info);

      return () => {
        map.off("click", logEvent);
        eventLog.remove();
        info.remove();
      };
    },
  },

  {
    id: "move-events",
    title: "Move Events",
    description: "Track camera movement lifecycle",
    category: "events",
    code: `map.on('movestart', () => console.log('Move started'));
map.on('move', () => console.log('Moving...'));
map.on('moveend', () => console.log('Move ended'));`,
    run: (map, container) => {
      const eventLog = document.createElement("div");
      eventLog.id = "event-log";
      eventLog.innerHTML = `<h4>Move Events</h4>`;
      container.appendChild(eventLog);

      let eventCount = { movestart: 0, move: 0, moveend: 0 };

      const logEvent = (type: string) => () => {
        eventCount[type as keyof typeof eventCount]++;
        const entry = document.createElement("div");
        entry.className = "event";
        entry.innerHTML = `
          <span class="event-type">${type}</span>
          <span class="event-time">${new Date().toLocaleTimeString()}</span>
        `;
        eventLog.insertBefore(entry, eventLog.children[1]);
        while (eventLog.children.length > 11) {
          eventLog.removeChild(eventLog.lastChild!);
        }
      };

      const movestart = logEvent("movestart");
      const move = logEvent("move");
      const moveend = logEvent("moveend");

      map.on("movestart", movestart);
      map.on("move", move);
      map.on("moveend", moveend);

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Move Events</h4>
        <p style="color: #888; font-size: 0.75rem;">Pan or zoom the map to see events.</p>
        <div class="stat"><span class="label">Lifecycle</span><span class="value">start → move → end</span></div>
      `;
      container.appendChild(info);

      return () => {
        map.off("movestart", movestart);
        map.off("move", move);
        map.off("moveend", moveend);
        eventLog.remove();
        info.remove();
      };
    },
  },

  {
    id: "zoom-events",
    title: "Zoom Events",
    description: "Track zoom changes",
    category: "events",
    code: `map.on('zoomstart', () => console.log('Zoom started'));
map.on('zoom', () => console.log('Zoom:', map.getZoom()));
map.on('zoomend', () => console.log('Zoom ended'));`,
    run: (map, container) => {
      const eventLog = document.createElement("div");
      eventLog.id = "event-log";
      eventLog.innerHTML = `<h4>Zoom Events</h4>`;
      container.appendChild(eventLog);

      const logEvent = (type: string) => () => {
        const zoom = map.getZoom().toFixed(2);
        const entry = document.createElement("div");
        entry.className = "event";
        entry.innerHTML = `
          <span class="event-type">${type}</span>
          zoom: ${zoom}
          <span class="event-time">${new Date().toLocaleTimeString()}</span>
        `;
        eventLog.insertBefore(entry, eventLog.children[1]);
        while (eventLog.children.length > 11) {
          eventLog.removeChild(eventLog.lastChild!);
        }
      };

      const zoomstart = logEvent("zoomstart");
      const zoom = logEvent("zoom");
      const zoomend = logEvent("zoomend");

      map.on("zoomstart", zoomstart);
      map.on("zoom", zoom);
      map.on("zoomend", zoomend);

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Zoom Events</h4>
        <p style="color: #888; font-size: 0.75rem;">Scroll or use controls to zoom.</p>
        <div class="stat"><span class="label">Current Zoom</span><span class="value" id="zoom-val">${map.getZoom().toFixed(2)}</span></div>
      `;
      container.appendChild(info);

      map.on("zoom", () => {
        const el = document.getElementById("zoom-val");
        if (el) el.textContent = map.getZoom().toFixed(2);
      });

      return () => {
        map.off("zoomstart", zoomstart);
        map.off("zoom", zoom);
        map.off("zoomend", zoomend);
        eventLog.remove();
        info.remove();
      };
    },
  },

  {
    id: "layer-events",
    title: "Layer Events",
    description: "Handle clicks on specific layers",
    category: "events",
    code: `map.on('click', 'my-layer', (e) => {
  const feature = e.features[0];
  console.log('Clicked feature:', feature.properties);
});

map.on('mouseenter', 'my-layer', () => {
  map.getCanvas().style.cursor = 'pointer';
});`,
    run: (map, container) => {
      // Add interactive circles
      const points = Array.from({ length: 20 }, (_, i) => ({
        type: "Feature" as const,
        geometry: {
          type: "Point" as const,
          coordinates: [
            -74.006 + (Math.random() - 0.5) * 0.08,
            40.7128 + (Math.random() - 0.5) * 0.08,
          ],
        },
        properties: {
          id: i,
          name: `Point ${i + 1}`,
        },
      }));

      map.addSource("demo-interactive", {
        type: "geojson",
        data: { type: "FeatureCollection", features: points },
      });

      map.addLayer({
        id: "demo-interactive-circles",
        type: "circle",
        source: "demo-interactive",
        paint: {
          "circle-radius": 12,
          "circle-color": "#e94560",
          "circle-opacity": 0.8,
        },
      });

      const eventLog = document.createElement("div");
      eventLog.id = "event-log";
      eventLog.innerHTML = `<h4>Layer Events</h4>`;
      container.appendChild(eventLog);

      const logClick = (e: maplibregl.MapMouseEvent & { features?: maplibregl.MapGeoJSONFeature[] }) => {
        if (!e.features || e.features.length === 0) return;
        const feature = e.features[0];
        const entry = document.createElement("div");
        entry.className = "event";
        entry.innerHTML = `
          <span class="event-type">click</span>
          ${feature.properties?.name}
          <span class="event-time">${new Date().toLocaleTimeString()}</span>
        `;
        eventLog.insertBefore(entry, eventLog.children[1]);
        while (eventLog.children.length > 11) {
          eventLog.removeChild(eventLog.lastChild!);
        }
      };

      const mouseenter = () => {
        map.getCanvas().style.cursor = "pointer";
      };

      const mouseleave = () => {
        map.getCanvas().style.cursor = "";
      };

      map.on("click", "demo-interactive-circles", logClick);
      map.on("mouseenter", "demo-interactive-circles", mouseenter);
      map.on("mouseleave", "demo-interactive-circles", mouseleave);

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Layer Events</h4>
        <p style="color: #888; font-size: 0.75rem;">Click on any circle to see layer-specific events.</p>
        <div class="stat"><span class="label">Layer</span><span class="value">demo-interactive-circles</span></div>
      `;
      container.appendChild(info);

      return () => {
        map.off("click", "demo-interactive-circles", logClick);
        map.off("mouseenter", "demo-interactive-circles", mouseenter);
        map.off("mouseleave", "demo-interactive-circles", mouseleave);
        if (map.getLayer("demo-interactive-circles")) map.removeLayer("demo-interactive-circles");
        if (map.getSource("demo-interactive")) map.removeSource("demo-interactive");
        eventLog.remove();
        info.remove();
      };
    },
  },

  {
    id: "hover-events",
    title: "Hover Effects",
    description: "Change styles on hover using feature state",
    category: "events",
    code: `let hoveredId = null;

map.on('mousemove', 'layer', (e) => {
  if (hoveredId !== null) {
    map.setFeatureState({ source: 'src', id: hoveredId }, { hover: false });
  }
  hoveredId = e.features[0].id;
  map.setFeatureState({ source: 'src', id: hoveredId }, { hover: true });
});`,
    run: (map, container) => {
      // Add circles with IDs for feature state
      const points = Array.from({ length: 15 }, (_, i) => ({
        type: "Feature" as const,
        id: i,
        geometry: {
          type: "Point" as const,
          coordinates: [
            -74.006 + (Math.random() - 0.5) * 0.06,
            40.7128 + (Math.random() - 0.5) * 0.06,
          ],
        },
        properties: { name: `Point ${i + 1}` },
      }));

      map.addSource("demo-hover", {
        type: "geojson",
        data: { type: "FeatureCollection", features: points },
      });

      map.addLayer({
        id: "demo-hover-circles",
        type: "circle",
        source: "demo-hover",
        paint: {
          "circle-radius": [
            "case",
            ["boolean", ["feature-state", "hover"], false],
            18,
            12,
          ],
          "circle-color": [
            "case",
            ["boolean", ["feature-state", "hover"], false],
            "#2ecc71",
            "#e94560",
          ],
          "circle-opacity": 0.8,
          "circle-stroke-width": [
            "case",
            ["boolean", ["feature-state", "hover"], false],
            3,
            0,
          ],
          "circle-stroke-color": "#fff",
        },
      });

      let hoveredId: number | null = null;

      const mousemove = (e: maplibregl.MapMouseEvent & { features?: maplibregl.MapGeoJSONFeature[] }) => {
        if (!e.features || e.features.length === 0) return;

        if (hoveredId !== null) {
          map.setFeatureState({ source: "demo-hover", id: hoveredId }, { hover: false });
        }

        hoveredId = e.features[0].id as number;
        map.setFeatureState({ source: "demo-hover", id: hoveredId }, { hover: true });
        map.getCanvas().style.cursor = "pointer";
      };

      const mouseleave = () => {
        if (hoveredId !== null) {
          map.setFeatureState({ source: "demo-hover", id: hoveredId }, { hover: false });
        }
        hoveredId = null;
        map.getCanvas().style.cursor = "";
      };

      map.on("mousemove", "demo-hover-circles", mousemove);
      map.on("mouseleave", "demo-hover-circles", mouseleave);

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Hover Effects</h4>
        <p style="color: #888; font-size: 0.75rem;">Hover over circles to see feature state changes.</p>
        <div class="stat"><span class="label">Method</span><span class="value">setFeatureState</span></div>
        <div class="stat"><span class="label">Hovered</span><span class="value" id="hovered-id">none</span></div>
      `;
      container.appendChild(info);

      map.on("mousemove", "demo-hover-circles", (e) => {
        if (e.features && e.features.length > 0) {
          document.getElementById("hovered-id")!.textContent = e.features[0].properties?.name || "unknown";
        }
      });

      map.on("mouseleave", "demo-hover-circles", () => {
        document.getElementById("hovered-id")!.textContent = "none";
      });

      return () => {
        map.off("mousemove", "demo-hover-circles", mousemove);
        map.off("mouseleave", "demo-hover-circles", mouseleave);
        if (map.getLayer("demo-hover-circles")) map.removeLayer("demo-hover-circles");
        if (map.getSource("demo-hover")) map.removeSource("demo-hover");
        info.remove();
      };
    },
  },

  {
    id: "lifecycle-events",
    title: "Lifecycle Events",
    description: "Track map loading and idle states",
    category: "events",
    code: `map.on('load', () => console.log('Map loaded'));
map.on('idle', () => console.log('Map idle'));
map.on('render', () => console.log('Rendered'));`,
    run: (map, container) => {
      const eventLog = document.createElement("div");
      eventLog.id = "event-log";
      eventLog.innerHTML = `<h4>Lifecycle Events</h4>`;
      container.appendChild(eventLog);

      let renderCount = 0;

      const logEvent = (type: string, extra?: string) => () => {
        if (type === "render") {
          renderCount++;
          if (renderCount % 10 !== 0) return; // Log every 10th render
        }
        const entry = document.createElement("div");
        entry.className = "event";
        entry.innerHTML = `
          <span class="event-type">${type}</span>
          ${extra || ""}
          <span class="event-time">${new Date().toLocaleTimeString()}</span>
        `;
        eventLog.insertBefore(entry, eventLog.children[1]);
        while (eventLog.children.length > 11) {
          eventLog.removeChild(eventLog.lastChild!);
        }
      };

      const idle = logEvent("idle");
      const render = logEvent("render", "(every 10th)");

      map.on("idle", idle);
      map.on("render", render);

      // Log initial load
      logEvent("load", "(already loaded)")();

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Lifecycle Events</h4>
        <div class="stat"><span class="label">load</span><span class="value">Fired once</span></div>
        <div class="stat"><span class="label">idle</span><span class="value">No pending work</span></div>
        <div class="stat"><span class="label">render</span><span class="value">Every frame</span></div>
        <p style="margin-top: 0.5rem; color: #888; font-size: 0.75rem;">Pan/zoom to trigger events.</p>
      `;
      container.appendChild(info);

      return () => {
        map.off("idle", idle);
        map.off("render", render);
        eventLog.remove();
        info.remove();
      };
    },
  },

  {
    id: "data-events",
    title: "Data Events",
    description: "Track source and style data loading",
    category: "events",
    code: `map.on('sourcedata', (e) => {
  if (e.isSourceLoaded) {
    console.log('Source loaded:', e.sourceId);
  }
});

map.on('styledata', () => {
  console.log('Style data updated');
});`,
    run: (map, container) => {
      const eventLog = document.createElement("div");
      eventLog.id = "event-log";
      eventLog.innerHTML = `<h4>Data Events</h4>`;
      container.appendChild(eventLog);

      const logEvent = (type: string, detail: string) => {
        const entry = document.createElement("div");
        entry.className = "event";
        entry.innerHTML = `
          <span class="event-type">${type}</span>
          ${detail}
          <span class="event-time">${new Date().toLocaleTimeString()}</span>
        `;
        eventLog.insertBefore(entry, eventLog.children[1]);
        while (eventLog.children.length > 11) {
          eventLog.removeChild(eventLog.lastChild!);
        }
      };

      const sourcedata = (e: maplibregl.MapSourceDataEvent) => {
        if (e.isSourceLoaded) {
          logEvent("sourcedata", e.sourceId || "unknown");
        }
      };

      const styledata = () => {
        logEvent("styledata", "style updated");
      };

      map.on("sourcedata", sourcedata);
      map.on("styledata", styledata);

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Data Events</h4>
        <div class="stat"><span class="label">sourcedata</span><span class="value">Source loaded/changed</span></div>
        <div class="stat"><span class="label">styledata</span><span class="value">Style updated</span></div>
        <p style="margin-top: 0.5rem; color: #888; font-size: 0.75rem;">Pan/zoom to load new tiles.</p>
      `;
      container.appendChild(info);

      return () => {
        map.off("sourcedata", sourcedata);
        map.off("styledata", styledata);
        eventLog.remove();
        info.remove();
      };
    },
  },

  {
    id: "query-features",
    title: "Query Features",
    description: "Query rendered features at a point",
    category: "events",
    code: `map.on('click', (e) => {
  const features = map.queryRenderedFeatures(e.point);
  console.log('Features at click:', features);
});

// Query specific layers
const features = map.queryRenderedFeatures(e.point, {
  layers: ['my-layer']
});`,
    run: (map, container) => {
      const eventLog = document.createElement("div");
      eventLog.id = "event-log";
      eventLog.innerHTML = `<h4>Queried Features</h4>`;
      container.appendChild(eventLog);

      const logClick = (e: maplibregl.MapMouseEvent) => {
        const features = map.queryRenderedFeatures(e.point);
        const entry = document.createElement("div");
        entry.className = "event";

        if (features.length === 0) {
          entry.innerHTML = `
            <span class="event-type">query</span>
            No features found
            <span class="event-time">${new Date().toLocaleTimeString()}</span>
          `;
        } else {
          const layerTypes = [...new Set(features.map((f) => f.layer?.type || "unknown"))];
          entry.innerHTML = `
            <span class="event-type">query</span>
            ${features.length} features (${layerTypes.join(", ")})
            <span class="event-time">${new Date().toLocaleTimeString()}</span>
          `;
        }

        eventLog.insertBefore(entry, eventLog.children[1]);
        while (eventLog.children.length > 11) {
          eventLog.removeChild(eventLog.lastChild!);
        }
      };

      map.on("click", logClick);

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Query Features</h4>
        <p style="color: #888; font-size: 0.75rem;">Click anywhere to query features at that point.</p>
        <div class="stat"><span class="label">Method</span><span class="value">queryRenderedFeatures</span></div>
      `;
      container.appendChild(info);

      return () => {
        map.off("click", logClick);
        eventLog.remove();
        info.remove();
      };
    },
  },
];
