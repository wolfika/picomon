#!/usr/bin/env bash
# Starts a release binary and checks that it serves the embedded dashboard and
# real metrics. Usage: smoke-test.sh <binary> [expected-version]
set -euo pipefail

bin=$(realpath "$1")
version=${2:-}
port=7399
dir=$(mktemp -d)
trap 'kill "${pid:-}" 2>/dev/null || true; rm -rf "$dir"' EXIT

"$bin" --version
if [[ -n $version ]]; then
  "$bin" --version | grep -qx "picomon $version" || { echo "expected version $version" >&2; exit 1; }
fi

printf '[server]\nlisten = "127.0.0.1:%s"\n[collector]\ninterval = 1.0\n' "$port" > "$dir/config.toml"
"$bin" --config "$dir/config.toml" &
pid=$!

# /api/sample returns 503 until the first sample is collected.
for _ in $(seq 1 30); do
  curl -fsS "http://127.0.0.1:$port/api/sample" -o "$dir/sample.json" 2>/dev/null && break
  sleep 1
done
[[ -s $dir/sample.json ]] || { echo "no sample after 30 s" >&2; exit 1; }

# A build without web/dist serves a plain-text "Frontend not built" page instead.
curl -fsS "http://127.0.0.1:$port/" | grep -q '<div id="app">' || {
  echo "index.html is missing; the frontend was not embedded" >&2
  exit 1
}

python3 - "$dir/sample.json" <<'PY'
import json, sys
s = json.load(open(sys.argv[1]))
assert s["mem"]["total"] > 0, "memory total is zero"
assert s["cpu"]["cores"], "no CPU cores"
print(f"ok: seq {s['seq']}, {len(s['cpu']['cores'])} cores, {len(s['disks'])} disks, {len(s['net']['ifaces'])} interfaces")
PY
