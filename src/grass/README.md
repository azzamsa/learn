# Grass

A collection of daily scripts implemented in various languages.

## Why?

I want to taste another language than Python (my daily driver). By implementing them to my small-scoped real-world problem. I can compare their user-experience, libraries availability, and maturity, easy-to-find documentation, and answers.

## Scripts

Read complete experience in each project.

### 1) [howold](howold) - tell how old are you.

- Python (main)
- Go
- Rust
- Elisp

I miss the Python f-string when working with Go fmt (hope it will be added soon). I can get the job done fast with Python, by utilizing its huge third-party libraries. One reason I try to learn Go simply because I want to be able to share my app in one single small binary. Some python tools can achieve the same thing, but the resulting binary is huge.

Previously, I choose Rust for that goal. But it is hard and complex compared to Go. Given the fact that I never use statically typed language. Modeling the complex nature that Rust trying to solve is hard. So I accept its complexity.

Some of the Rust cons also immature and small ecosystems. But the community and the compiler are very helpful. I keep coming back because I see a lot of potential in it. Learning a completely different language than Python also will expand my views on programming. *Is Rust syntaxes can be made simpler?*

### 2) [sfetch](sfetch) - simple fetch.

- Python (main)
- sh

This is the new chapter in my shell programming journey. After moving to bspwm from i3 I write a bunch of shell scripts. `shellcheck` is very helpful in writing a complex script, but I doubt in the maintenance and readability side. I was looking for other alternative languages. In the end, I settled with Python.

`sfetch` is a simple system information fetcher that I wrote initially in bash, then ported to POSIX-sh. I want it to grow easily, so it end-up rewritten in Python.

I put them here to show how the taste of each language. Though I moved to Python completely before finishing the sh version. For growing applications, I doubt I will write it in bash/sh.
