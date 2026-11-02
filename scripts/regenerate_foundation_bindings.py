"""Regenerate mandatory metadata from native schemas and alias one foundation.

Development only: PYTHONPATH=../taut/src python scripts/regenerate_foundation_bindings.py
Normal application builds use no Python, compiler, authoring loader or generator.
"""
import hashlib
import json
from pathlib import Path
from taut.gen.scaffold import rust_api
from taut.ir.load import load_schema

ROOT=Path(__file__).resolve().parents[1]
SCHEMAS=ROOT.parent/'taut-shape/ir'
DEST=ROOT/'crates/taut-shape/src'
provenance={'generator_build':'rust-oneof-reference.1','descriptor_contract':1,
            'codec_contract':1,'runtime_owner':'taut.rust.foundation','sources':{}}
for name in ('log','atom','stream','value','swmr','crdt'):
    source=SCHEMAS/f'shape_{name}.taut.py'
    schema=load_schema(source)
    text=rust_api(schema)
    text=text.replace('#![allow(dead_code,', '#![cfg_attr(rustfmt, rustfmt::skip)]\n#![allow(dead_code,',1)
    text=f'// Source: taut-shape/ir/{source.name}; regenerate with scripts/regenerate_foundation_bindings.py\n'+text
    filename='generated.rs' if name=='log' else f'generated_{name}.rs'
    (DEST/filename).write_text(text,encoding='utf-8',newline='\n')
    provenance['sources'][filename]={'schema':source.name,'schema_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
                                     'generated_sha256':hashlib.sha256(text.encode()).hexdigest()}
(DEST/'cbor.rs').write_text('''//! Compatibility facade: the actual runtime is maintained once in base taut.
// Source : taut/src/taut/gen/runtime/cbor_fail_closed.rs at taut v0.10.0.
// Regenerate the foundation crate with taut/rust/sync_foundation.py.
// This re-export preserves Cbor, DecodeError and descriptor nominal identity.
pub use taut_foundation::cbor::*;
''',encoding='utf-8',newline='\n')
(DEST/'foundation_provenance.json').write_text(json.dumps(provenance,indent=2,sort_keys=True)+'\n',encoding='utf-8',newline='\n')
print('six native Rust shape schemas regenerated against one foundation owner')
