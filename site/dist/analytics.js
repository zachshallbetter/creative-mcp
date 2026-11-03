// Google Analytics 4 for creativemcp.com. Page views come from gtag config;
// interactions arrive as creative-mcp:interaction events from /events.js.
(function () {
  "use strict";
  var measurementId = "G-JBJ1NMP96V";
  window.dataLayer = window.dataLayer || [];
  function gtag() { window.dataLayer.push(arguments); }
  window.gtag = window.gtag || gtag;

  var config = {};
  // Local previews and ?debug_analytics show up in GA DebugView; the developer traffic filter keeps them out of reports.
  if (/^(localhost|127\.0\.0\.1|\[::1\])$/.test(location.hostname) || /[?&]debug_analytics\b/.test(location.search)) config.debug_mode = true;
  gtag("js", new Date());
  gtag("config", measurementId, config);

  function linkEventName(eventId) {
    if (eventId === "theme_toggle") return "theme_change";
    if (eventId.indexOf("nav_") === 0) return "navigation_click";
    return "cta_click";
  }

  window.addEventListener("creative-mcp:interaction", function (event) {
    var detail = event.detail || {};
    var params = {};
    if (detail.eventId) params.link_id = detail.eventId;
    if (detail.destination) params.link_url = detail.destination;
    if (detail.eventId === "theme_toggle") params.theme = document.documentElement.dataset.theme;
    Object.keys(detail.params || {}).forEach(function (key) { params[key] = detail.params[key]; });
    var name = detail.eventName || (detail.eventId && linkEventName(detail.eventId));
    if (name) gtag("event", name, params);
  });
})();
