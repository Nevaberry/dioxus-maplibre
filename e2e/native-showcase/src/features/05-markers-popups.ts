import { type Demo } from "../types.ts";

declare const maplibregl: typeof import("maplibre-gl");

export const markerDemos: Demo[] = [
  {
    id: "basic-marker",
    title: "Basic Marker",
    description: "Add a simple marker to the map",
    category: "markers",
    code: `const marker = new maplibregl.Marker()
  .setLngLat([-74.006, 40.7128])
  .addTo(map);`,
    run: (map, container) => {
      const marker = new maplibregl.Marker()
        .setLngLat([-74.006, 40.7128])
        .addTo(map);

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Basic Marker</h4>
        <div class="stat"><span class="label">Position</span><span class="value">-74.006, 40.713</span></div>
        <div class="stat"><span class="label">Color</span><span class="value">#3FB1CE (default)</span></div>
      `;
      container.appendChild(info);

      return () => {
        marker.remove();
        info.remove();
      };
    },
  },

  {
    id: "colored-markers",
    title: "Colored Markers",
    description: "Customize marker colors",
    category: "markers",
    code: `new maplibregl.Marker({ color: '#e74c3c' })
  .setLngLat([-74.006, 40.7128])
  .addTo(map);`,
    run: (map, container) => {
      const colors = ["#e74c3c", "#3498db", "#2ecc71", "#f39c12", "#9b59b6"];
      const markers: maplibregl.Marker[] = [];

      colors.forEach((color, i) => {
        const offset = (i - 2) * 0.015;
        const marker = new maplibregl.Marker({ color })
          .setLngLat([-74.006 + offset, 40.7128])
          .addTo(map);
        markers.push(marker);
      });

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Colored Markers</h4>
        <div style="display: flex; gap: 0.25rem; margin-top: 0.5rem;">
          ${colors.map((c) => `<div style="width: 20px; height: 20px; background: ${c}; border-radius: 50%;"></div>`).join("")}
        </div>
      `;
      container.appendChild(info);

      return () => {
        markers.forEach((m) => m.remove());
        info.remove();
      };
    },
  },

  {
    id: "custom-marker",
    title: "Custom HTML Marker",
    description: "Use custom HTML elements as markers",
    category: "markers",
    code: `const el = document.createElement('div');
el.className = 'custom-marker';
el.innerHTML = '<span>Hello!</span>';

new maplibregl.Marker({ element: el })
  .setLngLat([-74.006, 40.7128])
  .addTo(map);`,
    run: (map, container) => {
      const el = document.createElement("div");
      el.innerHTML = `
        <div style="
          background: linear-gradient(135deg, #e94560, #0f3460);
          color: white;
          padding: 8px 16px;
          border-radius: 20px;
          font-weight: bold;
          font-size: 14px;
          box-shadow: 0 4px 12px rgba(0,0,0,0.3);
          cursor: pointer;
          transform: translate(-50%, -100%);
          white-space: nowrap;
        ">
          Custom HTML Marker
        </div>
      `;

      const marker = new maplibregl.Marker({ element: el, anchor: "bottom" })
        .setLngLat([-74.006, 40.7128])
        .addTo(map);

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Custom HTML Marker</h4>
        <p style="color: #888; font-size: 0.75rem;">
          Any HTML element can be used as a marker. This allows for rich, interactive markers.
        </p>
      `;
      container.appendChild(info);

      return () => {
        marker.remove();
        info.remove();
      };
    },
  },

  {
    id: "draggable-marker",
    title: "Draggable Marker",
    description: "Enable marker dragging with events",
    category: "markers",
    code: `const marker = new maplibregl.Marker({ draggable: true })
  .setLngLat([-74.006, 40.7128])
  .addTo(map);

marker.on('dragend', () => {
  const lngLat = marker.getLngLat();
  console.log(lngLat);
});`,
    run: (map, container) => {
      const marker = new maplibregl.Marker({ draggable: true, color: "#e94560" })
        .setLngLat([-74.006, 40.7128])
        .addTo(map);

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Draggable Marker</h4>
        <div class="stat"><span class="label">Lng</span><span class="value" id="drag-lng">-74.0060</span></div>
        <div class="stat"><span class="label">Lat</span><span class="value" id="drag-lat">40.7128</span></div>
        <p style="margin-top: 0.5rem; color: #888; font-size: 0.75rem;">Drag the marker and watch coordinates update!</p>
      `;
      container.appendChild(info);

      const updatePosition = () => {
        const pos = marker.getLngLat();
        document.getElementById("drag-lng")!.textContent = pos.lng.toFixed(4);
        document.getElementById("drag-lat")!.textContent = pos.lat.toFixed(4);
      };

      marker.on("drag", updatePosition);
      marker.on("dragend", updatePosition);

      return () => {
        marker.remove();
        info.remove();
      };
    },
  },

  {
    id: "marker-rotation",
    title: "Rotated Marker",
    description: "Rotate markers with alignment options",
    category: "markers",
    code: `new maplibregl.Marker({
  rotation: 45,
  rotationAlignment: 'map', // or 'viewport'
  pitchAlignment: 'map'
})
  .setLngLat([-74.006, 40.7128])
  .addTo(map);`,
    run: (map, container) => {
      let rotation = 0;
      const marker = new maplibregl.Marker({
        rotation: 0,
        rotationAlignment: "map",
        color: "#e94560",
      })
        .setLngLat([-74.006, 40.7128])
        .addTo(map);

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Marker Rotation</h4>
        <div class="stat"><span class="label">Rotation</span><span class="value" id="marker-rot">0°</span></div>
        <div style="margin-top: 0.5rem;">
          <input type="range" id="rot-slider" min="0" max="360" value="0" style="width: 100%;">
        </div>
        <div style="margin-top: 0.5rem;">
          <label style="color: #888; font-size: 0.75rem;">Alignment:</label>
          <select id="rot-align" style="width: 100%; padding: 0.25rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px;">
            <option value="map">map</option>
            <option value="viewport">viewport</option>
            <option value="auto">auto</option>
          </select>
        </div>
      `;
      container.appendChild(info);

      document.getElementById("rot-slider")!.addEventListener("input", (e) => {
        rotation = parseInt((e.target as HTMLInputElement).value);
        marker.setRotation(rotation);
        document.getElementById("marker-rot")!.textContent = `${rotation}°`;
      });

      document.getElementById("rot-align")!.addEventListener("change", (e) => {
        const align = (e.target as HTMLSelectElement).value as "map" | "viewport" | "auto";
        marker.setRotationAlignment(align);
      });

      // Pitch the map to show alignment difference
      map.easeTo({ pitch: 45, duration: 1000 });

      return () => {
        marker.remove();
        info.remove();
      };
    },
  },

  {
    id: "basic-popup",
    title: "Basic Popup",
    description: "Add a popup to the map",
    category: "markers",
    code: `new maplibregl.Popup()
  .setLngLat([-74.006, 40.7128])
  .setHTML('<h3>Hello World!</h3><p>This is a popup.</p>')
  .addTo(map);`,
    run: (map, container) => {
      const popup = new maplibregl.Popup({ closeOnClick: false })
        .setLngLat([-74.006, 40.7128])
        .setHTML(`
          <div style="padding: 0.5rem;">
            <h3 style="margin: 0 0 0.5rem; color: #1a1a2e;">Hello World!</h3>
            <p style="margin: 0; color: #666;">This is a MapLibre popup.</p>
          </div>
        `)
        .addTo(map);

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Basic Popup</h4>
        <div class="stat"><span class="label">Close Button</span><span class="value">Yes</span></div>
        <div class="stat"><span class="label">Close on Click</span><span class="value">No</span></div>
      `;
      container.appendChild(info);

      return () => {
        popup.remove();
        info.remove();
      };
    },
  },

  {
    id: "marker-with-popup",
    title: "Marker with Popup",
    description: "Attach a popup to a marker",
    category: "markers",
    code: `const popup = new maplibregl.Popup({ offset: 25 })
  .setHTML('<h3>Marker Info</h3>');

const marker = new maplibregl.Marker()
  .setLngLat([-74.006, 40.7128])
  .setPopup(popup)
  .addTo(map);`,
    run: (map, container) => {
      const popup = new maplibregl.Popup({ offset: 25 }).setHTML(`
        <div style="padding: 0.5rem;">
          <h3 style="margin: 0 0 0.5rem; color: #1a1a2e;">New York City</h3>
          <p style="margin: 0; color: #666;">Click the marker to toggle this popup!</p>
        </div>
      `);

      const marker = new maplibregl.Marker({ color: "#e94560" })
        .setLngLat([-74.006, 40.7128])
        .setPopup(popup)
        .addTo(map);

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Marker with Popup</h4>
        <p style="color: #888; font-size: 0.75rem;">Click the marker to show/hide the popup.</p>
        <button id="toggle-popup" style="margin-top: 0.5rem; padding: 0.4rem 0.75rem; background: #e94560; border: none; color: white; border-radius: 4px; cursor: pointer; width: 100%;">
          Toggle Popup
        </button>
      `;
      container.appendChild(info);

      document.getElementById("toggle-popup")!.addEventListener("click", () => {
        marker.togglePopup();
      });

      return () => {
        marker.remove();
        info.remove();
      };
    },
  },

  {
    id: "popup-options",
    title: "Popup Options",
    description: "Customize popup behavior and appearance",
    category: "markers",
    code: `new maplibregl.Popup({
  closeButton: false,
  closeOnClick: true,
  closeOnMove: true,
  anchor: 'bottom',
  offset: [0, -10],
  maxWidth: '300px',
  className: 'my-popup'
})`,
    run: (map, container) => {
      const anchors: maplibregl.PositionAnchor[] = ["top", "bottom", "left", "right", "center"];
      let currentAnchor = 0;

      const popup = new maplibregl.Popup({
        closeButton: true,
        closeOnClick: false,
        anchor: anchors[currentAnchor],
        maxWidth: "200px",
      })
        .setLngLat([-74.006, 40.7128])
        .setHTML(`<p style="padding: 0.5rem; margin: 0;">Anchor: <strong>${anchors[currentAnchor]}</strong></p>`)
        .addTo(map);

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Popup Options</h4>
        <div class="stat"><span class="label">Anchor</span><span class="value" id="popup-anchor">${anchors[currentAnchor]}</span></div>
        <button id="change-anchor" style="margin-top: 0.5rem; padding: 0.4rem 0.75rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px; cursor: pointer; width: 100%;">
          Change Anchor
        </button>
      `;
      container.appendChild(info);

      document.getElementById("change-anchor")!.addEventListener("click", () => {
        currentAnchor = (currentAnchor + 1) % anchors.length;
        popup.remove();

        const newPopup = new maplibregl.Popup({
          closeButton: true,
          closeOnClick: false,
          anchor: anchors[currentAnchor],
          maxWidth: "200px",
        })
          .setLngLat([-74.006, 40.7128])
          .setHTML(`<p style="padding: 0.5rem; margin: 0;">Anchor: <strong>${anchors[currentAnchor]}</strong></p>`)
          .addTo(map);

        document.getElementById("popup-anchor")!.textContent = anchors[currentAnchor];
      });

      return () => {
        popup.remove();
        info.remove();
      };
    },
  },

  {
    id: "multiple-markers",
    title: "Multiple Markers",
    description: "Add many markers efficiently",
    category: "markers",
    code: `const locations = [
  { coords: [-74.006, 40.7128], name: 'NYC' },
  { coords: [-73.9857, 40.7484], name: 'Times Square' },
  // ...
];

locations.forEach(loc => {
  new maplibregl.Marker()
    .setLngLat(loc.coords)
    .addTo(map);
});`,
    run: (map, container) => {
      const locations = [
        { coords: [-74.006, 40.7128] as [number, number], name: "Downtown" },
        { coords: [-73.9857, 40.7484] as [number, number], name: "Times Square" },
        { coords: [-73.9654, 40.7829] as [number, number], name: "Central Park" },
        { coords: [-74.0445, 40.6892] as [number, number], name: "Statue of Liberty" },
        { coords: [-73.9857, 40.7580] as [number, number], name: "Rockefeller" },
        { coords: [-73.9712, 40.7614] as [number, number], name: "MoMA" },
        { coords: [-74.0014, 40.7061] as [number, number], name: "Brooklyn Bridge" },
        { coords: [-73.9632, 40.7794] as [number, number], name: "Met Museum" },
      ];

      const markers: maplibregl.Marker[] = [];

      locations.forEach((loc, i) => {
        const popup = new maplibregl.Popup({ offset: 25 }).setHTML(`
          <div style="padding: 0.25rem;">
            <strong>${loc.name}</strong>
          </div>
        `);

        const marker = new maplibregl.Marker({
          color: `hsl(${(i * 45) % 360}, 70%, 50%)`,
        })
          .setLngLat(loc.coords)
          .setPopup(popup)
          .addTo(map);

        markers.push(marker);
      });

      map.fitBounds([[-74.05, 40.68], [-73.96, 40.79]], { padding: 50 });

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Multiple Markers</h4>
        <div class="stat"><span class="label">Count</span><span class="value">${locations.length}</span></div>
        <p style="margin-top: 0.5rem; color: #888; font-size: 0.75rem;">Click any marker to see its popup.</p>
      `;
      container.appendChild(info);

      return () => {
        markers.forEach((m) => m.remove());
        info.remove();
      };
    },
  },
];
