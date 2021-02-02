//!
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

pub static LOGS: [&sct::Log; 32] = [
    /*
     * {
     *   "description": "Venafi Gen2 CT log",
     *   "dns_api_endpoint": "venafi2.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEjicnerZVCXTrbEuUhGW85BXx6lrYfA43zro/bAna5ymW00VQb94etBzSg4j/KS/Oqf/fNN51D8DMGA2ULvw3AQ==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     6
     *   ],
     *   "url": "ctlog-gen2.api.venafi.com/"
     * }
     */
    &sct::Log {
        description: "Venafi Gen2 CT log",
        url: "ctlog-gen2.api.venafi.com/",
        operated_by: "Venafi",
        key: b"\x04\x8e\x27\x27\x7a\xb6\x55\x09\x74\xeb\x6c\x4b\x94\x84\x65\xbc\xe4\x15\xf1\xea\x5a\xd8\x7c\x0e\x37\xce\xba\x3f\x6c\x09\xda\xe7\x29\x96\xd3\x45\x50\x6f\xde\x1e\xb4\x1c\xd2\x83\x88\xff\x29\x2f\xce\xa9\xff\xdf\x34\xde\x75\x0f\xc0\xcc\x18\x0d\x94\x2e\xfc\x37\x01",
        id: [ 0x03, 0x01, 0x9d, 0xf3, 0xfd, 0x85, 0xa6, 0x9a, 0x8e, 0xbd, 0x1f, 0xac, 0xc6, 0xda, 0x9b, 0xa7, 0x3e, 0x46, 0x97, 0x74, 0xfe, 0x77, 0xf5, 0x79, 0xfc, 0x5a, 0x08, 0xb8, 0x32, 0x8c, 0x1d, 0x6b ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Google 'Xenon2020' log",
     *   "dns_api_endpoint": "xenon2020.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEZU75VqjyzSTgFZKAnWg1QeYfFFIRZTMK7q3kWWZsmHhQdrBYnHRZ3OA4kUeUx0JN+xX+dSgt1ruqUhhl7jOvmw==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     0
     *   ],
     *   "url": "ct.googleapis.com/logs/xenon2020/"
     * }
     */
    &sct::Log {
        description: "Google 'Xenon2020' log",
        url: "ct.googleapis.com/logs/xenon2020/",
        operated_by: "Google",
        key: b"\x04\x65\x4e\xf9\x56\xa8\xf2\xcd\x24\xe0\x15\x92\x80\x9d\x68\x35\x41\xe6\x1f\x14\x52\x11\x65\x33\x0a\xee\xad\xe4\x59\x66\x6c\x98\x78\x50\x76\xb0\x58\x9c\x74\x59\xdc\xe0\x38\x91\x47\x94\xc7\x42\x4d\xfb\x15\xfe\x75\x28\x2d\xd6\xbb\xaa\x52\x18\x65\xee\x33\xaf\x9b",
        id: [ 0x07, 0xb7, 0x5c, 0x1b, 0xe5, 0x7d, 0x68, 0xff, 0xf1, 0xb0, 0xc6, 0x1d, 0x23, 0x15, 0xc7, 0xba, 0xe6, 0x57, 0x7c, 0x57, 0x94, 0xb7, 0x6a, 0xee, 0xbc, 0x61, 0x3a, 0x1a, 0x69, 0xd3, 0xa2, 0x1c ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Google 'Xenon2019' log",
     *   "dns_api_endpoint": "xenon2019.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE/XyDwqzXL9i2GTjMYkqaEyiRL0Dy9sHq/BTebFdshbvCaXXEh6mjUK0Yy+AsDcI4MpzF1l7Kded2MD5zi420gA==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     0
     *   ],
     *   "url": "ct.googleapis.com/logs/xenon2019/"
     * }
     */
    &sct::Log {
        description: "Google 'Xenon2019' log",
        url: "ct.googleapis.com/logs/xenon2019/",
        operated_by: "Google",
        key: b"\x04\xfd\x7c\x83\xc2\xac\xd7\x2f\xd8\xb6\x19\x38\xcc\x62\x4a\x9a\x13\x28\x91\x2f\x40\xf2\xf6\xc1\xea\xfc\x14\xde\x6c\x57\x6c\x85\xbb\xc2\x69\x75\xc4\x87\xa9\xa3\x50\xad\x18\xcb\xe0\x2c\x0d\xc2\x38\x32\x9c\xc5\xd6\x5e\xca\x75\xe7\x76\x30\x3e\x73\x8b\x8d\xb4\x80",
        id: [ 0x08, 0x41, 0x14, 0x98, 0x00, 0x71, 0x53, 0x2c, 0x16, 0x19, 0x04, 0x60, 0xbc, 0xfc, 0x47, 0xfd, 0xc2, 0x65, 0x3a, 0xfa, 0x29, 0x2c, 0x72, 0xb3, 0x7f, 0xf8, 0x63, 0xae, 0x29, 0xcc, 0xc9, 0xf0 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "DigiCert Yeti2022 Log",
     *   "dns_api_endpoint": "digicert-yeti2022.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEn/jYHd77W1G1+131td5mEbCdX/1v/KiYW5hPLcOROvv+xA8Nw2BDjB7y+RGyutD2vKXStp/5XIeiffzUfdYTJg==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     2
     *   ],
     *   "url": "yeti2022.ct.digicert.com/log/"
     * }
     */
    &sct::Log {
        description: "DigiCert Yeti2022 Log",
        url: "yeti2022.ct.digicert.com/log/",
        operated_by: "DigiCert",
        key: b"\x04\x9f\xf8\xd8\x1d\xde\xfb\x5b\x51\xb5\xfb\x5d\xf5\xb5\xde\x66\x11\xb0\x9d\x5f\xfd\x6f\xfc\xa8\x98\x5b\x98\x4f\x2d\xc3\x91\x3a\xfb\xfe\xc4\x0f\x0d\xc3\x60\x43\x8c\x1e\xf2\xf9\x11\xb2\xba\xd0\xf6\xbc\xa5\xd2\xb6\x9f\xf9\x5c\x87\xa2\x7d\xfc\xd4\x7d\xd6\x13\x26",
        id: [ 0x22, 0x45, 0x45, 0x07, 0x59, 0x55, 0x24, 0x56, 0x96, 0x3f, 0xa1, 0x2f, 0xf1, 0xf7, 0x6d, 0x86, 0xe0, 0x23, 0x26, 0x63, 0xad, 0xc0, 0x4b, 0x7f, 0x5d, 0xc6, 0x83, 0x5c, 0x6e, 0xe2, 0x0f, 0x02 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Google 'Icarus' log",
     *   "dns_api_endpoint": "icarus.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAETtK8v7MICve56qTHHDhhBOuV4IlUaESxZryCfk9QbG9co/CqPvTsgPDbCpp6oFtyAHwlDhnvr7JijXRD9Cb2FA==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     0
     *   ],
     *   "url": "ct.googleapis.com/icarus/"
     * }
     */
    &sct::Log {
        description: "Google 'Icarus' log",
        url: "ct.googleapis.com/icarus/",
        operated_by: "Google",
        key: b"\x04\x4e\xd2\xbc\xbf\xb3\x08\x0a\xf7\xb9\xea\xa4\xc7\x1c\x38\x61\x04\xeb\x95\xe0\x89\x54\x68\x44\xb1\x66\xbc\x82\x7e\x4f\x50\x6c\x6f\x5c\xa3\xf0\xaa\x3e\xf4\xec\x80\xf0\xdb\x0a\x9a\x7a\xa0\x5b\x72\x00\x7c\x25\x0e\x19\xef\xaf\xb2\x62\x8d\x74\x43\xf4\x26\xf6\x14",
        id: [ 0x29, 0x3c, 0x51, 0x96, 0x54, 0xc8, 0x39, 0x65, 0xba, 0xaa, 0x50, 0xfc, 0x58, 0x07, 0xd4, 0xb7, 0x6f, 0xbf, 0x58, 0x7a, 0x29, 0x72, 0xdc, 0xa4, 0xc3, 0x0c, 0xf4, 0xe5, 0x45, 0x47, 0xf4, 0x78 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "DigiCert Yeti2023 Log",
     *   "dns_api_endpoint": "digicert-yeti2023.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEfQ0DsdWYitzwFTvG3F4Nbj8Nv5XIVYzQpkyWsU4nuSYlmcwrAp6m092fsdXEw6w1BAeHlzaqrSgNfyvZaJ9y0Q==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     2
     *   ],
     *   "url": "yeti2023.ct.digicert.com/log/"
     * }
     */
    &sct::Log {
        description: "DigiCert Yeti2023 Log",
        url: "yeti2023.ct.digicert.com/log/",
        operated_by: "DigiCert",
        key: b"\x04\x7d\x0d\x03\xb1\xd5\x98\x8a\xdc\xf0\x15\x3b\xc6\xdc\x5e\x0d\x6e\x3f\x0d\xbf\x95\xc8\x55\x8c\xd0\xa6\x4c\x96\xb1\x4e\x27\xb9\x26\x25\x99\xcc\x2b\x02\x9e\xa6\xd3\xdd\x9f\xb1\xd5\xc4\xc3\xac\x35\x04\x07\x87\x97\x36\xaa\xad\x28\x0d\x7f\x2b\xd9\x68\x9f\x72\xd1",
        id: [ 0x35, 0xcf, 0x19, 0x1b, 0xbf, 0xb1, 0x6c, 0x57, 0xbf, 0x0f, 0xad, 0x4c, 0x6d, 0x42, 0xcb, 0xbb, 0xb6, 0x27, 0x20, 0x26, 0x51, 0xea, 0x3f, 0xe1, 0x2a, 0xef, 0xa8, 0x03, 0xc3, 0x3b, 0xd6, 0x4c ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Cloudflare 'Nimbus2022' Log",
     *   "dns_api_endpoint": "cloudflare-nimbus2022.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAESLJHTlAycmJKDQxIv60pZG8g33lSYxYpCi5gteI6HLevWbFVCdtZx+m9b+0LrwWWl/87mkNN6xE0M4rnrIPA/w==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     1
     *   ],
     *   "url": "ct.cloudflare.com/logs/nimbus2022/"
     * }
     */
    &sct::Log {
        description: "Cloudflare 'Nimbus2022' Log",
        url: "ct.cloudflare.com/logs/nimbus2022/",
        operated_by: "Cloudflare",
        key: b"\x04\x48\xb2\x47\x4e\x50\x32\x72\x62\x4a\x0d\x0c\x48\xbf\xad\x29\x64\x6f\x20\xdf\x79\x52\x63\x16\x29\x0a\x2e\x60\xb5\xe2\x3a\x1c\xb7\xaf\x59\xb1\x55\x09\xdb\x59\xc7\xe9\xbd\x6f\xed\x0b\xaf\x05\x96\x97\xff\x3b\x9a\x43\x4d\xeb\x11\x34\x33\x8a\xe7\xac\x83\xc0\xff",
        id: [ 0x41, 0xc8, 0xca, 0xb1, 0xdf, 0x22, 0x46, 0x4a, 0x10, 0xc6, 0xa1, 0x3a, 0x09, 0x42, 0x87, 0x5e, 0x4e, 0x31, 0x8b, 0x1b, 0x03, 0xeb, 0xeb, 0x4b, 0xc7, 0x68, 0xf0, 0x90, 0x62, 0x96, 0x06, 0xf6 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Cloudflare 'Nimbus2021' Log",
     *   "dns_api_endpoint": "cloudflare-nimbus2021.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExpon7ipsqehIeU1bmpog9TFo4Pk8+9oN8OYHl1Q2JGVXnkVFnuuvPgSo2Ep+6vLffNLcmEbxOucz03sFiematg==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     1
     *   ],
     *   "url": "ct.cloudflare.com/logs/nimbus2021/"
     * }
     */
    &sct::Log {
        description: "Cloudflare 'Nimbus2021' Log",
        url: "ct.cloudflare.com/logs/nimbus2021/",
        operated_by: "Cloudflare",
        key: b"\x04\xc6\x9a\x27\xee\x2a\x6c\xa9\xe8\x48\x79\x4d\x5b\x9a\x9a\x20\xf5\x31\x68\xe0\xf9\x3c\xfb\xda\x0d\xf0\xe6\x07\x97\x54\x36\x24\x65\x57\x9e\x45\x45\x9e\xeb\xaf\x3e\x04\xa8\xd8\x4a\x7e\xea\xf2\xdf\x7c\xd2\xdc\x98\x46\xf1\x3a\xe7\x33\xd3\x7b\x05\x89\xe9\x9a\xb6",
        id: [ 0x44, 0x94, 0x65, 0x2e, 0xb0, 0xee, 0xce, 0xaf, 0xc4, 0x40, 0x07, 0xd8, 0xa8, 0xfe, 0x28, 0xc0, 0xda, 0xe6, 0x82, 0xbe, 0xd8, 0xcb, 0x31, 0xb5, 0x3f, 0xd3, 0x33, 0x96, 0xb5, 0xb6, 0x81, 0xa8 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Google 'Xenon2022' log",
     *   "dns_api_endpoint": "xenon2022.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE+WS9FSxAYlCVEzg8xyGwOrmPonoV14nWjjETAIdZvLvukPzIWBMKv6tDNlQjpIHNrUcUt1igRPpqoKDXw2MeKw==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     0
     *   ],
     *   "url": "ct.googleapis.com/logs/xenon2022/"
     * }
     */
    &sct::Log {
        description: "Google 'Xenon2022' log",
        url: "ct.googleapis.com/logs/xenon2022/",
        operated_by: "Google",
        key: b"\x04\xf9\x64\xbd\x15\x2c\x40\x62\x50\x95\x13\x38\x3c\xc7\x21\xb0\x3a\xb9\x8f\xa2\x7a\x15\xd7\x89\xd6\x8e\x31\x13\x00\x87\x59\xbc\xbb\xee\x90\xfc\xc8\x58\x13\x0a\xbf\xab\x43\x36\x54\x23\xa4\x81\xcd\xad\x47\x14\xb7\x58\xa0\x44\xfa\x6a\xa0\xa0\xd7\xc3\x63\x1e\x2b",
        id: [ 0x46, 0xa5, 0x55, 0xeb, 0x75, 0xfa, 0x91, 0x20, 0x30, 0xb5, 0xa2, 0x89, 0x69, 0xf4, 0xf3, 0x7d, 0x11, 0x2c, 0x41, 0x74, 0xbe, 0xfd, 0x49, 0xb8, 0x85, 0xab, 0xf2, 0xfc, 0x70, 0xfe, 0x6d, 0x47 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "DigiCert Nessie2022 Log",
     *   "dns_api_endpoint": "digicert-nessie2022.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEJyTdaAMoy/5jvg4RR019F2ihEV1McclBKMe2okuX7MCv/C87v+nxsfz1Af+p+0lADGMkmNd5LqZVqxbGvlHYcQ==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     2
     *   ],
     *   "url": "nessie2022.ct.digicert.com/log/"
     * }
     */
    &sct::Log {
        description: "DigiCert Nessie2022 Log",
        url: "nessie2022.ct.digicert.com/log/",
        operated_by: "DigiCert",
        key: b"\x04\x27\x24\xdd\x68\x03\x28\xcb\xfe\x63\xbe\x0e\x11\x47\x4d\x7d\x17\x68\xa1\x11\x5d\x4c\x71\xc9\x41\x28\xc7\xb6\xa2\x4b\x97\xec\xc0\xaf\xfc\x2f\x3b\xbf\xe9\xf1\xb1\xfc\xf5\x01\xff\xa9\xfb\x49\x40\x0c\x63\x24\x98\xd7\x79\x2e\xa6\x55\xab\x16\xc6\xbe\x51\xd8\x71",
        id: [ 0x51, 0xa3, 0xb0, 0xf5, 0xfd, 0x01, 0x79, 0x9c, 0x56, 0x6d, 0xb8, 0x37, 0x78, 0x8f, 0x0c, 0xa4, 0x7a, 0xcc, 0x1b, 0x27, 0xcb, 0xf7, 0x9e, 0x88, 0x42, 0x9a, 0x0d, 0xfe, 0xd4, 0x8b, 0x05, 0xe5 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Sectigo 'Sabre' CT log",
     *   "dns_api_endpoint": "comodo-sabre.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE8m/SiQ8/xfiHHqtls9m7FyOMBg4JVZY9CgiixXGz0akvKD6DEL8S0ERmFe9U4ZiA0M4kbT5nmuk3I85Sk4bagA==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     9
     *   ],
     *   "url": "sabre.ct.comodo.com/"
     * }
     */
    &sct::Log {
        description: "Sectigo 'Sabre' CT log",
        url: "sabre.ct.comodo.com/",
        operated_by: "Sectigo",
        key: b"\x04\xf2\x6f\xd2\x89\x0f\x3f\xc5\xf8\x87\x1e\xab\x65\xb3\xd9\xbb\x17\x23\x8c\x06\x0e\x09\x55\x96\x3d\x0a\x08\xa2\xc5\x71\xb3\xd1\xa9\x2f\x28\x3e\x83\x10\xbf\x12\xd0\x44\x66\x15\xef\x54\xe1\x98\x80\xd0\xce\x24\x6d\x3e\x67\x9a\xe9\x37\x23\xce\x52\x93\x86\xda\x80",
        id: [ 0x55, 0x81, 0xd4, 0xc2, 0x16, 0x90, 0x36, 0x01, 0x4a, 0xea, 0x0b, 0x9b, 0x57, 0x3c, 0x53, 0xf0, 0xc0, 0xe4, 0x38, 0x78, 0x70, 0x25, 0x08, 0x17, 0x2f, 0xa3, 0xaa, 0x1d, 0x07, 0x13, 0xd3, 0x0c ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "DigiCert Log Server",
     *   "dns_api_endpoint": "digicert.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEAkbFvhu7gkAW6MHSrBlpE1n4+HCFRkC5OLAjgqhkTH+/uzSfSl8ois8ZxAD2NgaTZe1M9akhYlrYkes4JECs6A==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     2
     *   ],
     *   "url": "ct1.digicert-ct.com/log/"
     * }
     */
    &sct::Log {
        description: "DigiCert Log Server",
        url: "ct1.digicert-ct.com/log/",
        operated_by: "DigiCert",
        key: b"\x04\x02\x46\xc5\xbe\x1b\xbb\x82\x40\x16\xe8\xc1\xd2\xac\x19\x69\x13\x59\xf8\xf8\x70\x85\x46\x40\xb9\x38\xb0\x23\x82\xa8\x64\x4c\x7f\xbf\xbb\x34\x9f\x4a\x5f\x28\x8a\xcf\x19\xc4\x00\xf6\x36\x06\x93\x65\xed\x4c\xf5\xa9\x21\x62\x5a\xd8\x91\xeb\x38\x24\x40\xac\xe8",
        id: [ 0x56, 0x14, 0x06, 0x9a, 0x2f, 0xd7, 0xc2, 0xec, 0xd3, 0xf5, 0xe1, 0xbd, 0x44, 0xb2, 0x3e, 0xc7, 0x46, 0x76, 0xb9, 0xbc, 0x99, 0x11, 0x5c, 0xc0, 0xef, 0x94, 0x98, 0x55, 0xd6, 0x89, 0xd0, 0xdd ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "DigiCert Yeti2021 Log",
     *   "dns_api_endpoint": "digicert-yeti2021.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE6J4EbcpIAl1+AkSRsbhoY5oRTj3VoFfaf1DlQkfi7Rbe/HcjfVtrwN8jaC+tQDGjF+dqvKhWJAQ6Q6ev6q9Mew==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     2
     *   ],
     *   "url": "yeti2021.ct.digicert.com/log/"
     * }
     */
    &sct::Log {
        description: "DigiCert Yeti2021 Log",
        url: "yeti2021.ct.digicert.com/log/",
        operated_by: "DigiCert",
        key: b"\x04\xe8\x9e\x04\x6d\xca\x48\x02\x5d\x7e\x02\x44\x91\xb1\xb8\x68\x63\x9a\x11\x4e\x3d\xd5\xa0\x57\xda\x7f\x50\xe5\x42\x47\xe2\xed\x16\xde\xfc\x77\x23\x7d\x5b\x6b\xc0\xdf\x23\x68\x2f\xad\x40\x31\xa3\x17\xe7\x6a\xbc\xa8\x56\x24\x04\x3a\x43\xa7\xaf\xea\xaf\x4c\x7b",
        id: [ 0x5c, 0xdc, 0x43, 0x92, 0xfe, 0xe6, 0xab, 0x45, 0x44, 0xb1, 0x5e, 0x9a, 0xd4, 0x56, 0xe6, 0x10, 0x37, 0xfb, 0xd5, 0xfa, 0x47, 0xdc, 0xa1, 0x73, 0x94, 0xb2, 0x5e, 0xe6, 0xf6, 0xc7, 0x0e, 0xca ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Cloudflare 'Nimbus2020' Log",
     *   "dns_api_endpoint": "cloudflare-nimbus2020.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE01EAhx4o0zPQrXTcYjgCt4MVFsT0Pwjzb1RwrM0lhWDlxAYPP6/gyMCXNkOn/7KFsjL7rwk78tHMpY8rXn8AYg==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     1
     *   ],
     *   "url": "ct.cloudflare.com/logs/nimbus2020/"
     * }
     */
    &sct::Log {
        description: "Cloudflare 'Nimbus2020' Log",
        url: "ct.cloudflare.com/logs/nimbus2020/",
        operated_by: "Cloudflare",
        key: b"\x04\xd3\x51\x00\x87\x1e\x28\xd3\x33\xd0\xad\x74\xdc\x62\x38\x02\xb7\x83\x15\x16\xc4\xf4\x3f\x08\xf3\x6f\x54\x70\xac\xcd\x25\x85\x60\xe5\xc4\x06\x0f\x3f\xaf\xe0\xc8\xc0\x97\x36\x43\xa7\xff\xb2\x85\xb2\x32\xfb\xaf\x09\x3b\xf2\xd1\xcc\xa5\x8f\x2b\x5e\x7f\x00\x62",
        id: [ 0x5e, 0xa7, 0x73, 0xf9, 0xdf, 0x56, 0xc0, 0xe7, 0xb5, 0x36, 0x48, 0x7d, 0xd0, 0x49, 0xe0, 0x32, 0x7a, 0x91, 0x9a, 0x0c, 0x84, 0xa1, 0x12, 0x12, 0x84, 0x18, 0x75, 0x96, 0x81, 0x71, 0x45, 0x58 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Google 'Argon2019' log",
     *   "dns_api_endpoint": "argon2019.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEI3MQm+HzXvaYa2mVlhB4zknbtAT8cSxakmBoJcBKGqGwYS0bhxSpuvABM1kdBTDpQhXnVdcq+LSiukXJRpGHVg==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     0
     *   ],
     *   "url": "ct.googleapis.com/logs/argon2019/"
     * }
     */
    &sct::Log {
        description: "Google 'Argon2019' log",
        url: "ct.googleapis.com/logs/argon2019/",
        operated_by: "Google",
        key: b"\x04\x23\x73\x10\x9b\xe1\xf3\x5e\xf6\x98\x6b\x69\x95\x96\x10\x78\xce\x49\xdb\xb4\x04\xfc\x71\x2c\x5a\x92\x60\x68\x25\xc0\x4a\x1a\xa1\xb0\x61\x2d\x1b\x87\x14\xa9\xba\xf0\x01\x33\x59\x1d\x05\x30\xe9\x42\x15\xe7\x55\xd7\x2a\xf8\xb4\xa2\xba\x45\xc9\x46\x91\x87\x56",
        id: [ 0x63, 0xf2, 0xdb, 0xcd, 0xe8, 0x3b, 0xcc, 0x2c, 0xcf, 0x0b, 0x72, 0x84, 0x27, 0x57, 0x6b, 0x33, 0xa4, 0x8d, 0x61, 0x77, 0x8f, 0xbd, 0x75, 0xa6, 0x38, 0xb1, 0xc7, 0x68, 0x54, 0x4b, 0xd8, 0x8d ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Google 'Aviator' log",
     *   "dns_api_endpoint": "aviator.ct.googleapis.com",
     *   "final_sth": {
     *     "sha256_root_hash": "LcGcZRsm+LGYmrlyC5LXhV1T6OD8iH5dNlb0sEJl9bA=",
     *     "timestamp": 1480512258330,
     *     "tree_head_signature": "BAMASDBGAiEA/M0Nvt77aNe+9eYbKsv6rRpTzFTKa5CGqb56ea4hnt8CIQCJDE7pL6xgAewMd5i3G1lrBWgFooT2kd3+zliEz5Rw8w==",
     *     "tree_size": 46466472
     *   },
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE1/TMabLkDpCjiupacAlP7xNi0I1JYP8bQFAHDG1xhtolSY1l4QgNRzRrvSe8liE+NPWHdjGxfx3JhTsN9x8/6Q==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     0
     *   ],
     *   "url": "ct.googleapis.com/aviator/"
     * }
     */
    &sct::Log {
        description: "Google 'Aviator' log",
        url: "ct.googleapis.com/aviator/",
        operated_by: "Google",
        key: b"\x04\xd7\xf4\xcc\x69\xb2\xe4\x0e\x90\xa3\x8a\xea\x5a\x70\x09\x4f\xef\x13\x62\xd0\x8d\x49\x60\xff\x1b\x40\x50\x07\x0c\x6d\x71\x86\xda\x25\x49\x8d\x65\xe1\x08\x0d\x47\x34\x6b\xbd\x27\xbc\x96\x21\x3e\x34\xf5\x87\x76\x31\xb1\x7f\x1d\xc9\x85\x3b\x0d\xf7\x1f\x3f\xe9",
        id: [ 0x68, 0xf6, 0x98, 0xf8, 0x1f, 0x64, 0x82, 0xbe, 0x3a, 0x8c, 0xee, 0xb9, 0x28, 0x1d, 0x4c, 0xfc, 0x71, 0x51, 0x5d, 0x67, 0x93, 0xd4, 0x44, 0xd1, 0x0a, 0x67, 0xac, 0xbb, 0x4f, 0x4f, 0xfb, 0xc4 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Sectigo 'Mammoth' CT log",
     *   "dns_api_endpoint": "comodo-mammoth.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE7+R9dC4VFbbpuyOL+yy14ceAmEf7QGlo/EmtYU6DRzwat43f/3swtLr/L8ugFOOt1YU/RFmMjGCL17ixv66MZw==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     9
     *   ],
     *   "url": "mammoth.ct.comodo.com/"
     * }
     */
    &sct::Log {
        description: "Sectigo 'Mammoth' CT log",
        url: "mammoth.ct.comodo.com/",
        operated_by: "Sectigo",
        key: b"\x04\xef\xe4\x7d\x74\x2e\x15\x15\xb6\xe9\xbb\x23\x8b\xfb\x2c\xb5\xe1\xc7\x80\x98\x47\xfb\x40\x69\x68\xfc\x49\xad\x61\x4e\x83\x47\x3c\x1a\xb7\x8d\xdf\xff\x7b\x30\xb4\xba\xff\x2f\xcb\xa0\x14\xe3\xad\xd5\x85\x3f\x44\x59\x8c\x8c\x60\x8b\xd7\xb8\xb1\xbf\xae\x8c\x67",
        id: [ 0x6f, 0x53, 0x76, 0xac, 0x31, 0xf0, 0x31, 0x19, 0xd8, 0x99, 0x00, 0xa4, 0x51, 0x15, 0xff, 0x77, 0x15, 0x1c, 0x11, 0xd9, 0x02, 0xc1, 0x00, 0x29, 0x06, 0x8d, 0xb2, 0x08, 0x9a, 0x37, 0xd9, 0x13 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Cloudflare 'Nimbus2019' Log",
     *   "dns_api_endpoint": "cloudflare-nimbus2019.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEkZHz1v5r8a9LmXSMegYZAg4UW+Ug56GtNfJTDNFZuubEJYgWf4FcC5D+ZkYwttXTDSo4OkanG9b3AI4swIQ28g==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     1
     *   ],
     *   "url": "ct.cloudflare.com/logs/nimbus2019/"
     * }
     */
    &sct::Log {
        description: "Cloudflare 'Nimbus2019' Log",
        url: "ct.cloudflare.com/logs/nimbus2019/",
        operated_by: "Cloudflare",
        key: b"\x04\x91\x91\xf3\xd6\xfe\x6b\xf1\xaf\x4b\x99\x74\x8c\x7a\x06\x19\x02\x0e\x14\x5b\xe5\x20\xe7\xa1\xad\x35\xf2\x53\x0c\xd1\x59\xba\xe6\xc4\x25\x88\x16\x7f\x81\x5c\x0b\x90\xfe\x66\x46\x30\xb6\xd5\xd3\x0d\x2a\x38\x3a\x46\xa7\x1b\xd6\xf7\x00\x8e\x2c\xc0\x84\x36\xf2",
        id: [ 0x74, 0x7e, 0xda, 0x83, 0x31, 0xad, 0x33, 0x10, 0x91, 0x21, 0x9c, 0xce, 0x25, 0x4f, 0x42, 0x70, 0xc2, 0xbf, 0xfd, 0x5e, 0x42, 0x20, 0x08, 0xc6, 0x37, 0x35, 0x79, 0xe6, 0x10, 0x7b, 0xcc, 0x56 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Cloudflare 'Nimbus2023' Log",
     *   "dns_api_endpoint": "cloudflare-nimbus2023.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEi/8tkhjLRp0SXrlZdTzNkTd6HqmcmXiDJz3fAdWLgOhjmv4mohvRhwXul9bgW0ODgRwC9UGAgH/vpGHPvIS1qA==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     1
     *   ],
     *   "url": "ct.cloudflare.com/logs/nimbus2023/"
     * }
     */
    &sct::Log {
        description: "Cloudflare 'Nimbus2023' Log",
        url: "ct.cloudflare.com/logs/nimbus2023/",
        operated_by: "Cloudflare",
        key: b"\x04\x8b\xff\x2d\x92\x18\xcb\x46\x9d\x12\x5e\xb9\x59\x75\x3c\xcd\x91\x37\x7a\x1e\xa9\x9c\x99\x78\x83\x27\x3d\xdf\x01\xd5\x8b\x80\xe8\x63\x9a\xfe\x26\xa2\x1b\xd1\x87\x05\xee\x97\xd6\xe0\x5b\x43\x83\x81\x1c\x02\xf5\x41\x80\x80\x7f\xef\xa4\x61\xcf\xbc\x84\xb5\xa8",
        id: [ 0x7a, 0x32, 0x8c, 0x54, 0xd8, 0xb7, 0x2d, 0xb6, 0x20, 0xea, 0x38, 0xe0, 0x52, 0x1e, 0xe9, 0x84, 0x16, 0x70, 0x32, 0x13, 0x85, 0x4d, 0x3b, 0xd2, 0x2b, 0xc1, 0x3a, 0x57, 0xa3, 0x52, 0xeb, 0x52 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Google 'Xenon2021' log",
     *   "dns_api_endpoint": "xenon2021.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAER+1MInu8Q39BwDZ5Rp9TwXhwm3ktvgJzpk/r7dDgGk7ZacMm3ljfcoIvP1E72T8jvyLT1bvdapylajZcTH6W5g==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     0
     *   ],
     *   "url": "ct.googleapis.com/logs/xenon2021/"
     * }
     */
    &sct::Log {
        description: "Google 'Xenon2021' log",
        url: "ct.googleapis.com/logs/xenon2021/",
        operated_by: "Google",
        key: b"\x04\x47\xed\x4c\x22\x7b\xbc\x43\x7f\x41\xc0\x36\x79\x46\x9f\x53\xc1\x78\x70\x9b\x79\x2d\xbe\x02\x73\xa6\x4f\xeb\xed\xd0\xe0\x1a\x4e\xd9\x69\xc3\x26\xde\x58\xdf\x72\x82\x2f\x3f\x51\x3b\xd9\x3f\x23\xbf\x22\xd3\xd5\xbb\xdd\x6a\x9c\xa5\x6a\x36\x5c\x4c\x7e\x96\xe6",
        id: [ 0x7d, 0x3e, 0xf2, 0xf8, 0x8f, 0xff, 0x88, 0x55, 0x68, 0x24, 0xc2, 0xc0, 0xca, 0x9e, 0x52, 0x89, 0x79, 0x2b, 0xc5, 0x0e, 0x78, 0x09, 0x7f, 0x2e, 0x6a, 0x97, 0x68, 0x99, 0x7e, 0x22, 0xf0, 0xd7 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "DigiCert Log Server 2",
     *   "dns_api_endpoint": "digicert2.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEzF05L2a4TH/BLgOhNKPoioYCrkoRxvcmajeb8Dj4XQmNY+gxa4Zmz3mzJTwe33i0qMVp+rfwgnliQ/bM/oFmhA==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     2
     *   ],
     *   "url": "ct2.digicert-ct.com/log/"
     * }
     */
    &sct::Log {
        description: "DigiCert Log Server 2",
        url: "ct2.digicert-ct.com/log/",
        operated_by: "DigiCert",
        key: b"\x04\xcc\x5d\x39\x2f\x66\xb8\x4c\x7f\xc1\x2e\x03\xa1\x34\xa3\xe8\x8a\x86\x02\xae\x4a\x11\xc6\xf7\x26\x6a\x37\x9b\xf0\x38\xf8\x5d\x09\x8d\x63\xe8\x31\x6b\x86\x66\xcf\x79\xb3\x25\x3c\x1e\xdf\x78\xb4\xa8\xc5\x69\xfa\xb7\xf0\x82\x79\x62\x43\xf6\xcc\xfe\x81\x66\x84",
        id: [ 0x87, 0x75, 0xbf, 0xe7, 0x59, 0x7c, 0xf8, 0x8c, 0x43, 0x99, 0x5f, 0xbd, 0xf3, 0x6e, 0xff, 0x56, 0x8d, 0x47, 0x56, 0x36, 0xff, 0x4a, 0xb5, 0x60, 0xc1, 0xb4, 0xea, 0xff, 0x5e, 0xa0, 0x83, 0x0f ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Google 'Pilot' log",
     *   "dns_api_endpoint": "pilot.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEfahLEimAoz2t01p3uMziiLOl/fHTDM0YDOhBRuiBARsV4UvxG2LdNgoIGLrtCzWE0J5APC2em4JlvR8EEEFMoA==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     0
     *   ],
     *   "url": "ct.googleapis.com/pilot/"
     * }
     */
    &sct::Log {
        description: "Google 'Pilot' log",
        url: "ct.googleapis.com/pilot/",
        operated_by: "Google",
        key: b"\x04\x7d\xa8\x4b\x12\x29\x80\xa3\x3d\xad\xd3\x5a\x77\xb8\xcc\xe2\x88\xb3\xa5\xfd\xf1\xd3\x0c\xcd\x18\x0c\xe8\x41\x46\xe8\x81\x01\x1b\x15\xe1\x4b\xf1\x1b\x62\xdd\x36\x0a\x08\x18\xba\xed\x0b\x35\x84\xd0\x9e\x40\x3c\x2d\x9e\x9b\x82\x65\xbd\x1f\x04\x10\x41\x4c\xa0",
        id: [ 0xa4, 0xb9, 0x09, 0x90, 0xb4, 0x18, 0x58, 0x14, 0x87, 0xbb, 0x13, 0xa2, 0xcc, 0x67, 0x70, 0x0a, 0x3c, 0x35, 0x98, 0x04, 0xf9, 0x1b, 0xdf, 0xb8, 0xe3, 0x77, 0xcd, 0x0e, 0xc8, 0x0d, 0xdc, 0x10 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Google 'Argon2020' log",
     *   "dns_api_endpoint": "argon2020.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE6Tx2p1yKY4015NyIYvdrk36es0uAc1zA4PQ+TGRY+3ZjUTIYY9Wyu+3q/147JG4vNVKLtDWarZwVqGkg6lAYzA==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     0
     *   ],
     *   "url": "ct.googleapis.com/logs/argon2020/"
     * }
     */
    &sct::Log {
        description: "Google 'Argon2020' log",
        url: "ct.googleapis.com/logs/argon2020/",
        operated_by: "Google",
        key: b"\x04\xe9\x3c\x76\xa7\x5c\x8a\x63\x8d\x35\xe4\xdc\x88\x62\xf7\x6b\x93\x7e\x9e\xb3\x4b\x80\x73\x5c\xc0\xe0\xf4\x3e\x4c\x64\x58\xfb\x76\x63\x51\x32\x18\x63\xd5\xb2\xbb\xed\xea\xff\x5e\x3b\x24\x6e\x2f\x35\x52\x8b\xb4\x35\x9a\xad\x9c\x15\xa8\x69\x20\xea\x50\x18\xcc",
        id: [ 0xb2, 0x1e, 0x05, 0xcc, 0x8b, 0xa2, 0xcd, 0x8a, 0x20, 0x4e, 0x87, 0x66, 0xf9, 0x2b, 0xb9, 0x8a, 0x25, 0x20, 0x67, 0x6b, 0xda, 0xfa, 0x70, 0xe7, 0xb2, 0x49, 0x53, 0x2d, 0xef, 0x8b, 0x90, 0x5e ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "DigiCert Nessie2023 Log",
     *   "dns_api_endpoint": "digicert-nessie2023.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEEXu8iQwSCRSf2CbITGpUpBtFVt8+I0IU0d1C36Lfe1+fbwdaI0Z5FktfM2fBoI1bXBd18k2ggKGYGgdZBgLKTg==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     2
     *   ],
     *   "url": "nessie2023.ct.digicert.com/log/"
     * }
     */
    &sct::Log {
        description: "DigiCert Nessie2023 Log",
        url: "nessie2023.ct.digicert.com/log/",
        operated_by: "DigiCert",
        key: b"\x04\x11\x7b\xbc\x89\x0c\x12\x09\x14\x9f\xd8\x26\xc8\x4c\x6a\x54\xa4\x1b\x45\x56\xdf\x3e\x23\x42\x14\xd1\xdd\x42\xdf\xa2\xdf\x7b\x5f\x9f\x6f\x07\x5a\x23\x46\x79\x16\x4b\x5f\x33\x67\xc1\xa0\x8d\x5b\x5c\x17\x75\xf2\x4d\xa0\x80\xa1\x98\x1a\x07\x59\x06\x02\xca\x4e",
        id: [ 0xb3, 0x73, 0x77, 0x07, 0xe1, 0x84, 0x50, 0xf8, 0x63, 0x86, 0xd6, 0x05, 0xa9, 0xdc, 0x11, 0x09, 0x4a, 0x79, 0x2d, 0xb1, 0x67, 0x0c, 0x0b, 0x87, 0xdc, 0xf0, 0x03, 0x0e, 0x79, 0x36, 0xa5, 0x9a ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Google 'Skydiver' log",
     *   "dns_api_endpoint": "skydiver.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEEmyGDvYXsRJsNyXSrYc9DjHsIa2xzb4UR7ZxVoV6mrc9iZB7xjI6+NrOiwH+P/xxkRmOFG6Jel20q37hTh58rA==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     0
     *   ],
     *   "url": "ct.googleapis.com/skydiver/"
     * }
     */
    &sct::Log {
        description: "Google 'Skydiver' log",
        url: "ct.googleapis.com/skydiver/",
        operated_by: "Google",
        key: b"\x04\x12\x6c\x86\x0e\xf6\x17\xb1\x12\x6c\x37\x25\xd2\xad\x87\x3d\x0e\x31\xec\x21\xad\xb1\xcd\xbe\x14\x47\xb6\x71\x56\x85\x7a\x9a\xb7\x3d\x89\x90\x7b\xc6\x32\x3a\xf8\xda\xce\x8b\x01\xfe\x3f\xfc\x71\x91\x19\x8e\x14\x6e\x89\x7a\x5d\xb4\xab\x7e\xe1\x4e\x1e\x7c\xac",
        id: [ 0xbb, 0xd9, 0xdf, 0xbc, 0x1f, 0x8a, 0x71, 0xb5, 0x93, 0x94, 0x23, 0x97, 0xaa, 0x92, 0x7b, 0x47, 0x38, 0x57, 0x95, 0x0a, 0xab, 0x52, 0xe8, 0x1a, 0x90, 0x96, 0x64, 0x36, 0x8e, 0x1e, 0xd1, 0x85 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "DigiCert Nessie2020 Log",
     *   "dns_api_endpoint": "digicert-nessie2020.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE4hHIyMVIrR9oShgbQMYEk8WX1lmkfFKB448Gn93KbsZnnwljDHY6MQqEnWfKGgMOq0gh3QK48c5ZB3UKSIFZ4g==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     2
     *   ],
     *   "url": "nessie2020.ct.digicert.com/log/"
     * }
     */
    &sct::Log {
        description: "DigiCert Nessie2020 Log",
        url: "nessie2020.ct.digicert.com/log/",
        operated_by: "DigiCert",
        key: b"\x04\xe2\x11\xc8\xc8\xc5\x48\xad\x1f\x68\x4a\x18\x1b\x40\xc6\x04\x93\xc5\x97\xd6\x59\xa4\x7c\x52\x81\xe3\x8f\x06\x9f\xdd\xca\x6e\xc6\x67\x9f\x09\x63\x0c\x76\x3a\x31\x0a\x84\x9d\x67\xca\x1a\x03\x0e\xab\x48\x21\xdd\x02\xb8\xf1\xce\x59\x07\x75\x0a\x48\x81\x59\xe2",
        id: [ 0xc6, 0x52, 0xa0, 0xec, 0x48, 0xce, 0xb3, 0xfc, 0xab, 0x17, 0x09, 0x92, 0xc4, 0x3a, 0x87, 0x41, 0x33, 0x09, 0xe8, 0x00, 0x65, 0xa2, 0x62, 0x52, 0x40, 0x1b, 0xa3, 0x36, 0x2a, 0x17, 0xc5, 0x65 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "DigiCert Yeti2019 Log",
     *   "dns_api_endpoint": "digicert-yeti2019.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEkZd/ow8X+FSVWAVSf8xzkFohcPph/x6pS1JHh7g1wnCZ5y/8Hk6jzJxs6t3YMAWz2CPd4VkCdxwKexGhcFxD9A==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     2
     *   ],
     *   "url": "yeti2019.ct.digicert.com/log/"
     * }
     */
    &sct::Log {
        description: "DigiCert Yeti2019 Log",
        url: "yeti2019.ct.digicert.com/log/",
        operated_by: "DigiCert",
        key: b"\x04\x91\x97\x7f\xa3\x0f\x17\xf8\x54\x95\x58\x05\x52\x7f\xcc\x73\x90\x5a\x21\x70\xfa\x61\xff\x1e\xa9\x4b\x52\x47\x87\xb8\x35\xc2\x70\x99\xe7\x2f\xfc\x1e\x4e\xa3\xcc\x9c\x6c\xea\xdd\xd8\x30\x05\xb3\xd8\x23\xdd\xe1\x59\x02\x77\x1c\x0a\x7b\x11\xa1\x70\x5c\x43\xf4",
        id: [ 0xe2, 0x69, 0x4b, 0xae, 0x26, 0xe8, 0xe9, 0x40, 0x09, 0xe8, 0x86, 0x1b, 0xb6, 0x3b, 0x83, 0xd4, 0x3e, 0xe7, 0xfe, 0x74, 0x88, 0xfb, 0xa4, 0x8f, 0x28, 0x93, 0x01, 0x9d, 0xdd, 0xf1, 0xdb, 0xfe ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Google 'Rocketeer' log",
     *   "dns_api_endpoint": "rocketeer.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEIFsYyDzBi7MxCAC/oJBXK7dHjG+1aLCOkHjpoHPqTyghLpzA9BYbqvnV16mAw04vUjyYASVGJCUoI3ctBcJAeg==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     0
     *   ],
     *   "url": "ct.googleapis.com/rocketeer/"
     * }
     */
    &sct::Log {
        description: "Google 'Rocketeer' log",
        url: "ct.googleapis.com/rocketeer/",
        operated_by: "Google",
        key: b"\x04\x20\x5b\x18\xc8\x3c\xc1\x8b\xb3\x31\x08\x00\xbf\xa0\x90\x57\x2b\xb7\x47\x8c\x6f\xb5\x68\xb0\x8e\x90\x78\xe9\xa0\x73\xea\x4f\x28\x21\x2e\x9c\xc0\xf4\x16\x1b\xaa\xf9\xd5\xd7\xa9\x80\xc3\x4e\x2f\x52\x3c\x98\x01\x25\x46\x24\x25\x28\x23\x77\x2d\x05\xc2\x40\x7a",
        id: [ 0xee, 0x4b, 0xbd, 0xb7, 0x75, 0xce, 0x60, 0xba, 0xe1, 0x42, 0x69, 0x1f, 0xab, 0xe1, 0x9e, 0x66, 0xa3, 0x0f, 0x7e, 0x5f, 0xb0, 0x72, 0xd8, 0x83, 0x00, 0xc4, 0x7b, 0x89, 0x7a, 0xa8, 0xfd, 0xcb ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "DigiCert Nessie2021 Log",
     *   "dns_api_endpoint": "digicert-nessie2021.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE9o7AiwrbGBIX6Lnc47I6OfLMdZnRzKoP5u072nBi6vpIOEooktTi1gNwlRPzGC2ySGfuc1xLDeaA/wSFGgpYFg==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     2
     *   ],
     *   "url": "nessie2021.ct.digicert.com/log/"
     * }
     */
    &sct::Log {
        description: "DigiCert Nessie2021 Log",
        url: "nessie2021.ct.digicert.com/log/",
        operated_by: "DigiCert",
        key: b"\x04\xf6\x8e\xc0\x8b\x0a\xdb\x18\x12\x17\xe8\xb9\xdc\xe3\xb2\x3a\x39\xf2\xcc\x75\x99\xd1\xcc\xaa\x0f\xe6\xed\x3b\xda\x70\x62\xea\xfa\x48\x38\x4a\x28\x92\xd4\xe2\xd6\x03\x70\x95\x13\xf3\x18\x2d\xb2\x48\x67\xee\x73\x5c\x4b\x0d\xe6\x80\xff\x04\x85\x1a\x0a\x58\x16",
        id: [ 0xee, 0xc0, 0x95, 0xee, 0x8d, 0x72, 0x64, 0x0f, 0x92, 0xe3, 0xc3, 0xb9, 0x1b, 0xc7, 0x12, 0xa3, 0x69, 0x6a, 0x09, 0x7b, 0x4b, 0x6a, 0x1a, 0x14, 0x38, 0xe6, 0x47, 0xb2, 0xcb, 0xed, 0xc5, 0xf9 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "DigiCert Yeti2020 Log",
     *   "dns_api_endpoint": "digicert-yeti2020.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEURAG+Zo0ac3n37ifZKUhBFEV6jfcCzGIRz3tsq8Ca9BP/5XUHy6ZiqsPaAEbVM0uI3Tm9U24RVBHR9JxDElPmg==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     2
     *   ],
     *   "url": "yeti2020.ct.digicert.com/log/"
     * }
     */
    &sct::Log {
        description: "DigiCert Yeti2020 Log",
        url: "yeti2020.ct.digicert.com/log/",
        operated_by: "DigiCert",
        key: b"\x04\x51\x10\x06\xf9\x9a\x34\x69\xcd\xe7\xdf\xb8\x9f\x64\xa5\x21\x04\x51\x15\xea\x37\xdc\x0b\x31\x88\x47\x3d\xed\xb2\xaf\x02\x6b\xd0\x4f\xff\x95\xd4\x1f\x2e\x99\x8a\xab\x0f\x68\x01\x1b\x54\xcd\x2e\x23\x74\xe6\xf5\x4d\xb8\x45\x50\x47\x47\xd2\x71\x0c\x49\x4f\x9a",
        id: [ 0xf0, 0x95, 0xa4, 0x59, 0xf2, 0x00, 0xd1, 0x82, 0x40, 0x10, 0x2d, 0x2f, 0x93, 0x88, 0x8e, 0xad, 0x4b, 0xfe, 0x1d, 0x47, 0xe3, 0x99, 0xe1, 0xd0, 0x34, 0xa6, 0xb0, 0xa8, 0xaa, 0x8e, 0xb2, 0x73 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Google 'Argon2021' log",
     *   "dns_api_endpoint": "argon2021.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAETeBmZOrzZKo4xYktx9gI2chEce3cw/tbr5xkoQlmhB18aKfsxD+MnILgGNl0FOm0eYGilFVi85wLRIOhK8lxKw==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     0
     *   ],
     *   "url": "ct.googleapis.com/logs/argon2021/"
     * }
     */
    &sct::Log {
        description: "Google 'Argon2021' log",
        url: "ct.googleapis.com/logs/argon2021/",
        operated_by: "Google",
        key: b"\x04\x4d\xe0\x66\x64\xea\xf3\x64\xaa\x38\xc5\x89\x2d\xc7\xd8\x08\xd9\xc8\x44\x71\xed\xdc\xc3\xfb\x5b\xaf\x9c\x64\xa1\x09\x66\x84\x1d\x7c\x68\xa7\xec\xc4\x3f\x8c\x9c\x82\xe0\x18\xd9\x74\x14\xe9\xb4\x79\x81\xa2\x94\x55\x62\xf3\x9c\x0b\x44\x83\xa1\x2b\xc9\x71\x2b",
        id: [ 0xf6, 0x5c, 0x94, 0x2f, 0xd1, 0x77, 0x30, 0x22, 0x14, 0x54, 0x18, 0x08, 0x30, 0x94, 0x56, 0x8e, 0xe3, 0x4d, 0x13, 0x19, 0x33, 0xbf, 0xdf, 0x0c, 0x2f, 0x20, 0x0b, 0xcc, 0x4e, 0xf1, 0x64, 0xe3 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "DigiCert Nessie2019 Log",
     *   "dns_api_endpoint": "digicert-nessie2019.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEX+0nudCKImd7QCtelhMrDW0OXni5RE10tiiClZesmrwUk2iHLCoTHHVV+yg5D4n/rxCRVyRhikPpVDOLMLxJaA==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     2
     *   ],
     *   "url": "nessie2019.ct.digicert.com/log/"
     * }
     */
    &sct::Log {
        description: "DigiCert Nessie2019 Log",
        url: "nessie2019.ct.digicert.com/log/",
        operated_by: "DigiCert",
        key: b"\x04\x5f\xed\x27\xb9\xd0\x8a\x22\x67\x7b\x40\x2b\x5e\x96\x13\x2b\x0d\x6d\x0e\x5e\x78\xb9\x44\x4d\x74\xb6\x28\x82\x95\x97\xac\x9a\xbc\x14\x93\x68\x87\x2c\x2a\x13\x1c\x75\x55\xfb\x28\x39\x0f\x89\xff\xaf\x10\x91\x57\x24\x61\x8a\x43\xe9\x54\x33\x8b\x30\xbc\x49\x68",
        id: [ 0xfe, 0x44, 0x61, 0x08, 0xb1, 0xd0, 0x1a, 0xb7, 0x8a, 0x62, 0xcc, 0xfe, 0xab, 0x6a, 0xb2, 0xb2, 0xba, 0xbf, 0xf3, 0xab, 0xda, 0xd8, 0x0a, 0x4d, 0x8b, 0x30, 0xdf, 0x2d, 0x00, 0x08, 0x83, 0x0c ],
        max_merge_delay: 86400,
    },

];
