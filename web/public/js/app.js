(function () {
  "use strict";

  // --- Reading Progress Bar ---
  var progressBar = document.getElementById("progress-bar");
  function updateProgress() {
    var scrollTop = window.scrollY;
    var docHeight = document.documentElement.scrollHeight - window.innerHeight;
    var pct = docHeight > 0 ? (scrollTop / docHeight) * 100 : 0;
    if (progressBar) progressBar.style.width = Math.min(pct, 100) + "%";
  }
  window.addEventListener("scroll", updateProgress, { passive: true });
  updateProgress();

  // --- Theme Toggle ---
  var THEME_KEY = "tlm-theme";
  function getPreferred() {
    var stored = localStorage.getItem(THEME_KEY);
    if (stored) return stored;
    return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
  }
  function applyTheme(theme) {
    document.documentElement.setAttribute("data-theme", theme);
    localStorage.setItem(THEME_KEY, theme);
    var btn = document.getElementById("theme-toggle");
    if (btn) btn.textContent = theme === "dark" ? "\u2600" : "\u263E";
  }

  // Create theme toggle button
  (function () {
    var btn = document.createElement("button");
    btn.className = "theme-toggle";
    btn.id = "theme-toggle";
    btn.setAttribute("aria-label", "Toggle theme");
    btn.setAttribute("title", "Toggle dark/light theme");
    btn.addEventListener("click", function () {
      var current = document.documentElement.getAttribute("data-theme") || getPreferred();
      applyTheme(current === "dark" ? "light" : "dark");
    });
    document.body.appendChild(btn);
    applyTheme(getPreferred());
  })();

  // --- TOC Sidebar ---
  var tocToggle = document.getElementById("toc-toggle");
  var tocSidebar = document.getElementById("toc-sidebar");
  var tocClose = document.getElementById("toc-close");
  var overlay = null;

  function createOverlay() {
    if (overlay) return overlay;
    overlay = document.createElement("div");
    overlay.className = "toc-overlay";
    overlay.addEventListener("click", closeToc);
    document.body.appendChild(overlay);
    return overlay;
  }

  function openToc() {
    if (!tocSidebar) return;
    tocSidebar.classList.add("open");
    createOverlay().classList.add("active");
    document.body.style.overflow = "hidden";
  }

  function closeToc() {
    if (!tocSidebar) return;
    tocSidebar.classList.remove("open");
    if (overlay) overlay.classList.remove("active");
    document.body.style.overflow = "";
  }

  if (tocToggle) tocToggle.addEventListener("click", openToc);
  if (tocClose) tocClose.addEventListener("click", closeToc);

  // Close TOC on Escape
  document.addEventListener("keydown", function (e) {
    if (e.key === "Escape") closeToc();
  });

  // Close TOC when clicking a link inside it
  if (tocSidebar) {
    tocSidebar.addEventListener("click", function (e) {
      if (e.target.closest(".toc-link")) closeToc();
    });
  }

  // --- Highlight current chapter in TOC ---
  var currentChapter = document.querySelector("[data-chapter]");
  if (currentChapter) {
    var slug = currentChapter.getAttribute("data-chapter");
    var links = document.querySelectorAll(".toc-link");
    links.forEach(function (link) {
      if (link.getAttribute("href") === "/chapter/" + slug) {
        link.style.color = "var(--accent)";
        link.style.fontWeight = "600";
      }
    });
  }
})();
