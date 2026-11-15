# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.

## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable = { $count ->
        [one] { $count } shortcut on this drive names no file, so whether it opens this app could not be told.
       *[other] { $count } shortcuts on this drive name no file, so whether they open this app could not be told.
    }
