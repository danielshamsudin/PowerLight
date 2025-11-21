import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";

interface SearchResult {
  name: string;
  path: string;
  kind: string;
}

let results: SearchResult[] = [];
let selectedIndex = 0;

const searchInput = document.getElementById("search-input") as HTMLInputElement;
const resultsContainer = document.getElementById("results")!;
const appContainer = document.getElementById("app")!;

let debounceTimer: number;

console.log("Powerlight initialized");

searchInput.addEventListener("input", () => {
  clearTimeout(debounceTimer);
  debounceTimer = setTimeout(async () => {
    const query = searchInput.value.trim();
    if (query.length === 0) {
      results = [];
      renderResults();
      await updateWindowHeight();
      return;
    }

    results = await invoke<SearchResult[]>("search", { query });
    selectedIndex = 0;
    renderResults();
    await updateWindowHeight();
  }, 100);
});

// Global keydown handler
window.addEventListener("keydown", async (e) => {
  console.log("Key pressed:", e.key);

  if (e.key === "Escape") {
    console.log("Escape pressed, hiding window");
    e.preventDefault();
    e.stopPropagation();
    await hideWindow();
    return;
  }

  if (e.key === "ArrowDown") {
    e.preventDefault();
    if (results.length > 0) {
      selectedIndex = Math.min(selectedIndex + 1, results.length - 1);
      renderResults();
    }
    return;
  }

  if (e.key === "ArrowUp") {
    e.preventDefault();
    if (results.length > 0) {
      selectedIndex = Math.max(selectedIndex - 1, 0);
      renderResults();
    }
    return;
  }

  if (e.key === "Enter") {
    e.preventDefault();
    if (results.length > 0) {
      console.log("Enter pressed, launching app");
      await launchSelected();
    }
    return;
  }
}, true); // Use capture phase

async function hideWindow() {
  console.log("Hiding window...");
  const window = getCurrentWindow();
  searchInput.value = "";
  results = [];
  renderResults();
  await updateWindowHeight();
  await window.hide();
  console.log("Window hidden");
}

async function launchSelected() {
  const selected = results[selectedIndex];
  console.log("Launching:", selected);
  if (selected) {
    try {
      await invoke("launch", { path: selected.path });
      console.log("Launch command sent, hiding window");
      await hideWindow();
    } catch (e) {
      console.error("Failed to launch:", e);
    }
  }
}

function renderResults() {
  resultsContainer.innerHTML = results
    .map(
      (r, i) => `
      <div class="result-item ${i === selectedIndex ? "selected" : ""}" data-index="${i}">
        <div>
          <div class="result-name">${escapeHtml(r.name)}</div>
          <div class="result-path">${escapeHtml(r.path)}</div>
        </div>
      </div>
    `
    )
    .join("");

  // Re-attach click handlers
  resultsContainer.querySelectorAll(".result-item").forEach((el) => {
    el.addEventListener("click", async (e) => {
      console.log("Result clicked");
      e.preventDefault();
      e.stopPropagation();
      const index = parseInt(el.getAttribute("data-index")!);
      selectedIndex = index;
      await launchSelected();
    }, true);
  });
}

async function updateWindowHeight() {
  const baseHeight = 72;
  const itemHeight = 58;
  const maxVisibleResults = 4;
  const resultCount = Math.min(results.length, maxVisibleResults);
  const newHeight = baseHeight + resultCount * itemHeight;

  console.log(`Updating height to ${newHeight}px for ${results.length} results`);

  // Update container height
  appContainer.style.height = `${newHeight}px`;

  // Show scrollbar if more than 4 results
  if (results.length > maxVisibleResults) {
    resultsContainer.style.overflowY = "auto";
  } else {
    resultsContainer.style.overflowY = "hidden";
  }

  // Update window size
  try {
    const window = getCurrentWindow();
    await window.setSize(new LogicalSize(680, newHeight));
  } catch (e) {
    console.error("Failed to resize window:", e);
  }
}

function escapeHtml(str: string): string {
  const div = document.createElement("div");
  div.textContent = str;
  return div.innerHTML;
}

// Focus input when window becomes visible
getCurrentWindow().onFocusChanged(({ payload: focused }) => {
  console.log("Focus changed:", focused);
  if (focused) {
    searchInput.focus();
    searchInput.select();
  }
});

// Initialize height
updateWindowHeight();

console.log("Event listeners attached");
