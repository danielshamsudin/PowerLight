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

searchInput.focus();

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

window.addEventListener("keydown", async (e) => {
  if (e.key.length === 1 && !e.ctrlKey && !e.altKey && !e.metaKey) {
    searchInput.focus();
    return;
  }

  if (e.key === "Escape") {
    e.preventDefault();
    e.stopPropagation();
    await hideWindow();
    return;
  }

  if (e.key === "ArrowDown") {
    e.preventDefault();
    e.stopPropagation();
    if (results.length > 0) {
      selectedIndex = Math.min(selectedIndex + 1, results.length - 1);
      renderResults();
    }
    return;
  }

  if (e.key === "ArrowUp") {
    e.preventDefault();
    e.stopPropagation();
    if (results.length > 0) {
      selectedIndex = Math.max(selectedIndex - 1, 0);
      renderResults();
    }
    return;
  }

  if (e.key === "Enter") {
    e.preventDefault();
    e.stopPropagation();
    if (results.length > 0) {
      await launchSelected();
    }
    return;
  }
});

async function hideWindow() {
  const window = getCurrentWindow();
  searchInput.value = "";
  results = [];
  renderResults();
  await updateWindowHeight();
  await window.hide();
}

async function launchSelected() {
  const selected = results[selectedIndex];
  if (selected) {
    await invoke("launch", { path: selected.path });
    await hideWindow();
  }
}

function renderResults() {
  resultsContainer.innerHTML = results
    .map(
      (r, i) => `
      <div class="result-item ${i === selectedIndex ? "selected" : ""}" data-index="${i}">
        <div class="result-content">
          <div class="result-header">
            <div class="result-name">${escapeHtml(r.name)}</div>
            <span class="result-badge badge-${r.kind}">${r.kind}</span>
          </div>
          <div class="result-path">${escapeHtml(r.path)}</div>
        </div>
      </div>
    `
    )
    .join("");

  // Re-attach click handlers
  resultsContainer.querySelectorAll(".result-item").forEach((el) => {
    el.addEventListener("click", async (e) => {
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

  appContainer.style.height = `${newHeight}px`;

  if (results.length > maxVisibleResults) {
    resultsContainer.style.overflowY = "auto";
  } else {
    resultsContainer.style.overflowY = "hidden";
  }

  const window = getCurrentWindow();
  await window.setSize(new LogicalSize(680, newHeight));
}

function escapeHtml(str: string): string {
  const div = document.createElement("div");
  div.textContent = str;
  return div.innerHTML;
}

getCurrentWindow().onFocusChanged(({ payload: focused }) => {
  if (focused) {
    searchInput.focus();
    searchInput.select();
  }
});

updateWindowHeight();
