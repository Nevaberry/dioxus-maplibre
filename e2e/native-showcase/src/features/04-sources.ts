import { type Demo } from "../types.ts";

declare const maplibregl: typeof import("maplibre-gl");

export const sourceDemos: Demo[] = [
  {
    id: "geojson-source",
    title: "GeoJSON Source",
    description: "Load GeoJSON data inline or from URL",
    category: "sources",
    code: `map.addSource('geojson', {
  type: 'geojson',
  data: {
    type: 'FeatureCollection',
    features: [...]
  }
});

// Or from URL
map.addSource('geojson', {
  type: 'geojson',
  data: 'https://example.com/data.geojson'
});`,
    run: (map, container) => {
      const geojsonData = {
        type: "FeatureCollection" as const,
        features: [
          {
            type: "Feature" as const,
            geometry: { type: "Point" as const, coordinates: [-74.006, 40.7128] },
            properties: { name: "Downtown", type: "commercial" },
          },
          {
            type: "Feature" as const,
            geometry: { type: "Point" as const, coordinates: [-73.965, 40.782] },
            properties: { name: "Central Park", type: "park" },
          },
          {
            type: "Feature" as const,
            geometry: {
              type: "LineString" as const,
              coordinates: [[-74.006, 40.7128], [-73.965, 40.782]],
            },
            properties: { name: "Connection" },
          },
        ],
      };

      map.addSource("demo-geojson", {
        type: "geojson",
        data: geojsonData,
      });

      map.addLayer({
        id: "demo-geojson-points",
        type: "circle",
        source: "demo-geojson",
        filter: ["==", "$type", "Point"],
        paint: {
          "circle-radius": 10,
          "circle-color": [
            "match",
            ["get", "type"],
            "commercial", "#e74c3c",
            "park", "#2ecc71",
            "#888",
          ],
        },
      });

      map.addLayer({
        id: "demo-geojson-line",
        type: "line",
        source: "demo-geojson",
        filter: ["==", "$type", "LineString"],
        paint: {
          "line-color": "#3498db",
          "line-width": 3,
          "line-dasharray": [4, 2],
        },
      });

      map.fitBounds([[-74.01, 40.71], [-73.96, 40.79]], { padding: 50 });

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>GeoJSON Source</h4>
        <div class="stat"><span class="label">Features</span><span class="value">3</span></div>
        <div class="stat"><span class="label">Points</span><span class="value">2</span></div>
        <div class="stat"><span class="label">Lines</span><span class="value">1</span></div>
      `;
      container.appendChild(info);

      return () => {
        if (map.getLayer("demo-geojson-line")) map.removeLayer("demo-geojson-line");
        if (map.getLayer("demo-geojson-points")) map.removeLayer("demo-geojson-points");
        if (map.getSource("demo-geojson")) map.removeSource("demo-geojson");
        info.remove();
      };
    },
  },

  {
    id: "geojson-update",
    title: "GeoJSON Update",
    description: "Update GeoJSON data dynamically with setData()",
    category: "sources",
    code: `const source = map.getSource('geojson');
source.setData({
  type: 'FeatureCollection',
  features: newFeatures
});`,
    run: (map, container) => {
      let pointCount = 10;

      const generatePoints = (count: number) => ({
        type: "FeatureCollection" as const,
        features: Array.from({ length: count }, () => ({
          type: "Feature" as const,
          geometry: {
            type: "Point" as const,
            coordinates: [
              -74.006 + (Math.random() - 0.5) * 0.1,
              40.7128 + (Math.random() - 0.5) * 0.1,
            ],
          },
          properties: {},
        })),
      });

      map.addSource("demo-dynamic-geojson", {
        type: "geojson",
        data: generatePoints(pointCount),
      });

      map.addLayer({
        id: "demo-dynamic-points",
        type: "circle",
        source: "demo-dynamic-geojson",
        paint: {
          "circle-radius": 8,
          "circle-color": "#e94560",
          "circle-opacity": 0.8,
        },
      });

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Dynamic GeoJSON</h4>
        <div class="stat"><span class="label">Points</span><span class="value" id="point-count">${pointCount}</span></div>
        <div style="display: flex; flex-direction: column; gap: 0.25rem; margin-top: 0.5rem;">
          <button id="add-points" style="padding: 0.4rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px; cursor: pointer;">Add 10 Points</button>
          <button id="remove-points" style="padding: 0.4rem; background: #0f3460; border: 1px solid #16213e; color: #ccc; border-radius: 4px; cursor: pointer;">Remove 5 Points</button>
          <button id="regenerate" style="padding: 0.4rem; background: #e94560; border: none; color: white; border-radius: 4px; cursor: pointer;">Regenerate All</button>
        </div>
      `;
      container.appendChild(info);

      const updateCount = () => {
        document.getElementById("point-count")!.textContent = pointCount.toString();
      };

      document.getElementById("add-points")!.addEventListener("click", () => {
        pointCount += 10;
        (map.getSource("demo-dynamic-geojson") as maplibregl.GeoJSONSource).setData(generatePoints(pointCount));
        updateCount();
      });

      document.getElementById("remove-points")!.addEventListener("click", () => {
        pointCount = Math.max(0, pointCount - 5);
        (map.getSource("demo-dynamic-geojson") as maplibregl.GeoJSONSource).setData(generatePoints(pointCount));
        updateCount();
      });

      document.getElementById("regenerate")!.addEventListener("click", () => {
        (map.getSource("demo-dynamic-geojson") as maplibregl.GeoJSONSource).setData(generatePoints(pointCount));
      });

      return () => {
        if (map.getLayer("demo-dynamic-points")) map.removeLayer("demo-dynamic-points");
        if (map.getSource("demo-dynamic-geojson")) map.removeSource("demo-dynamic-geojson");
        info.remove();
      };
    },
  },

  {
    id: "clustering",
    title: "GeoJSON Clustering",
    description: "Cluster nearby points automatically",
    category: "sources",
    code: `map.addSource('clustered', {
  type: 'geojson',
  data: pointsData,
  cluster: true,
  clusterRadius: 50,
  clusterMaxZoom: 14,
  clusterProperties: {
    sum: ['+', ['get', 'value']]
  }
});`,
    run: (map, container) => {
      // Generate many points for clustering
      const points = {
        type: "FeatureCollection" as const,
        features: Array.from({ length: 500 }, () => ({
          type: "Feature" as const,
          geometry: {
            type: "Point" as const,
            coordinates: [
              -74.006 + (Math.random() - 0.5) * 0.2,
              40.7128 + (Math.random() - 0.5) * 0.2,
            ],
          },
          properties: {
            value: Math.floor(Math.random() * 100),
          },
        })),
      };

      map.addSource("demo-clustered", {
        type: "geojson",
        data: points,
        cluster: true,
        clusterRadius: 50,
        clusterMaxZoom: 14,
      });

      // Cluster circles
      map.addLayer({
        id: "demo-clusters",
        type: "circle",
        source: "demo-clustered",
        filter: ["has", "point_count"],
        paint: {
          "circle-color": [
            "step",
            ["get", "point_count"],
            "#51bbd6", 10,
            "#f1f075", 50,
            "#f28cb1", 100,
            "#e74c3c",
          ],
          "circle-radius": [
            "step",
            ["get", "point_count"],
            15, 10,
            20, 50,
            25, 100,
            30,
          ],
        },
      });

      // Cluster count labels
      map.addLayer({
        id: "demo-cluster-count",
        type: "symbol",
        source: "demo-clustered",
        filter: ["has", "point_count"],
        layout: {
          "text-field": "{point_count_abbreviated}",
          "text-size": 12,
        },
        paint: {
          "text-color": "#fff",
        },
      });

      // Unclustered points
      map.addLayer({
        id: "demo-unclustered",
        type: "circle",
        source: "demo-clustered",
        filter: ["!", ["has", "point_count"]],
        paint: {
          "circle-color": "#11b4da",
          "circle-radius": 6,
          "circle-stroke-width": 1,
          "circle-stroke-color": "#fff",
        },
      });

      map.setZoom(10);

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Clustering</h4>
        <div class="stat"><span class="label">Total Points</span><span class="value">500</span></div>
        <div class="stat"><span class="label">Cluster Radius</span><span class="value">50px</span></div>
        <div class="stat"><span class="label">Max Zoom</span><span class="value">14</span></div>
        <p style="margin-top: 0.5rem; color: #888; font-size: 0.75rem;">Zoom in to see clusters expand!</p>
      `;
      container.appendChild(info);

      // Click on cluster to zoom
      map.on("click", "demo-clusters", async (e) => {
        const features = map.queryRenderedFeatures(e.point, { layers: ["demo-clusters"] });
        if (features.length === 0) return;

        const clusterId = features[0].properties?.cluster_id;
        if (clusterId === undefined) return;

        const source = map.getSource("demo-clustered") as maplibregl.GeoJSONSource;
        const zoom = await source.getClusterExpansionZoom(clusterId);
        const geometry = features[0].geometry as GeoJSON.Point;

        map.easeTo({
          center: geometry.coordinates as [number, number],
          zoom,
        });
      });

      return () => {
        if (map.getLayer("demo-unclustered")) map.removeLayer("demo-unclustered");
        if (map.getLayer("demo-cluster-count")) map.removeLayer("demo-cluster-count");
        if (map.getLayer("demo-clusters")) map.removeLayer("demo-clusters");
        if (map.getSource("demo-clustered")) map.removeSource("demo-clustered");
        info.remove();
      };
    },
  },

  {
    id: "image-source",
    title: "Image Source",
    description: "Overlay an image on the map",
    category: "sources",
    code: `map.addSource('overlay', {
  type: 'image',
  url: 'https://example.com/image.png',
  coordinates: [
    [-74.1, 40.8],   // top-left
    [-73.9, 40.8],   // top-right
    [-73.9, 40.6],   // bottom-right
    [-74.1, 40.6]    // bottom-left
  ]
});`,
    run: (map, container) => {
      // Create a simple colored rectangle as a data URL
      const canvas = document.createElement("canvas");
      canvas.width = 200;
      canvas.height = 200;
      const ctx = canvas.getContext("2d")!;

      // Create a gradient
      const gradient = ctx.createLinearGradient(0, 0, 200, 200);
      gradient.addColorStop(0, "rgba(233, 69, 96, 0.7)");
      gradient.addColorStop(1, "rgba(15, 52, 96, 0.7)");
      ctx.fillStyle = gradient;
      ctx.fillRect(0, 0, 200, 200);

      // Add text
      ctx.fillStyle = "white";
      ctx.font = "bold 24px sans-serif";
      ctx.textAlign = "center";
      ctx.fillText("Image Overlay", 100, 105);

      const imageUrl = canvas.toDataURL();

      map.addSource("demo-image", {
        type: "image",
        url: imageUrl,
        coordinates: [
          [-74.05, 40.75],
          [-73.95, 40.75],
          [-73.95, 40.68],
          [-74.05, 40.68],
        ],
      });

      map.addLayer({
        id: "demo-image-layer",
        type: "raster",
        source: "demo-image",
        paint: {
          "raster-opacity": 0.85,
        },
      });

      map.fitBounds([[-74.06, 40.67], [-73.94, 40.76]], { padding: 30 });

      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Image Source</h4>
        <div class="stat"><span class="label">Type</span><span class="value">Canvas Data URL</span></div>
        <div class="stat"><span class="label">Size</span><span class="value">200x200px</span></div>
        <p style="margin-top: 0.5rem; color: #888; font-size: 0.75rem;">Image is georeferenced to specific coordinates.</p>
      `;
      container.appendChild(info);

      return () => {
        if (map.getLayer("demo-image-layer")) map.removeLayer("demo-image-layer");
        if (map.getSource("demo-image")) map.removeSource("demo-image");
        info.remove();
      };
    },
  },

  {
    id: "vector-source",
    title: "Vector Tiles",
    description: "Load vector tile source",
    category: "sources",
    code: `map.addSource('vector', {
  type: 'vector',
  tiles: ['https://example.com/{z}/{x}/{y}.pbf'],
  minzoom: 0,
  maxzoom: 14
});`,
    run: (map, container) => {
      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Vector Tiles</h4>
        <p style="color: #888; font-size: 0.75rem; margin-bottom: 0.5rem;">
          Vector tiles are already loaded from the base style. This demo shows the concept.
        </p>
        <div class="stat"><span class="label">Format</span><span class="value">MVT (Mapbox Vector Tiles)</span></div>
        <div class="stat"><span class="label">New Format</span><span class="value">MLT (MapLibre Tiles)</span></div>
        <code style="display: block; background: #0d1117; padding: 0.5rem; border-radius: 3px; font-size: 0.7rem; margin-top: 0.5rem; white-space: pre-wrap;">tiles: [
  'https://example.com/{z}/{x}/{y}.pbf'
]</code>
      `;
      container.appendChild(info);

      return () => info.remove();
    },
  },

  {
    id: "raster-dem",
    title: "Raster DEM Source",
    description: "Digital Elevation Model for terrain",
    category: "sources",
    code: `map.addSource('dem', {
  type: 'raster-dem',
  url: 'https://demotiles.maplibre.org/terrain-tiles/tiles.json',
  tileSize: 256
});

map.setTerrain({ source: 'dem', exaggeration: 1.5 });`,
    run: (map, container) => {
      const info = document.createElement("div");
      info.className = "info-panel";
      info.innerHTML = `
        <h4>Raster DEM Source</h4>
        <p style="color: #888; font-size: 0.75rem; margin-bottom: 0.5rem;">
          DEM sources provide elevation data for 3D terrain rendering.
        </p>
        <div class="stat"><span class="label">Encodings</span><span class="value">Mapbox, Terrarium, Custom</span></div>
        <div class="stat"><span class="label">Tile Size</span><span class="value">256px or 512px</span></div>
        <code style="display: block; background: #0d1117; padding: 0.5rem; border-radius: 3px; font-size: 0.7rem; margin-top: 0.5rem; white-space: pre-wrap;">encoding: 'mapbox' // or 'terrarium'
// Custom:
redFactor: 1,
greenFactor: 1,
blueFactor: 1,
baseShift: 0</code>
        <p style="margin-top: 0.5rem; color: #888; font-size: 0.75rem;">See 3D Features section for terrain demo.</p>
      `;
      container.appendChild(info);

      return () => info.remove();
    },
  },
];
