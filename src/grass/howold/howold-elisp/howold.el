(require 'ts)

(ts-human-format-duration (ts-diff (ts-now) (ts-parse "2000-11-28")))

;; "19 years, 294 days, 5 hours, 47 minutes, 14 seconds"
