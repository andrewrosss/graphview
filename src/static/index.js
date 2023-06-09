/** @type {HTMLTextAreaElement | null} */
const textarea = document.getElementById('dot');
const textareaValue = textarea?.value;

// sync textarea value to URL hash
textarea?.addEventListener('input', (event) => {
  const dot = event.target.value;
  const compressed = LZString.compressToEncodedURIComponent(dot);
  const url = new URL(location);
  url.hash = compressed;
  history.replaceState(null, '', url);
});

// sync URL hash and textarea value on page load if one is set and the other is not
window.addEventListener("DOMContentLoaded", () => {
  const url = new URL(location);
  const compressed = url.hash.slice(1);
  if (!compressed && !textareaValue) {
    return;
  }
  // url hash is set but textarea is not
  if (compressed && !textareaValue) {
    const dot = LZString.decompressFromEncodedURIComponent(compressed);
    textarea.value = dot;
    return;
  }
  // textarea is set but url hash is not
  if (!compressed && textareaValue) {
    const dot = textareaValue;
    const compressed = LZString.compressToEncodedURIComponent(dot);
    url.hash = compressed;
    history.replaceState(null, '', url);
    return;
  }
});

// copy url to clipboard
/** @type {HTMLButtonElement | null} */
const button = document.getElementById('copy-url-button');
const buttonText = button?.textContent;

let copyUrlButtonTimeout = null;
button?.addEventListener('click', () => {
  if (copyUrlButtonTimeout) {
    clearTimeout(copyUrlButtonTimeout);
  }
  const previousText = buttonText ?? "URL"; // should never need the fallback
  button.textContent = 'Copied! 🎉';
  copyUrlButtonTimeout = setTimeout(() => {
    button.textContent = previousText;
  }, 2000);

  // update the url hash and copy to clipboard
  const dot = textarea.value;
  const compressed = LZString.compressToEncodedURIComponent(dot);
  const url = new URL(location);
  url.hash = compressed;
  history.replaceState(null, '', url);
  navigator.clipboard.writeText(url.toString());
});

// download svg
/** @type {HTMLButtonElement | null} */
const downloadSvgButton = document.getElementById('download-svg-button');

downloadSvgButton?.addEventListener('click', () => {
  const svg = document.getElementById('graph');
  const svgData = svg.outerHTML;
  const svgBlob = new Blob([svgData], { type: "image/svg+xml;charset=utf-8" });
  const svgUrl = URL.createObjectURL(svgBlob);
  const downloadLink = document.createElement("a");
  downloadLink.href = svgUrl;
  downloadLink.download = "graph.svg";
  document.body.appendChild(downloadLink);
  downloadLink.click();
  document.body.removeChild(downloadLink);
});

// download png
/** @type {HTMLButtonElement | null} */
const downloadPngButton = document.getElementById('download-png-button');

const SCALE = 2;
downloadPngButton?.addEventListener('click', () => {
  const svg = document.getElementById('graph');
  const svgData = new XMLSerializer().serializeToString(svg);
  const svgBlob = new Blob([svgData], { type: 'image/svg+xml;charset=utf-8' });
  const svgUrl = URL.createObjectURL(svgBlob);
  const img = new Image();
  img.onload = function () {
    const canvas = document.createElement('canvas');
    const ctx = canvas.getContext('2d');

    canvas.width = svg.width.baseVal.value * 2;
    canvas.height = svg.height.baseVal.value * 2;

    // Draw the image to the canvas
    ctx.drawImage(img, 0, 0, canvas.width, canvas.height);

    // Now get the PNG data from the canvas
    const pngData = canvas.toDataURL('image/png');

    // Create a download link and click it
    const downloadLink = document.createElement('a');
    downloadLink.href = pngData;
    downloadLink.download = 'graph.png';
    downloadLink.click();
  };
  // Load the SVG URL into the image
  img.src = svgUrl;
});
