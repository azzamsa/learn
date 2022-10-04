# Howold

## Summary

I have good experience with Python and GO. Both of them have easy-to-find answer
on how to *diff between two datetimes*. They also have a similar ability to
find `years - months - days` difference. Python supported by
[dateutil][dateutil] module, and Go supported by [Gox/timex][gox].

Sadly, rust [chrono] only have `days` in their diff. So it's not a piece-of-cake
when calculating the time diff. You have to figure out your self.

I added elisp for more comparison. Working with default `parse-time-string` is
tedious -- I have work with it in my previos projet before `ts.el` was born --,
so I pick `ts.el`. Sadly it is also doesn't display *days* in the diff.


[dateutil]: https://github.com/dateutil/dateutil
[gox]: https://github.com/icza/gox/tree/master/timex
[chrono]: https://github.com/chronotope/chrono/issues/416
