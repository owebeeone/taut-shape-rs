"""Publish or reconcile the exact canonical foundation archive. CI only."""
import argparse
import hashlib
import io
import json
import os
from pathlib import Path
import re
import subprocess
import tarfile
from urllib.error import HTTPError
from urllib.request import Request, urlopen


def fetch(url):
    with urlopen(Request(url, headers={'User-Agent':'taut-foundation-release/1.0'}), timeout=60) as response:
        data = response.read(16*1024*1024+1)
        if len(data)>16*1024*1024:
            raise RuntimeError('registry response exceeds the foundation package bound')
        return data


def payload(data, expected=None):
    files = {}
    with tarfile.open(fileobj=io.BytesIO(data), mode='r:gz') as archive:
        for member in archive:
            if not member.isfile() or member.name in files:
                raise RuntimeError('unexpected or duplicate crate archive entry')
            if expected is not None and (member.name not in expected or member.size != len(expected[member.name])):
                raise RuntimeError('published foundation differs from the reviewed package payload')
            files[member.name] = archive.extractfile(member).read()
    return files


def publish_or_verify(archive, version, source_sha, *, get=fetch, upload=None):
    if not re.fullmatch(r'\d+\.\d+\.\d+',version) or not re.fullmatch(r'[0-9a-f]{40}',source_sha):
        raise RuntimeError('explicit release version and exact source SHA are required')
    expected = payload(archive)
    prefix = 'taut-foundation-'+version+'/'
    vcs = json.loads(expected[prefix+'.cargo_vcs_info.json'])
    if vcs['git']['sha1'] != source_sha or vcs['git'].get('dirty',False):
        raise RuntimeError('local package is not the exact clean reviewed source')
    url = 'https://crates.io/api/v1/crates/taut-foundation/'+version
    try:
        metadata = json.loads(get(url))['version']
    except HTTPError as error:
        if error.code != 404:
            raise
        if upload is None:
            raise RuntimeError('absent foundation requires the CI publication operation')
        upload()
        # A same-input rerun reconciles an upload whose response was lost.
        # Lookup errors are failures, never evidence of version absence.
        metadata = json.loads(get(url))['version']
    if metadata['num'] != version or metadata.get('yanked'):
        raise RuntimeError('registry version identity is unavailable or yanked')
    published = get('https://static.crates.io/crates/taut-foundation/taut-foundation-'+version+'.crate')
    checksum = hashlib.sha256(published).hexdigest()
    if checksum != metadata['checksum'] or payload(published,expected) != expected:
        raise RuntimeError('published foundation differs from the reviewed package payload')
    return {'version':version,'source_sha':source_sha,'registry_checksum':checksum,
            'local_archive_checksum':hashlib.sha256(archive).hexdigest(),
            'payload_files':len(expected),'status':'published-payload-verified'}


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--version',required=True)
    parser.add_argument('--source-sha',required=True)
    args=parser.parse_args()
    manifest=Path('rust/taut-foundation/Cargo.toml')
    archive=manifest.parent/'target/package'/('taut-foundation-'+args.version+'.crate')
    def upload():
        if not os.environ.get('CARGO_REGISTRY_TOKEN'):
            raise RuntimeError('foundation bootstrap credential is unavailable')
        subprocess.run(['cargo','publish','--manifest-path',str(manifest)],check=True)
    result=publish_or_verify(archive.read_bytes(),args.version,args.source_sha,upload=upload)
    Path('foundation-publication.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
    print(json.dumps(result,sort_keys=True))


if __name__=='__main__':
    main()
