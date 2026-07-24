(() => {
  "use strict";

  const CACHE_PREFIX = "dioxus-maplibre-mobile";
  const CACHE_VERSION = "v1";
  const PACK_KEY = "dioxus-maplibre-offline-packs";
  const MODE_KEY = "dioxus-maplibre-mode";

  const sharedAssets = [
    "/",
    "/manifest.webmanifest",
    "/maplibre-loader.js",
    "/offline-runtime.js",
    "/vendor/maplibre/maplibre-gl.css",
    "/vendor/maplibre/maplibre-gl.mjs",
    "/vendor/maplibre/maplibre-gl-shared.mjs",
    "/vendor/maplibre/maplibre-gl-worker.mjs",
  ];

  const packs = {
    helsinki: [
      "/offline/helsinki-style.json",
      "/offline/helsinki-base.geojson",
      "/offline/mobile-pin.svg",
      "/offline/mobile-pin.png",
    ],
    matterhorn: [
      "/offline/matterhorn-style.json",
      "/offline/matterhorn-base.geojson",
      "/offline/terrain-tile.png",
      "/offline/mobile-pin.svg",
      "/offline/mobile-pin.png",
    ],
    archipelago: [
      "/offline/archipelago-style.json",
      "/offline/archipelago-base.geojson",
      "/offline/mobile-pin.svg",
      "/offline/mobile-pin.png",
    ],
    tokyo: [
      "/offline/tokyo-style.json",
      "/offline/tokyo-base.geojson",
    ],
  };

  let controller = null;

  function runtimeAssetUrls() {
    const urls = new Set();
    for (const entry of performance.getEntriesByType("resource")) {
      try {
        const url = new URL(entry.name, location.href);
        if (url.origin === location.origin && url.pathname.startsWith("/assets/")) {
          urls.add(url.pathname);
        }
      } catch {
        // Ignore non-URL performance entries.
      }
    }
    for (const element of document.querySelectorAll("script[src], link[href]")) {
      const candidate = element.src || element.href;
      if (!candidate) continue;
      const url = new URL(candidate, location.href);
      if (url.origin === location.origin) urls.add(url.pathname);
    }
    return [...urls];
  }

  async function warmAppShell() {
    const cache = await caches.open(`${CACHE_PREFIX}-shell-${CACHE_VERSION}`);
    const urls = [...new Set([...sharedAssets, ...runtimeAssetUrls()])];
    await Promise.allSettled(urls.map((url) => cacheOne(cache, url)));
  }

  function savedPacks() {
    try {
      const value = JSON.parse(localStorage.getItem(PACK_KEY) || "[]");
      return Array.isArray(value) ? value : [];
    } catch {
      return [];
    }
  }

  function storePack(pack) {
    const values = new Set(savedPacks());
    values.add(pack);
    localStorage.setItem(PACK_KEY, JSON.stringify([...values]));
  }

  function send(callback, event) {
    try {
      callback(event);
    } catch (error) {
      console.error("[showcase-mobile] Offline callback failed", error);
    }
  }

  async function pausePoint(callback) {
    while (controller?.paused && !controller.cancelled) {
      await new Promise((resolve) => setTimeout(resolve, 80));
    }
    if (controller?.cancelled) {
      send(callback, { kind: "cancelled" });
      throw new DOMException("Offline download cancelled", "AbortError");
    }
  }

  async function cacheOne(cache, url) {
    const response = await fetch(url, { cache: "reload" });
    if (!response.ok) {
      throw new Error(`${url} returned ${response.status}`);
    }
    await cache.put(url, response.clone());
  }

  async function downloadPack(pack, callback = () => {}) {
    if (!("serviceWorker" in navigator) || !("caches" in globalThis) || !isSecureContext) {
      send(callback, {
        kind: "error",
        message: "Offline packs require HTTPS or localhost so the service worker can run",
      });
      return;
    }
    if (!packs[pack]) {
      send(callback, { kind: "error", message: `Unknown offline pack: ${pack}` });
      return;
    }

    controller = { paused: false, cancelled: false };
    const urls = [
      ...sharedAssets,
      ...runtimeAssetUrls(),
      ...packs.helsinki,
      ...packs.tokyo,
      ...packs[pack],
    ];
    const uniqueUrls = [...new Set(urls)];

    try {
      const cache = await caches.open(`${CACHE_PREFIX}-${CACHE_VERSION}-${pack}`);
      for (let index = 0; index < uniqueUrls.length; index += 1) {
        await pausePoint(callback);
        const url = uniqueUrls[index];
        const component = url.includes("maplibre")
          ? "MapLibre engine"
          : url.includes("style")
            ? "Map style"
            : url.includes("terrain")
              ? "Elevation"
              : url.endsWith(".geojson")
                ? "Local data"
                : "App shell";
        await cacheOne(cache, url);
        const progress = Math.round(((index + 1) / uniqueUrls.length) * 100);
        send(callback, { kind: "progress", progress, component });
        // Keeps the progress state visible and makes pause/resume demonstrable.
        await new Promise((resolve) => setTimeout(resolve, 90));
      }

      storePack(pack);
      send(callback, { kind: "complete", progress: 100, component: "Ready" });
    } catch (error) {
      if (error?.name !== "AbortError") {
        send(callback, { kind: "error", message: error?.message || String(error) });
      }
    } finally {
      controller = null;
    }
  }

  const api = {
    snapshot() {
      return {
        mode: localStorage.getItem(MODE_KEY),
        packs: ["helsinki", ...savedPacks()],
        supported: Boolean(
          "serviceWorker" in navigator
          && "caches" in globalThis
          && isSecureContext
        ),
      };
    },

    setMode(mode) {
      localStorage.setItem(MODE_KEY, mode === "offline" ? "offline" : "online");
    },

    downloadPack,

    pauseDownload() {
      if (controller && !controller.paused) {
        controller.paused = true;
      }
    },

    resumeDownload() {
      if (controller?.paused) {
        controller.paused = false;
      }
    },

    cancelDownload() {
      if (controller) {
        controller.cancelled = true;
        controller.paused = false;
      }
    },
  };

  globalThis.dioxusMaplibreOffline = api;

  if ("serviceWorker" in navigator && location.protocol !== "file:") {
    const registerServiceWorker = () => {
      navigator.serviceWorker.register("/service-worker.js")
        .then(() => navigator.serviceWorker.ready)
        .then(() => warmAppShell())
        .catch((error) => {
          console.warn("[showcase-mobile] Service worker registration failed", error);
        });
    };
    if (document.readyState === "complete") {
      registerServiceWorker();
    } else {
      addEventListener("load", registerServiceWorker, { once: true });
    }
  }
})();
