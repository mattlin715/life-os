"""Read-only verifier for the v15 Cloud handoff. Python standard library only."""
import hashlib, json, sys, zipfile
from pathlib import Path, PurePosixPath

root = Path(__file__).resolve().parent
def sha(data): return hashlib.sha256(data).hexdigest()
def require(ok, msg):
    if not ok: raise ValueError(msg)

expected = {}
for line in (root / "SHA256SUMS").read_text(encoding="utf-8").splitlines():
    digest, rel = line.split("  ", 1)
    p = PurePosixPath(rel)
    require(not p.is_absolute() and ".." not in p.parts and "\\" not in rel, "Unsafe manifest path")
    require(rel not in expected, "Duplicate manifest path")
    require(len(digest)==64 and all(c in "0123456789abcdef" for c in digest), "Invalid hash")
    expected[rel] = digest
actual = {p.relative_to(root).as_posix() for p in root.rglob("*") if p.is_file() and p != root / "SHA256SUMS" and "__pycache__" not in p.parts}
require(actual == set(expected), "Unlisted or missing handoff file: " + str(actual ^ set(expected)))
for rel, digest in expected.items():
    p = root / rel
    require(not p.is_symlink() and p.is_file(), "Not a regular file: " + rel)
    require(sha(p.read_bytes()) == digest, "Hash mismatch: " + rel)

binding = json.loads((root / "SOURCE_BINDING.json").read_text(encoding="utf-8"))
counts = {}
for package in binding["packages"]:
    zp = root / package["archive"]
    require(sha(zp.read_bytes()) == package["sha256"], "Archive hash mismatch")
    with zipfile.ZipFile(zp) as z:
        names = [i.filename for i in z.infolist() if not i.is_dir()]
        require(len(names) == len(set(names)), "Duplicate archive entry")
        for n in names:
            p = PurePosixPath(n)
            require(not p.is_absolute() and ".." not in p.parts and "\\" not in n, "Unsafe archive entry")
            require(z.read(n) == (root / package["expanded"] / n).read_bytes(), "ZIP/expanded mismatch: " + n)
        inner = z.read("SHA256SUMS").decode("utf-8-sig").splitlines()
        count = 0
        for line in inner:
            if not line.strip(): continue
            digest, name = line.split(None, 1)
            name = name.strip().lstrip("*")
            require(sha(z.read(name)) == digest.lower(), "Inner hash mismatch: " + name)
            count += 1
        counts[package["archive"]] = {"zip_files": len(names), "inner_hashes": count}
proto = root / binding["prototype"]["path"]
require(sha(proto.read_bytes()) == binding["prototype"]["sha256"], "Prototype mismatch")
print(json.dumps({"result":"PASS","manifest_files":len(expected),"original_packages":counts,
                  "prototype_sha256":sha(proto.read_bytes()),
                  "scope":"handoff integrity only; no product tests or browser walkthrough"},ensure_ascii=False,indent=2))
