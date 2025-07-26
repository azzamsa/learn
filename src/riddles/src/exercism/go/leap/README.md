# Leap

Welcome to Leap on Exercism's Go Track.
If you need help running the tests or submitting your code, check out `HELP.md`.

## Introduction

A leap year (in the Gregorian calendar) occurs:

- In every year that is evenly divisible by 4.
- Unless the year is evenly divisible by 100, in which case it's only a leap year if the year is also evenly divisible by 400.

Some examples:

- 1997 was not a leap year as it's not divisible by 4.
- 1900 was not a leap year as it's not divisible by 400.
- 2000 was a leap year!

~~~~exercism/note
For a delightful, four-minute explanation of the whole phenomenon of leap years, check out [this YouTube video](https://www.youtube.com/watch?v=xX96xng7sAE).
~~~~

## Instructions

Your task is to determine whether a given year is a leap year.

You will see a `cases_test.go` file in this exercise. This holds the test
cases used in the `leap_test.go`. You can mostly ignore this file.

However, if you are interested... we sometimes generate the test data from a
[cross language repository][problem-specifications-leap]. In that repo
exercises may have a [.json file][problem-specifications-leap-json] that
contains common test data. Some of our local exercises have an
[intermediary program][local-leap-gen] that takes the problem specification
JSON and turns in into Go structs that are fed into the `<exercise>_test.go`
file. The Go specific transformation of that data lives in the `cases_test.go` file.

[problem-specifications-leap]: https://github.com/exercism/problem-specifications/tree/master/exercises/leap
[problem-specifications-leap-json]: https://github.com/exercism/problem-specifications/blob/master/exercises/leap/canonical-data.json
[local-leap-gen]: https://github.com/exercism/go/blob/main/exercises/practice/leap/.meta/gen.go

## Source

### Created by

- @kytrinyx

### Contributed to by

- @alebaffa
- @bitfield
- @da-edra
- @ekingery
- @ferhatelmas
- @hilary
- @ilmanzo
- @k4rtik
- @leenipper
- @petertseng
- @robphoenix
- @sebito91
- @soniakeys
- @tleen
- @tompao
- @zabawaba99
- @eklatzer

### Based on

CodeRanch Cattle Drive, Assignment 3 - https://coderanch.com/t/718816/Leap