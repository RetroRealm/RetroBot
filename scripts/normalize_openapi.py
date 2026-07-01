"""Normalize the playmatch OpenAPI spec so progenitor 0.14 can generate a client.

Two spec patterns break progenitor:
1. Operations whose error responses carry more than one distinct payload type
   (e.g. a typed 400 next to bare 401/403). Progenitor requires at most one
   error payload type per operation, so the payload declarations are dropped;
   the status codes remain.
2. Object-typed query parameters (SgdbAssetFilterQuery `filters`). The server
   parses these as flat query params, so they are flattened back into
   individual parameters exactly as the v1 spec declared them.
"""
import json
import sys

src, dst = sys.argv[1], sys.argv[2]
spec = json.load(open(src, encoding="utf-8"))
schemas = spec["components"]["schemas"]
changed = []

for path, item in spec["paths"].items():
    for method, op in item.items():
        if method.startswith("x-") or method == "parameters":
            continue

        errors = {
            code: resp
            for code, resp in op.get("responses", {}).items()
            if not code.startswith("2")
        }
        typed = {
            code: json.dumps(resp.get("content"), sort_keys=True)
            for code, resp in errors.items()
            if resp.get("content")
        }
        untyped = [c for c, r in errors.items() if not r.get("content")]
        if len(set(typed.values())) > 1 or (typed and untyped):
            for code in typed:
                del op["responses"][code]["content"]
                changed.append(f"error content: {method.upper()} {path} {code}")

        params = op.get("parameters", [])
        flattened = []
        for p in params:
            ref = p.get("schema", {}).get("$ref", "")
            if p.get("in") == "query" and ref:
                target = schemas[ref.rsplit("/", 1)[1]]
                if target.get("type") == "object":
                    for name, prop in sorted(target["properties"].items()):
                        flattened.append(
                            {"name": name, "in": "query", "required": False, "schema": prop}
                        )
                    changed.append(f"flattened param: {method.upper()} {path} {p['name']}")
                    continue
            flattened.append(p)
        if params:
            op["parameters"] = flattened

json.dump(spec, open(dst, "w", encoding="utf-8"), separators=(",", ":"))
for c in changed:
    print(" ", c)
