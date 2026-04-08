## A small beginner-friendly Unix shell written in Rust.
This project started as a way to learn low-level systems concepts like process creation, pipes, job control, and terminal handling.Essentially rebuilding a tiny subset of bash to understand how shells actually work.

## It’s not meant to be feature complete. The goal was learning how shells manage processes, foreground/background jobs, and I/O redirection using Rust and Unix syscalls.

## What this shell supports
->Running external programs (ls, cat, grep, etc.)
->Pipelines (|)
->Input redirection (<)
->Output redirection (>)
->Append redirection (>>)
->Background execution (&)
->Job control (jobs, fg, bg)
->Built-in commands (cd, pwd, echo, exit)
->Basic foreground process handling
->Suspended job tracking (Ctrl+Z)
->Multiple process pipeline execution
