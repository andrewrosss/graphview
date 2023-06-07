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
