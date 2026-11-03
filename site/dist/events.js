// Components report named interactions here; /analytics.js forwards them to GA4.
window.creativeMcpInteraction = function (eventName, params) {
  window.dispatchEvent(new CustomEvent("creative-mcp:interaction", {
    detail: { eventName: eventName, params: params || {} }
  }));
};

document.addEventListener("click", function (event) {
  var control = event.target.closest("[data-event-id]");
  if (!control) return;
  var canonical = document.querySelector('link[rel="canonical"]');
  var pageId = canonical ? new URL(canonical.href).pathname : location.pathname;
  var destination = null;
  if (control.matches("a[href]")) {
    var url = new URL(control.href, location.href);
    destination = url.origin + url.pathname;
  }
  window.dispatchEvent(new CustomEvent("creative-mcp:interaction", {
    detail: { eventId: control.dataset.eventId, pageId: pageId, destination: destination }
  }));
});
