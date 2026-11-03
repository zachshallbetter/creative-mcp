(function () {
  "use strict";

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
  var clientSets = {
    adobe: [
      { value: "claude", label: "Claude Code" },
      { value: "other", label: "Another client with MCP server configuration" }
    ],
    blender: [
      { value: "codex", label: "Codex" },
      { value: "claude-code", label: "Claude Code" },
      { value: "cursor", label: "Cursor" },
      { value: "gemini", label: "Gemini CLI" },
      { value: "antigravity", label: "Google Antigravity" }
    ]
  };
  var methodSets = {
    adobe: [{ value: "brew", label: "Homebrew (macOS, Linux)" }, { value: "npm", label: "npm / npx (macOS, Windows)" }],
    blender: [{ value: "brew", label: "Homebrew (macOS, Linux)" }, { value: "cargo", label: "Cargo, from source" }]
  };
  var brewTap = "brew tap zachshallbetter/tap\nbrew trust zachshallbetter/tap\n";
  var preferBrew = /Mac|Linux/.test(navigator.platform || navigator.userAgent);
  var readiness = {
    adobe: [
      "Node.js 18 or later is installed (Homebrew installs it for you).",
      "The Adobe app you want to use is installed on this computer.",
      "Your MCP client can launch local stdio servers.",
      "You know which project folders the server should be allowed to access."
    ],
    blender: [
      "Rust and Cargo are installed (or Homebrew, if you install with brew).",
      "Blender 4.0 or later is installed.",
      "The Blender MCP companion add-on is installed and enabled.",
      "Your MCP client can launch local stdio servers."
    ]
  };

  function track(name, params) { if (window.creativeMcpInteraction) window.creativeMcpInteraction(name, params); }

  function selectedClient() {
    return clients.value;
  }

  function fillOptions(select, options) {
    select.replaceChildren();
    options.forEach(function (option) {
      var element = document.createElement("option");
      element.value = option.value;
      element.textContent = option.label;
      select.appendChild(element);
    });
  }

  function updateClientOptions() {
    fillOptions(clients, clientSets[implementations.value]);
    fillOptions(methods, methodSets[implementations.value]);
    methods.selectedIndex = preferBrew ? 0 : 1;
    updateOutput();
    updateReadiness();
  }

  function updateOutput() {
    var product = implementations.value;
    var client = selectedClient();
    var selected = clients.options[clients.selectedIndex];
    var brew = methods.value === "brew";
    var config;
    if (product === "adobe" && client === "claude") {
      kicker.textContent = "Adobe MCP · Claude Code · " + (brew ? "Homebrew" : "npx");
      command.textContent = brew
        ? brewTap + "brew install zachshallbetter/tap/adobe-mcp\nclaude mcp add --scope user adobe -- adobe-mcp-suite"
        : "claude mcp add --scope user adobe -- npx -y adobe-mcp-suite";
      explanation.textContent = (brew ? "Installs the coordinator and every Adobe app server with Homebrew, then registers the coordinator in Claude Code. " : "Registers the Adobe MCP coordinator in Claude Code. ") + "It currently provides two Illustrator-to-InDesign workflows; add standalone app servers separately if you need their direct tools.";
    } else if (product === "adobe") {
      kicker.textContent = "Adobe MCP · client configuration · " + (brew ? "Homebrew" : "npx");
      config = { mcpServers: { adobe: brew ? { command: "adobe-mcp-suite" } : { command: "npx", args: ["-y", "adobe-mcp-suite"] } } };
      command.textContent = (brew ? brewTap + "brew install zachshallbetter/tap/adobe-mcp\n\n" : "") + JSON.stringify(config, null, 2);
      explanation.textContent = (brew ? "Install with Homebrew, then add" : "Add") + " this server entry using your client’s documented configuration method. The Adobe coordinator currently provides workflows between InDesign and Illustrator.";
    } else {
      kicker.textContent = "Blender MCP · " + selected.textContent + " · " + (brew ? "Homebrew" : "Cargo");
      command.textContent = (brew
        ? brewTap + "brew install --HEAD zachshallbetter/tap/blender-mcp"
        : "cargo install --locked --git https://github.com/zachshallbetter/blender-mcp-rs --package blender-mcp-cli") + "\nblender-mcp addon install\nblender-mcp client add " + client;
      explanation.textContent = (brew ? "Homebrew builds the CLI from the main branch (there is no tagged release yet). " : "") + "Install the CLI, install and enable its companion add-on in Blender, then register the local server with this client. Blender MCP requires Blender 4.0 or later.";
    }
    copyButton.textContent = "Copy";
  }

  function updateReadiness() {
    var items = readiness[implementations.value];
    checklist.replaceChildren();
    items.forEach(function (text) {
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
  implementations.addEventListener("change", updateClientOptions);
  clients.addEventListener("change", updateOutput);
  methods.addEventListener("change", updateOutput);
  [implementations, clients, methods].forEach(function (control) {
    control.addEventListener("change", function () {
      track("setup_select", { implementation: implementations.value, client: selectedClient(), install_method: methods.value });
    });
  });
  updateClientOptions();
}());
