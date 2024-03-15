#!/bin/bash

ARGS=2
E_BADARGS=85
if [$# -ne "$ARGS"]
then
	echo "Usage: `basename $0` first-number second-number"
	exit $E_BADRARGS
	fi
gcd()
{
	dividend=$1
	divisor=$2

	remainder=1

	until [ "$remainder" -eq 0 ]
	do
		let
