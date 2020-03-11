#!/bin/bash

for f in *.n3*
do
	COUNT=$(tr -d '[:blank:]' < $f | wc -c)
	echo "$f: $COUNT"
done
