
document.getElementById('canvas').setAttribute('width', document.documentElement.clientWidth);
document.getElementById('canvas').setAttribute('height', document.documentElement.clientHeight);
import('./pkg/rust_wasm_webgl.js')
  .then(({ default: init }) => init())
  .catch(console.error);
