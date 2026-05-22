#!/bin/sh

grep -Frnl "//~" $1 | xargs rm -f
grep -Frnl "//~^" $1 | xargs rm -f
grep -rnl compile-flags: $1 | xargs rm -f
grep -rnl run-fail $1 | xargs rm -f
grep -rnl build-fail $1 | xargs rm -f
grep -rnl only-[a-zA-Z0-9] $1 | xargs rm -f
grep -rnil thread $1 | xargs rm -f
grep -Frnl "[feature(" $1 | xargs rm -f
grep -Prl "(?<!(:|[a-zA-Z_.]))env(?![0-9a-zA-Z])" $1 | xargs rm -f
grep -rl std::sync $1 | xargs rm -f
sed -i "s/^fn main/pub fn main/" $1/*.rs

prelude='use std::prelude::\*\;'
sed -zi "/$prelude/!s/$/\\n$prelude/" $1/*.rs
rm $1/*.stderr
