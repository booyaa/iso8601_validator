#!/usr/bin/env perl
use v5.10;
use strict;
use warnings;

# use Encode;
use FFI::Raw;

my $lib_path = "target/debug/";
my $prefix = "lib"; # same for linux, empty for win32
my $extension = ".dylib"; # ".dll" for win32 / ".so" for linux
$lib_path = $lib_path . $prefix . "iso8601" . $extension; 

my $rust = FFI::Raw->new (
    $lib_path, # library
    "validate", # function
    FFI::Raw::ushort, # return type
    FFI::Raw::str # argument types
);

say "rust.validate(\"ddd\") = " . ( $rust->call("ddd") == 1 ? "true" : "false");
say "rust.validate(\"20060831T16:44+00:00\") = " . ( $rust->call("20060831T16:44+00:00") == 1 ? "true" : "false");


