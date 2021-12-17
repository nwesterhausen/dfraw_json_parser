#!/usr/bin/perl
use strict;
use warnings;
use Data::Dumper;

# This regex matches the token syntax [.*] and splits it into a KEY:VALUE match
my $regex = qr/(\[(?<key>[^\[:]+):?(?<value>[^\]\[]*)])/mp;

# Don't have this be hardcoded for future.
my $filename = "creature_domestic.txt";

open( FH, '<', $filename ) or die $!;

my %currobj = ();

while (<FH>) {

    if ( $_ =~ /$regex/g ) {

        # print("$+{key} :: $+{value}\n");
        my $key = $+{key};
        my $val = $+{value};

        if ( $key eq 'CREATURE' ) {
            if ( exists( $currobj{'id'} ) ) {
                print Dumper( \%currobj );
            }
            %currobj = (
                id   => $+{value},
                type => "CREATURE"
            );
        }
        elsif ( $key eq 'NAME' ) {
            @currobj{'namelist'} = [ split( /:/, $+{value} ) ];
            $currobj{'name'}     = $currobj{'namelist'}[2];
        }
    }
}

print Dumper( \%currobj )

