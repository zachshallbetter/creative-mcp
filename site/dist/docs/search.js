(function () {
  "use strict";
  var pages = [
    ["/docs/", "Documentation home"],
    ["/docs/getting-started.html", "Getting started"],
    ["/docs/content.html", "Content & capability guide"],
    ["/docs/api.html", "MCP API reference"],
    ["/docs/cli.html", "CLI & configuration"],
    ["/docs/architecture.html", "Architecture & distribution"],
    ["/docs/implementations/indesign.html", "InDesign"],
    ["/docs/implementations/illustrator.html", "Illustrator"],
    ["/docs/implementations/photoshop.html", "Photoshop"],
    ["/docs/implementations/aftereffects.html", "After Effects"],
    ["/docs/implementations/premiere.html", "Premiere Pro"],
    ["/docs/implementations/suite.html", "Adobe MCP coordinator"],
    ["/docs/implementations/blender.html", "Blender MCP"],
    ["/docs/implementations/core.html", "Shared core"],
    ["/docs/contributing.html", "Contributing an implementation"]
  ];
  var slots = Array.prototype.slice.call(document.querySelectorAll(".lede"));
  if (!slots.length) return;

  var host = document.createElement("div");
  host.className = "docs-search";
  host.innerHTML = '<label for="docs-search-input">Search documentation</label><div class="docs-search-field"><span aria-hidden="true">⌕</span><input id="docs-search-input" type="search" autocomplete="off" placeholder="Search setup, apps, tools, API…" aria-controls="docs-search-results" aria-describedby="docs-search-status"><span class="docs-search-shortcut">⌘ / Ctrl K</span></div><div class="docs-search-results" id="docs-search-results" role="region" aria-label="Documentation search results" hidden></div><div class="sr-only" id="docs-search-status" aria-live="polite"></div>';
  slots[0].insertAdjacentElement("afterend", host);
  var input = host.querySelector("input");
  var results = host.querySelector(".docs-search-results");
  var status = host.querySelector("#docs-search-status");
  var indexPromise;
  var timer;
  var trackTimer;
  var lastTracked = "";

  function track(name, params) { if (window.creativeMcpInteraction) window.creativeMcpInteraction(name, params); }

  function textOf(node) {
    return (node.textContent || "").replace(/\s+/g, " ").trim();
  }
  function loadIndex() {
    if (!indexPromise) {
      indexPromise = Promise.all(pages.map(function (page) {
        return fetch(page[0]).then(function (response) {
          if (!response.ok) throw new Error("Unable to load documentation page");
          return response.text();
        }).then(function (markup) {
          var doc = new DOMParser().parseFromString(markup, "text/html");
          var main = doc.querySelector("main");
          if (!main) return [];
          return Array.prototype.map.call(main.querySelectorAll(".doc-section, .story-section"), function (section, i) {
            var heading = section.querySelector("h2, h3");
            var title = heading ? textOf(heading) : page[1];
            if (!section.id) section.id = "section-" + (i + 1);
            return { pageTitle: page[1], title: title, url: page[0] + "#" + section.id, body: textOf(section) };
          });
        });
      })).then(function (entries) { return [].concat.apply([], entries); });
    }
    return indexPromise;
  }
  function snippet(text, terms) {
    var lower = text.toLowerCase();
    var at = terms.reduce(function (best, term) { var i = lower.indexOf(term); return i >= 0 && (best < 0 || i < best) ? i : best; }, -1);
    if (at < 0) return text.slice(0, 155) + (text.length > 155 ? "…" : "");
    var start = Math.max(0, at - 55), end = Math.min(text.length, at + 115);
    return (start ? "…" : "") + text.slice(start, end) + (end < text.length ? "…" : "");
  }
  function render(items, terms) {
    results.replaceChildren();
    if (!items.length) {
      var empty = document.createElement("div"); empty.className = "docs-search-message"; empty.textContent = "No matching documentation found."; results.appendChild(empty);
    } else {
      items.slice(0, 8).forEach(function (item) {
        var link = document.createElement("a"); link.className = "docs-search-result"; link.href = item.url;
        var title = document.createElement("strong"); title.textContent = item.pageTitle;
        var section = document.createElement("span"); section.className = "result-section"; section.textContent = item.title;
        var excerpt = document.createElement("span"); excerpt.className = "result-snippet"; excerpt.textContent = snippet(item.body, terms);
        link.append(title, section, excerpt); results.appendChild(link);
      });
    }
    results.hidden = false;
    status.textContent = items.length ? items.length + " matching sections" : "No matching documentation found.";
    clearTimeout(trackTimer);
    trackTimer = setTimeout(function () {
      var term = input.value.trim();
      if (term === lastTracked || term.split(/\s+/).some(function (part) { return part.length < 2; })) return;
      lastTracked = term;
      track("search", { search_term: term, search_scope: "docs", results_count: items.length });
    }, 1000);
  }
  function close() { results.hidden = true; }
  input.addEventListener("input", function () {
    clearTimeout(timer);
    var terms = input.value.toLowerCase().trim().split(/\s+/).filter(Boolean);
    if (!terms.length) { close(); status.textContent = ""; return; }
    if (terms.some(function (term) { return term.length < 2; })) { render([], terms); results.firstChild.textContent = "Enter at least two characters per search term."; status.textContent = "Enter at least two characters per search term."; return; }
    results.hidden = false; results.replaceChildren();
    var loading = document.createElement("div"); loading.className = "docs-search-message"; loading.textContent = "Searching documentation…"; results.appendChild(loading);
    timer = setTimeout(function () {
      loadIndex().then(function (entries) {
        var matches = entries.map(function (entry) {
          var title = (entry.pageTitle + " " + entry.title).toLowerCase(), body = entry.body.toLowerCase();
          var score = 0, matched = 0;
          terms.forEach(function (term) {
            var inTitle = title.indexOf(term) >= 0, inBody = body.indexOf(term) >= 0;
            if (inTitle || inBody) matched++;
            if (inTitle) score += 8;
            if (inBody) score++;
          });
          return { entry: entry, score: score, matched: matched };
        }).filter(function (match) { return match.matched === terms.length; }).sort(function (a, b) { return b.score - a.score || a.entry.pageTitle.localeCompare(b.entry.pageTitle); }).map(function (match) { return match.entry; });
        render(matches, terms);
      }).catch(function () { results.replaceChildren(); var error = document.createElement("div"); error.className = "docs-search-message"; error.textContent = "Search could not load the documentation. Check your connection and try again."; results.appendChild(error); results.hidden = false; });
    }, 120);
  });
  input.addEventListener("keydown", function (event) { if (event.key === "Escape") { close(); input.blur(); } });
  results.addEventListener("click", function (event) {
    var link = event.target.closest(".docs-search-result");
    if (link) track("select_content", { content_type: "docs_search_result", item_id: link.getAttribute("href"), search_term: input.value.trim() });
  });
  document.addEventListener("click", function (event) { if (!host.contains(event.target)) close(); });
  document.addEventListener("keydown", function (event) {
    if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "k") { event.preventDefault(); input.focus(); input.select(); }
  });
})();
