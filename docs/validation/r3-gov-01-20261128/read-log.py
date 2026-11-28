"""Decode a lossless verification-output envelope to stdout (stdlib only)."""
import hashlib,json,sys
from pathlib import Path
value=json.loads(Path(sys.argv[1]).read_text('utf-8'))
assert value['format']=='life-os/verification-output-v1' and value['encoding']=='utf-8'
raw=''.join(value['utf8_lines']).encode('utf-8')
assert len(raw)==value['byte_length'] and hashlib.sha256(raw).hexdigest()==value['sha256']
sys.stdout.buffer.write(raw)
