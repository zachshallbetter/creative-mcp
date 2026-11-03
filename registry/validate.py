#!/usr/bin/env python3
"""Validate registry.json against registry.schema.json and check cross-references."""
import json
import sys
from pathlib import Path

import jsonschema

HERE = Path(__file__).resolve().parent
schema = json.loads((HERE / "registry.schema.json").read_text())
data = json.loads((HERE / "registry.json").read_text())

jsonschema.Draft202012Validator.check_schema(schema)
errors = [f"{'/'.join(map(str, e.path))}: {e.message}" for e in jsonschema.Draft202012Validator(schema).iter_errors(data)]

client_ids = [c["id"] for c in data["clients"]]
tool_ids = [t["id"] for t in data["tools"]]
for ids, kind in ((client_ids, "client"), (tool_ids, "tool")):
    errors += [f"duplicate {kind} id: {i}" for i in sorted({i for i in ids if ids.count(i) > 1})]
for tool in data["tools"]:
    errors += [f"{tool['id']}: unknown client {c}" for c in tool["clients"] if c not in client_ids]
    apps = {t["app"] for t in tool["targets"]}
    errors += [f"{tool['id']}: server {s['command']} targets unknown app {s['app']}" for s in tool["servers"] if s.get("app") and s["app"] not in apps]

if errors:
    print("\n".join(errors))
    sys.exit(1)
print(f"registry.json is valid: {len(tool_ids)} tools, {len(client_ids)} clients")
