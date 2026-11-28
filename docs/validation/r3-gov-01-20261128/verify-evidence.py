"""Read-only source/evidence/protected-file integrity verification."""
import hashlib,json
from pathlib import Path
folder=Path(__file__).resolve().parent
root=folder.parents[2]
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
for path,hash_ in json.loads((folder/'SOURCE_HASHES.json').read_text()).items():assert sha(root/path)==hash_,path
before=json.loads((folder/'protected-before.json').read_text());after=json.loads((folder/'protected-after.json').read_text())
assert before==after
for path,hash_ in before.items():assert sha(root/path)==hash_,path
for file in folder.rglob('*.log.json'):
 v=json.loads(file.read_text());raw=''.join(v['utf8_lines']).encode('utf-8')
 assert len(raw)==v['byte_length'] and hashlib.sha256(raw).hexdigest()==v['sha256'],str(file)
for line in (folder/'SHA256SUMS').read_text().splitlines():
 hash_,path=line.split('  ',1);assert sha(root/path)==hash_,path
print(json.dumps({'result':'PASS','source_files':6,'protected_unchanged':len(before),'log_envelopes':'losslessly verified'}))
