#!/usr/bin/perl
use strict;
use warnings;
use JSON;
use Data::Dumper;

# This regex matches the token syntax [.*] and splits it into a KEY:VALUE match
my $regex = qr/(\[(?<key>[^\[:]+):?(?<value>[^\]\[]*)])/mp;

# some unwanted non-parameter tokens
my $unwanted = qr/^ATTACK_FLAG.*|^TL_.*/mp;

# sometimes people put spaces and commas in their object ids..
sub slug {
    my $s = shift;
    $s =~ s/,/-/g;
    $s =~ s/\W//g;
    return $s;
}

# supply the raw file as an argument
my $filename = shift or die "Usage: $0 FILENAME\n";

open( FH, '<', $filename ) or die $!;

my %currobj = ();
my $rawfile = "";
my @results;

while (<FH>) {
    if ( $. == 1 ) {
        $rawfile = $_;
        $rawfile =~ s/^\s+|\s+$//g
    }

    if ( $_ =~ /$regex/g ) {

        # print("$+{key} :: $+{value}\n");
        my $key = $+{key};
        my $val = $+{value};

        if ( $val eq "") {
            if ( exists( $currobj{'id'}) and $key !~ /$unwanted/) {
                push @{$currobj{'attributes'}}, $key;
            }
        }        elsif ( $key eq 'CREATURE' ) {
            if ( exists( $currobj{'type'} ) and $currobj{'type'} eq 'CREATURE' ) {                
                # print Dumper( \%currobj );
                push @results, encode_json (\%currobj);
            }
            %currobj = (
                id   => $val,
                type => "CREATURE",
                raw => $rawfile,
                objectID => sprintf("%s-%s-%s", $rawfile, "CREATURE", slug($val)),
                attributes => []
            );
        }
        elsif ( $key eq 'NAME' ) {
            @currobj{'namelist'} = [ split( /:/, $val ) ];
            $currobj{'name'}     = $currobj{'namelist'}[2];
        }
        elsif ( $key eq 'DESCRIPTION' ) {
            $currobj{'description'} = $val;
        }
        elsif ( $key eq 'CLUTCH_SIZE' ) {
            $currobj{'clutchRange'} = [ split( /:/, $val)];
        }
        elsif ( $key eq 'EGG_SIZE') {
            $currobj{'eggSize'} = $val;
        }
        elsif ( $key eq 'BODY_SIZE') {
            my @size = split( /:/, $val);
            my $yearf = $size[0] + ($size[1] / 336);
            my %sizehash = (
                years => sprintf("%.2f", $yearf),
                size => $size[2]
            );
            if (not exists( $currobj{'size'})) {
                $currobj{'size'} = [ \%sizehash];
            }
            else
            {
            push @{$currobj{'size'}}, \%sizehash;
            }
        }
    }
}

# print Dumper( \%currobj )
if (exists( $currobj{'type'} ) and $currobj{'type'} eq 'CREATURE') {
push @results, encode_json (\%currobj);
}
print join(", ", @results);

