#!/usr/bin/env perl
use v5.10;
use strict;
use warnings;
use Encode;

use FFI::CheckLib;
use FFI::Platypus;

my $ffi = FFI::Platypus->new;

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