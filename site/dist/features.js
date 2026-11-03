    var inventoryApps = [
      { id: "indesign", name: "InDesign", badge: "ID", tone: "var(--purple)", count: 2742 },
      { id: "illustrator", name: "Illustrator", badge: "AI", tone: "var(--orange)", count: 2540 },
      { id: "photoshop", name: "Photoshop", badge: "PS", tone: "var(--blue)", count: 2609 },
      { id: "aftereffects", name: "After Effects", badge: "Ae", tone: "var(--red)", count: 3198 },
      { id: "premiere", name: "Premiere Pro", badge: "Pr", tone: "var(--green)", count: 925 }
    ];
    var inventoryRoot = document.getElementById("inventory-apps");
    var loadedInventories = new Map();

    function track(name, params) { if (window.creativeMcpInteraction) window.creativeMcpInteraction(name, params); }

    function element(tag, className, text) {
      var node = document.createElement(tag);
      if (className) node.className = className;
      if (text !== undefined) node.textContent = text;
      return node;
    }

    function renderFeature(feature) {
      var card = element("article", "feature-record");
      var head = element("div", "feature-record-head");
      head.append(element("h4", "", feature.name || feature.id));
      head.append(element("span", "feature-tags", [feature.kind, feature.automation && feature.automation.route].filter(Boolean).join(" · ")));
      card.append(head);
      if (feature.summary) card.append(element("p", "", feature.summary));

      var evidence = element("details", "feature-evidence");
      evidence.append(element("summary", "", "Automation and evidence"));
      var automation = feature.automation || {};
      ["notes", "menu_path", "shortcut", "dom_refs", "uxp_refs", "menu_command_ids", "event_ids"].forEach(function (key) {
        var value = automation[key] || feature[key];
        if (value && (!Array.isArray(value) || value.length)) {
          var label = key.replaceAll("_", " ");
          evidence.append(element("p", "", label + ": " + (Array.isArray(value) ? value.join(" → ") : value)));
        }
      });
      var sources = Array.isArray(feature.sources) ? feature.sources : [];
      if (sources.length) {
        var list = element("ul", "feature-sources");
        sources.forEach(function (source, index) {
          try {
            var url = new URL(source);
            if (url.protocol !== "https:") return;
            var item = element("li");
            var link = element("a", "", "Source " + (index + 1));
            link.href = url.href;
            link.target = "_blank";
            link.rel = "noopener noreferrer";
            item.append(link);
            list.append(item);
          } catch (error) { /* Ignore malformed source references. */ }
        });
        if (list.childElementCount) evidence.append(list);
      }
      if (evidence.childElementCount > 1) card.append(evidence);
      return card;
    }

    function buildInventory(app, data, container) {
      var controls = element("div", "feature-controls");
      var search = element("input");
      search.type = "search";
      search.placeholder = "Search features, domains, or routes";
      search.setAttribute("aria-label", "Search " + app.name + " features");
      var routeFilter = element("select");
      routeFilter.setAttribute("aria-label", "Filter by automation route");
      var routes = Array.from(new Set((data.domains || []).flatMap(function (domain) {
        return (domain.features || []).map(function (feature) { return feature.automation && feature.automation.route || "unknown"; });
      }))).sort();
      [["", "All automation routes"]].concat(routes.map(function (route) {
        var label = route.replaceAll("_", " ").replace(/\b\w/g, function (letter) { return letter.toUpperCase(); });
        return [route, label];
      })).forEach(function (entry) {
        var option = element("option", "", entry[1]);
        option.value = entry[0];
        routeFilter.append(option);
      });
      var count = element("output", "", data.featureCount + " features");
      controls.append(search, routeFilter, count);
      container.append(controls);

      var domainNodes = [];
      (data.domains || []).forEach(function (domain) {
        var section = element("details", "feature-domain");
        var summary = element("summary");
        summary.append(element("span", "domain-name", domain.name));
        summary.append(element("span", "domain-meta", (domain.features || []).length + " features"));
        section.append(summary);
        if (domain.covers) section.append(element("p", "domain-covers", domain.covers));
        var records = element("div", "feature-records");
        var rows = (domain.features || []).map(function (feature) {
          var haystack = [feature.id, feature.name, feature.kind, feature.domain, feature.summary, domain.name, domain.covers,
            JSON.stringify(feature.automation || {}), (feature.sources || []).join(" ")].join(" ").toLowerCase();
          return { feature: feature, haystack: haystack, route: feature.automation && feature.automation.route || "unknown", matches: true };
        });
        section.append(records);
        section.addEventListener("toggle", function () {
          if (!section.open) return;
          records.replaceChildren();
          rows.forEach(function (row) { if (row.matches) records.append(renderFeature(row.feature)); });
        });
        container.append(section);
        domainNodes.push({ section: section, rows: rows, records: records });
      });

      function applyFilters() {
        var query = search.value.trim().toLowerCase();
        var route = routeFilter.value;
        var visible = 0;
        domainNodes.forEach(function (domain) {
          var domainVisible = 0;
          domain.rows.forEach(function (row) {
            var matches = (!query || row.haystack.includes(query)) && (!route || row.route === route);
            row.matches = matches;
            if (matches) domainVisible++;
          });
          domain.section.hidden = domainVisible === 0;
          if (domain.section.open) {
            domain.records.replaceChildren();
            domain.rows.forEach(function (row) { if (row.matches) domain.records.append(renderFeature(row.feature)); });
          }
          domainVisible && (visible += domainVisible);
        });
        count.textContent = visible + " of " + data.featureCount + " features";
      }
      var searchTimer;
      search.addEventListener("input", applyFilters);
      search.addEventListener("input", function () {
        clearTimeout(searchTimer);
        searchTimer = setTimeout(function () {
          var term = search.value.trim();
          if (term.length >= 2) track("search", { search_term: term, search_scope: app.id + "_features" });
        }, 1000);
      });
      routeFilter.addEventListener("change", applyFilters);
      routeFilter.addEventListener("change", function () { track("filter_features", { app: app.id, route: routeFilter.value || "all" }); });
      applyFilters();
    }

    inventoryApps.forEach(function (app, index) {
      var disclosure = element("details", "feature-browser");
      disclosure.style.setProperty("--tone", app.tone);
      var summary = element("summary");
      summary.append(element("span", "inventory-badge", app.badge));
      var title = element("span");
      title.append(element("span", "inventory-title", app.name));
      title.append(element("span", "inventory-subtitle", "Full app capability inventory · atlas data"));
      summary.append(title);
      summary.append(element("span", "inventory-count", app.count.toLocaleString() + " documented features"));
      disclosure.append(summary);
      var content = element("div", "feature-content");
      disclosure.append(content);
      disclosure.addEventListener("toggle", function () {
        if (!disclosure.open) return;
        track("view_inventory", { app: app.id });
        if (loadedInventories.has(app.id)) return;
        loadedInventories.set(app.id, "loading");
        content.append(element("p", "feature-empty", "Loading the " + app.name + " feature inventory…"));
        fetch("data/" + app.id + ".json").then(function (response) {
          if (!response.ok) throw new Error("Inventory request failed (" + response.status + ").");
          return response.json();
        }).then(function (data) {
          content.replaceChildren();
          buildInventory(app, data, content);
          loadedInventories.set(app.id, data);
        }).catch(function () {
          content.replaceChildren(element("p", "feature-error", "This inventory could not be loaded. Check your connection and reload the page to try again."));
          loadedInventories.delete(app.id);
        });
      });
      inventoryRoot.append(disclosure);
    });
