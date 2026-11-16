"""Controlled registry transitions; no network or publication in these tests."""
import hashlib
from io import BytesIO
import json
import tarfile
import unittest
from urllib.error import HTTPError, URLError
from publish_foundation import publish_or_verify, validate_foundation_manifest

SHA='a'*40
VERSION='0.11.0'


def crate(body=b'canonical'):
    stream=BytesIO()
    with tarfile.open(fileobj=stream,mode='w:gz') as tar:
        for name,data in {'.cargo_vcs_info.json':json.dumps({'git':{'sha1':SHA,'dirty':False}}).encode(),
                          'src/lib.rs':body}.items():
            item=tarfile.TarInfo('taut-foundation-'+VERSION+'/'+name)
            item.size=len(data)
            tar.addfile(item,BytesIO(data))
    return stream.getvalue()


class FoundationDependencyPolicy(unittest.TestCase):
    def test_default_and_opt_in_binding_are_accepted(self):
        validate_foundation_manifest({'features': {'default': []}})
        validate_foundation_manifest({'features': {'default': [], 'num-bigint': ['dep:num-bigint']},
            'dependencies': {'num-bigint': {'version': '0.4', 'optional': True, 'default-features': False}}})

    def test_implicit_or_unrelated_dependencies_are_rejected(self):
        for value in (
            {'features': {'default': ['num-bigint']}},
            {'dependencies': {'other': {'optional': True}}},
            {'dependencies': {'num-bigint': {'optional': False}}},
            {'dependencies': {'num-bigint': {'optional': True, 'default-features': False}}},
        ):
            with self.assertRaises(RuntimeError):
                validate_foundation_manifest(value)


class Registry:
    def __init__(self,published=None,error=None,lost=False):
        self.published,self.error,self.lost=published,error,lost
        self.uploads=0
    def get(self,url):
        if self.error: raise self.error
        if url.endswith('.crate'): return self.published
        if self.published is None: raise HTTPError(url,404,'absent',{},None)
        return json.dumps({'version':{'num':VERSION,'yanked':False,
                         'checksum':hashlib.sha256(self.published).hexdigest()}}).encode()
    def upload(self):
        self.uploads+=1
        self.published=crate()
        if self.lost: raise URLError('upload accepted but response lost')
    def run(self):
        return publish_or_verify(crate(),VERSION,SHA,get=self.get,upload=self.upload)


class PublicationRecovery(unittest.TestCase):
    def test_absent_uploads_once_then_verifies(self):
        registry=Registry()
        self.assertEqual(registry.run()['status'],'published-payload-verified')
        self.assertEqual(registry.uploads,1)
    def test_matching_existing_does_not_upload(self):
        registry=Registry(crate())
        registry.run()
        self.assertEqual(registry.uploads,0)
    def test_accepted_upload_lost_response_recovers_without_reupload(self):
        registry=Registry(lost=True)
        with self.assertRaises(URLError): registry.run()
        registry.run()
        self.assertEqual(registry.uploads,1)
    def test_mismatching_existing_fails_without_upload(self):
        registry=Registry(crate(b'other'))
        with self.assertRaisesRegex(RuntimeError,'differs'): registry.run()
        self.assertEqual(registry.uploads,0)
    def test_lookup_and_network_errors_never_mean_absence(self):
        for error in (HTTPError('url',500,'server error',{},None),URLError('offline')):
            registry=Registry(error=error)
            with self.assertRaises(type(error)): registry.run()
            self.assertEqual(registry.uploads,0)
    def test_source_identity_mismatch_fails(self):
        registry=Registry(crate())
        with self.assertRaisesRegex(RuntimeError,'exact clean'):
            publish_or_verify(crate(),VERSION,'b'*40,get=registry.get,upload=registry.upload)


if __name__=='__main__': unittest.main()
