/**
 * Port of upstream e2e/maplibre-gl-js/test/integration/lib/localize-urls.ts
 * Transforms local:// and mapbox:// URLs to point at our local fixture server.
 */

import { readFileSync } from "fs";
import { join } from "path";

export function localizeURLs(
  style: any,
  port: number,
  assetsDir: string,
): void {
  localizeStyleURLs(style, port);

  if (style.metadata?.test?.operations) {
    for (const op of style.metadata.test.operations) {
      if (op[0] === "addSource") {
        localizeSourceURLs(op[2], port);
      } else if (op[0] === "setStyle") {
        if (typeof op[1] === "object") {
          localizeStyleURLs(op[1], port);
          continue;
        }

        // op[1] is a "local://..." string path — read, parse, localize, inline
        let styleJSON: any;
        try {
          const relativePath = op[1].replace(/^local:\/\//, "");
          const raw = readFileSync(join(assetsDir, relativePath), "utf8");
          styleJSON = JSON.parse(raw);
        } catch (error) {
          console.log(`* ${error}`);
          continue;
        }

        localizeStyleURLs(styleJSON, port);
        op[1] = styleJSON;
        op[2] = { diff: false };
      }
    }
  }
}

function localizeURL(url: string, port: number): string {
  return url.replace(/^local:\/\//, `http://localhost:${port}/assets/`);
}

function localizeMapboxSpriteURL(url: string, port: number): string {
  return url.replace(/^mapbox:\/\//, `http://localhost:${port}/assets/`);
}

function localizeMapboxFontsURL(url: string, port: number): string {
  return url.replace(
    /^mapbox:\/\/fonts/,
    `http://localhost:${port}/assets/glyphs`,
  );
}

function localizeMapboxTilesURL(url: string, port: number): string {
  return url.replace(/^mapbox:\/\//, `http://localhost:${port}/assets/tiles/`);
}

function localizeMapboxTilesetURL(url: string, port: number): string {
  return url.replace(
    /^mapbox:\/\//,
    `http://localhost:${port}/assets/tilesets/`,
  );
}

function localizeSourceURLs(source: any, port: number): void {
  if (source.tiles) {
    for (let i = 0; i < source.tiles.length; i++) {
      source.tiles[i] = localizeMapboxTilesURL(source.tiles[i], port);
      source.tiles[i] = localizeURL(source.tiles[i], port);
    }
  }

  if (source.urls) {
    source.urls = source.urls.map((url: string) =>
      localizeMapboxTilesetURL(url, port),
    );
    source.urls = source.urls.map((url: string) => localizeURL(url, port));
  }

  if (source.url) {
    source.url = localizeMapboxTilesetURL(source.url, port);
    source.url = localizeURL(source.url, port);
  }

  if (source.data && typeof source.data === "string") {
    source.data = localizeURL(source.data, port);
  }
}

function localizeStyleURLs(style: any, port: number): void {
  if (style.sources) {
    for (const key in style.sources) {
      localizeSourceURLs(style.sources[key], port);
    }
  }

  if (style.sprite) {
    if (typeof style.sprite === "string") {
      style.sprite = localizeMapboxSpriteURL(style.sprite, port);
      style.sprite = localizeURL(style.sprite, port);
    } else if (Array.isArray(style.sprite)) {
      for (const sprite of style.sprite) {
        sprite.url = localizeMapboxSpriteURL(sprite.url, port);
        sprite.url = localizeURL(sprite.url, port);
      }
    }
  }

  if (style.glyphs) {
    style.glyphs = localizeMapboxFontsURL(style.glyphs, port);
    style.glyphs = localizeURL(style.glyphs, port);
  }
}
