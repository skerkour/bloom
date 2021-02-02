 # -*- coding: utf-8 -*-
import subprocess
import sys
import json
import hashlib
import time
import base64
from binascii import hexlify
from collections import namedtuple

HEADER = """//!
//! This library is automatically generated from Google's list of known CT
//! logs.  Don't edit it.
//!
//! The generation is done deterministically so you can verify it
//! yourself by inspecting and re-running the generation process.
//!

#![forbid(unsafe_code,
          unstable_features)]
#![deny(trivial_casts,
        trivial_numeric_casts,
        unused_import_braces,
        unused_extern_crates,
        unused_qualifications)]

pub static LOGS: [&sct::Log; %d] = ["""

FOOTER = """];"""

Log = namedtuple('Log', 'name url mmd operator key keyid json'.split())

LOG_LIST = 'https://www.gstatic.com/ct/log_list/log_list.json'
LOG_LIST_SIG = 'https://www.gstatic.com/ct/log_list/log_list.sig'

def fetch_and_check_sig():
    for cmd in (['curl', '-o', 'log_list.sig', LOG_LIST_SIG],
                ['curl', '-o', 'log_list.json', LOG_LIST],
                ['openssl', 'dgst', '-sha256', '-verify', 
                 'log_list_pubkey.pem', '-signature', 'log_list.sig', 'log_list.json'],
                ):
        subprocess.check_call(cmd, stdout = subprocess.PIPE)
    return json.load(open('log_list.json'))

def convert_json(json):
    operators = { v['id']: v['name'] for v in json['operators'] }

    for lj in json['logs']:
        operator = ', '.join(operators[op] for op in lj['operated_by'])
        key = base64.b64decode(lj['key'])
        keyid = hashlib.sha256(key).digest()

        disqualification = lj.get('disqualified_at', None)
        if disqualification and time.time() > disqualification:
            continue

        log = Log(lj['description'],
                lj['url'],
                lj['maximum_merge_delay'],
                operator,
                key,
                keyid,
                lj)
        yield log

def commentify(cert):
    lines = cert.splitlines()
    lines = [ll[2:] if ll.startswith('# ') else ll for ll in lines]
    return '/*\n     * ' + ('\n     * '.join(lines)) + '\n     */'

def convert_bytes(bb):
    return ''.join('\\x{:02x}'.format(b) for b in bb)

def raw_public_key(spki):
    def take_byte(b):
        return b[0], b[1:]

    def take_len(b):
        v, b = take_byte(b)

        if v & 0x80:
            r = 0
            for _ in range(v & 3):
                x, b = take_byte(b)
                r <<= 8
                r |= x
            return r, b

        return v, b

    def take_seq(b):
        tag, b = take_byte(b)
        ll, b = take_len(b)
        assert tag == 0x30
        return b[:ll], b[ll:]

    def take_bitstring(b):
        tag, b = take_byte(b)
        ll, b = take_len(b)
        bits, b = take_byte(b)
        assert tag == 0x03
        assert bits == 0
        return b[:ll-1], b[ll-1:]

    open('key.bin', 'wb').write(spki)
    spki, rest = take_seq(spki)
    assert len(rest) == 0
    id, data = take_seq(spki)
    keydata, rest = take_bitstring(data)
    assert len(rest) == 0
    return keydata

def print_log(log):
    comment = commentify(
        json.dumps(log.json,
            indent = 2,
            separators = (',', ': '),
            sort_keys = True)
        )

    id_up = hexlify(log.key).upper()[:16]
    description = log.name
    url = log.url
    operator = log.operator
    key = convert_bytes(raw_public_key(log.key))
    keyid_hex = ', '.join('0x{:02x}'.format(x) for x in log.keyid)
    mmd = log.mmd

    print("""    %(comment)s
    &sct::Log {
        description: "%(description)s",
        url: "%(url)s",
        operated_by: "%(operator)s",
        key: b"%(key)s",
        id: [ %(keyid_hex)s ],
        max_merge_delay: %(mmd)d,
    },
""" % locals())

if __name__ == '__main__':
    if sys.platform == "win32":
        import os, msvcrt
        msvcrt.setmode(sys.stdout.fileno(), os.O_BINARY)

    data = fetch_and_check_sig()

    logs = {}
    for log in convert_json(data):
        logs[hexlify(log.keyid)] = log

    print(HEADER % len(list(logs.keys())))
    for id in sorted(logs.keys()):
        print_log(logs[id])
    print(FOOTER)
