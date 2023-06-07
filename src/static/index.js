/** @type {HTMLTextAreaElement | null} */
const textarea = document.getElementById('dot');

textarea?.addEventListener('input', (event) => {
  const dot = event.target.value;
  const compressed = LZString.compressToEncodedURIComponent(dot);
  const url = new URL(location);
  url.hash = compressed;
  history.replaceState(null, '', url);
});

window.addEventListener("DOMContentLoaded", () => {
  const url = new URL(location);
  const compressed = url.hash.slice(1);
  if (!compressed) {
    return;
  }
  const dot = LZString.decompressFromEncodedURIComponent(compressed);
  textarea.value = dot;
  textarea.dispatchEvent(new Event('input'));
});

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

  const dot = textarea.value;
  const compressed = LZString.compressToEncodedURIComponent(dot);
  const url = new URL(location);
  url.hash = compressed;
  history.replaceState(null, '', url);
  navigator.clipboard.writeText(url.toString());
});
