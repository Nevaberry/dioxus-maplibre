const CACHE_PREFIX = "dioxus-maplibre-mobile";
const SHELL_CACHE = `${CACHE_PREFIX}-shell-v1`;
const INSTALL_ASSETS = [
  "/",
  "/manifest.webmanifest",
  "/maplibre-loader.js",
  "/offline-runtime.js",
  "/vendor/maplibre/maplibre-gl.css",
  "/vendor/maplibre/maplibre-gl.mjs",
  "/vendor/maplibre/maplibre-gl-shared.mjs",
  "/vendor/maplibre/maplibre-gl-worker.mjs",
  "/offline/helsinki-style.json",
  "/offline/helsinki-base.geojson",
  "/offline/tokyo-style.json",
  "/offline/tokyo-base.geojson",
  "/offline/mobile-pin.svg",
  "/offline/mobile-pin.png",
];

self.addEventListener("install", (event) => {
  event.waitUntil(
    caches.open(SHELL_CACHE)
      .then(async (cache) => {
        await Promise.allSettled(INSTALL_ASSETS.map(async (url) => {
          const response = await fetch(url, { cache: "reload" });
          if (response.ok) await cache.put(url, response);
        }));
      })
      .then(() => self.skipWaiting()),
  );
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches.keys()
      .then((keys) => Promise.all(
        keys
          .filter((key) => key.startsWith(CACHE_PREFIX) && !key.includes("v1"))
          .map((key) => caches.delete(key)),
      ))
      .then(() => self.clients.claim()),
  );
});

self.addEventListener("fetch", (event) => {
  const request = event.request;
  if (request.method !== "GET") return;

  const url = new URL(request.url);
  if (url.origin !== self.location.origin) return;

  event.respondWith((async () => {
    const cached = await caches.match(request, { ignoreSearch: true });
    if (cached) return cached;

    try {
      const response = await fetch(request);
      if (response.ok) {
        const cache = await caches.open(SHELL_CACHE);
        await cache.put(request, response.clone());
      }
      return response;
    } catch (error) {
      if (request.mode === "navigate") {
        const shell = await caches.match("/");
        if (shell) return shell;
      }
      throw error;
    }
  })());
});
