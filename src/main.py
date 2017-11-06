#!/usr/bin/env python3

import sys, ctypes
from ctypes import c_char_p, c_bool


prefix = {'win32': ''}.get(sys.platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
lib = ctypes.cdll.LoadLibrary("target/debug/" + prefix + "iso8601" + extension)

lib.validate.argtypes = (c_char_p,)
lib.validate.restype = c_bool

print("rust.validate(\"ddd\") = ",lib.validate(b"ddd"))
print("rust.validate(\"20060831T16:44+00:00\") = ",lib.validate(b"20060831T16:44+00:00"))