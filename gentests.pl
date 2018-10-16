#!/usr/bin/perl -w
use strict;

die "Usage: gentests.pl stepN" unless $ARGV[0];
my $st = $ARGV[0];

my $fn = "";
while(< /mnt/src/git/mal/tests/$st*mal >) {
    $fn = $_;
}

print "fn=$fn\n";

open F, "<", $fn or die;
open G, ">", "$st/src/tests.rs" or die;

my $in = undef;

my $ctr = 0;

sub esc($) {
    local $_ = $1;
    s!\\!\\\\!g;
    s!"!\\"!g;
    return $_;
}

while(<F>) {
    chomp;
    if (m!^;;(.*)!) {
        print G "// $1\n";
    } elsif (m!^\;\=\>(.*)!) {
        print G "#[test]\n";
        print G "fn test$ctr() {\n";
        print G "    super::test_it (\n";
        print G "        \"".esc($in)."\",\n";
        print G "        \"".esc($1)."\",\n";
        print G "    );\n";
        print G "}\n\n";

        $ctr+=1;
        $in="";
    } elsif (m!^\s*$!) {
    } else {
        $in = $_;
    }
}
