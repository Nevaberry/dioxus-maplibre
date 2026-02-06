/**
 * Generates the JavaScript for applyOperations() that runs inside the browser page.
 * Port of upstream run_render_tests.ts:603-687 applyOperations().
 *
 * The generated code assumes `map` and `options` (test metadata) are in scope.
 */

import { SERVER_PORT } from "./constants";

export function generateOperationsScript(): string {
  return `
async function applyOperations(options, map) {
  if (!options.operations || options.operations.length === 0) return;

  let idle = false;
  map.on('idle', () => { idle = true; });

  for (const operation of options.operations) {
    console.log('Running operation: ' + JSON.stringify(operation));

    switch (operation[0]) {
      case 'wait':
        if (operation.length <= 1) {
          // Wait until map is fully loaded
          while (!map.loaded()) {
            await map.once('render');
          }
        } else if (typeof operation[1] === 'string') {
          // Wait for a named event
          await map.once(operation[1]);
        } else {
          // Wait for a timeout in ms
          await new Promise(r => setTimeout(r, operation[1]));
          map._render();
        }
        break;

      case 'idle':
        map.repaint = false;
        if (!idle) {
          await map.once('idle');
        }
        break;

      case 'sleep':
        await new Promise(r => setTimeout(r, operation[1]));
        break;

      case 'addImage': {
        const img = new Image();
        img.src = 'http://localhost:${SERVER_PORT}/assets/' + operation[2];
        img.crossOrigin = 'anonymous';
        await img.decode();
        map.addImage(operation[1], img, operation[3] || {});
        break;
      }

      case 'addCustomLayer':
        // Custom layers require WebGL shader classes defined in-page
        if (window.__customLayerImplementations && window.__customLayerImplementations[operation[1]]) {
          map.addLayer(new window.__customLayerImplementations[operation[1]](), operation[2]);
          map._render();
        }
        break;

      case 'setStyle':
        map.setStyle(operation[1], { localIdeographFontFamily: false });
        break;

      default:
        if (typeof map[operation[0]] === 'function') {
          map[operation[0]](...operation.slice(1));
        }
        break;
    }
  }
}
`;
}
