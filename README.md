# Destined

Command line time management in rust!

## Features

- Track habits
- Track tasks
- Organize with goals
- Time block with a calendar

## Usage

### Syntax

Use your favorite editor to schedule tasks

```
- no goal task

# goal 1

- unscheduled task
- task to be done on a particular date _12dec2024
- task to be done on a particular date and time _12dec2028_2:20pm
- habit that repeats daily _RD
- habit that repeats daily at a time _RD _10:00am
- other available repeat intervals _RW _RM _RY
- have due dates as reminders _due_12jan2025
- can have time too _due_12jan2025_12:11pm
- habit that repeats daily and have a duration in hours _RD _10:00am _for_2.5h
- habit that can be tracked in the interval _RD _T _10:00am

# goal 2

...
```

## Setup

Put the required keys in `.destined` file in the location of executable. This is the default configuration:

```
todo_file=destined-todos.md
history_file=destined-history.md
undo_dir=.destined-undo
editor=nvim
```
