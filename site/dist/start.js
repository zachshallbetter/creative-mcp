(function () {
  "use strict";

  // Every choice comes from /data/setup.json, generated from the series catalogue by
  // site/scripts/generate-catalogue.py. This file only looks combinations up.
  var implementations = document.getElementById("implementation");
  var clients = document.getElementById("client");
  var methods = document.getElementById("install-method");
  var kicker = document.getElementById("setup-kicker");
  var command = document.getElementById("setup-command");
  var explanation = document.getElementById("setup-explanation");
  var copyButton = document.getElementById("copy-command");
  var checklist = document.getElementById("readiness-list");
  var count = document.getElementById("readiness-count");
  var message = document.getElementById("readiness-message");
  var preferBrew = /Mac|Linux/.test(navigator.platform || navigator.userAgent);
  var tools = {};

  function track(name, params) { if (window.creativeMcpInteraction) window.creativeMcpInteraction(name, params); }

  function selectedClient() {
    return clients.value;
  }

  function tool() {
    return tools[implementations.value];
  }

  function fillOptions(select, options) {
    select.replaceChildren();
    options.forEach(function (option) {
      var element = document.createElement("option");
      element.value = option.id;
      element.textContent = option.label;
      select.appendChild(element);
    });
  }

  function updateClientOptions() {
    fillOptions(clients, tool().clients);
    fillOptions(methods, tool().methods);
    // Homebrew first on macOS and Linux; otherwise the first other method.
    var brew = tool().methods.findIndex(function (m) { return m.id === "brew"; });
    var other = tool().methods.findIndex(function (m) { return m.id !== "brew"; });
    methods.selectedIndex = (preferBrew && brew >= 0) || other < 0 ? Math.max(brew, 0) : other;
    updateOutput();
    updateReadiness();
  }

  function updateOutput() {
    var entry = tool().entries[selectedClient() + "|" + methods.value];
    kicker.textContent = entry.kicker;
    command.textContent = entry.command;
    explanation.textContent = entry.explanation;
    copyButton.textContent = "Copy";
  }

  function updateReadiness() {
    checklist.replaceChildren();
    tool().readiness.forEach(function (text) {
      var label = document.createElement("label");
      label.className = "readiness-item";
      var input = document.createElement("input");
      input.type = "checkbox";
      var span = document.createElement("span");
      span.textContent = text;
      input.addEventListener("change", updateProgress);
      label.append(input, span);
      checklist.appendChild(label);
    });
    updateProgress();
  }

  function updateProgress() {
    var inputs = Array.prototype.slice.call(checklist.querySelectorAll("input"));
    var checked = inputs.filter(function (input) { return input.checked; }).length;
    var wasReady = count.dataset.ready === "true";
    count.dataset.ready = checked === inputs.length ? "true" : "false";
    if (checked === inputs.length && !wasReady && this instanceof HTMLInputElement) track("setup_ready", { implementation: implementations.value });
    count.textContent = checked + " of " + inputs.length + " checked";
    message.textContent = checked === inputs.length ? "You’re ready to follow the setup guide." : "Confirm each requirement on your computer before continuing.";
  }

  copyButton.addEventListener("click", async function () {
    try {
      await navigator.clipboard.writeText(command.textContent);
      track("copy_command", { command_id: "start_setup", implementation: implementations.value, client: selectedClient(), install_method: methods.value });
      copyButton.textContent = "Copied";
      window.setTimeout(function () { copyButton.textContent = "Copy"; }, 1400);
    } catch (error) {
      copyButton.textContent = "Select text";
    }
  });

  fetch("/data/setup.json")
    .then(function (response) {
      if (!response.ok) throw new Error(response.status);
      return response.json();
    })
    .then(function (data) {
      data.tools.forEach(function (t) { tools[t.id] = t; });
      implementations.addEventListener("change", updateClientOptions);
      clients.addEventListener("change", updateOutput);
      methods.addEventListener("change", updateOutput);
      [implementations, clients, methods].forEach(function (control) {
        control.addEventListener("change", function () {
          track("setup_select", { implementation: implementations.value, client: selectedClient(), install_method: methods.value });
        });
      });
      updateClientOptions();
    })
    .catch(function () {
      kicker.textContent = "Setup commands";
      command.textContent = "brew tap zachshallbetter/tap\nbrew trust zachshallbetter/tap\nbrew install zachshallbetter/tap/creative-mcp\ncreative-mcp setup";
      explanation.textContent = "The setup options couldn’t load. The installer above works for every implementation; Getting started lists each command.";
    });
}());
