# TUI Decision

Current decision: do not add a TUI in this release line.

The CLI already covers automation and scripted operator workflows. The native GUI covers review-oriented workflows, package controls, restore controls, dependency graph visualization, and inventory snapshots. A TUI would add another interaction surface without improving restore safety or enterprise certification.

Revisit a TUI only if operators need an interactive terminal review experience on systems where the GUI cannot run.
