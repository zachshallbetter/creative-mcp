(function () {
  "use strict";
  var key = "adobe-mcp-theme";
  var root = document.documentElement;
  var buttons = Array.prototype.slice.call(document.querySelectorAll(".theme-toggle"));

  function apply(theme, persist) {
    var dark = theme === "dark";
    root.dataset.theme = dark ? "dark" : "light";
    if (persist) {
      try { localStorage.setItem(key, dark ? "dark" : "light"); } catch (error) { /* Theme still applies for this page. */ }
    }
    buttons.forEach(function (button) {
      button.setAttribute("aria-pressed", dark ? "true" : "false");
      button.setAttribute("aria-label", "Switch to " + (dark ? "light" : "dark") + " theme");
      var label = button.querySelector(".theme-toggle-label");
      var icon = button.querySelector(".theme-toggle-icon");
      if (label) label.textContent = dark ? "Light" : "Dark";
      if (icon) icon.textContent = dark ? "☼" : "◐";
    });
    // An explicit choice overrides the system-matched theme-color pair.
    document.querySelectorAll('meta[name="theme-color"]').forEach(function (meta) {
      meta.setAttribute("content", dark ? "#111417" : "#f4f0e8");
    });
  }

  apply(root.dataset.theme, false);
  buttons.forEach(function (button) {
    button.addEventListener("click", function () {
      apply(root.dataset.theme === "dark" ? "light" : "dark", true);
    });
  });
  window.addEventListener("storage", function (event) {
    if (event.key === key) apply(event.newValue === "dark" ? "dark" : "light", false);
  });
  // With no saved choice, follow the system theme as it changes.
  var systemDark = window.matchMedia("(prefers-color-scheme: dark)");
  systemDark.addEventListener("change", function (event) {
    var stored = null;
    try { stored = localStorage.getItem(key); } catch (error) { /* Treat as no saved choice. */ }
    if (stored !== "dark" && stored !== "light") apply(event.matches ? "dark" : "light", false);
  });
})();
