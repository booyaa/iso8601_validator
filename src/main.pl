#!/usr/bin/env perl
use v5.10;
use strict;
use warnings;
use Encode;

# use FFI::Raw;

# my $lib_path = "target/debug/";
# my $prefix = "lib"; # same for linux, empty for win32
# my $extension = ".dylib"; # ".dll" for win32 / ".so" for linux
# $lib_path = $lib_path . $prefix . "iso8601" . $extension;

# my $rust = FFI::Raw->new (
#     $lib_path, # library
#     "validate", # function
#     FFI::Raw::ushort, # return type
#     FFI::Raw::str # argument types
# );

# say "rust.validate(\"ddd\") = " . ( $rust->call("ddd") == 1 ? "true" : "false");
# say "rust.validate(\"20060831T16:44+00:00\") = " . ( $rust->call("20060831T16:44+00:00") == 1 ? "true" : "false");

# use FFI::CheckLib 0.06;
use FFI::CheckLib;
use FFI::Platypus;

my $ffi = FFI::Platypus->new;

# my $lib_path = "target/debug/";
# my $prefix = "lib"; # same for linux, empty for win32
# my $extension = ".dylib"; # ".dll" for win32 / ".so" for linux
# $lib_path = $lib_path . $prefix . "iso8601" . $extension;
# $ffi->lib($lib_path);
my $lib = get_lib("iso8601");
say $lib;
$ffi->lib($lib);

$ffi->attach(validate => ['string'] => 'uint16');

say "rust.validate(\"ddd\") = " . validate("ddd");
say "rust.validate(\"20060831T16:44+00:00\") = " . validate("20060831T16:44+00:00");

sub get_lib{
    my $lib_name = $_[0];
    my $lib_path = "target/debug/";
    my $prefix = "lib"; # same for linux, empty for win32
    my $extension = ".dylib"; # ".dll" for win32 / ".so" for linux
    return $lib_path . $prefix . $lib_name . $extension;
}