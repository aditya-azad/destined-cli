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

- task 
- task to be done on a particular date _12dec2024
- task to be done on a particular date and time _12dec2028_2pm
- habit that repeats daily _RD
- habit that repeats daily at a time _RD _10:00am
- other available repeat intervals _RW _RM _RY
- have due dates as reminders _due_12jan2025
- can have time too _due_12jan2025_12pm

# goal 2

...
```

## Setup

Put the required keys in `.destined` file in the location of executable. This is the default configuration:

```
todo_file=destined-todos.md
history_file=destined-history.md
undo_file=.destined-undo.md
editor=nvim
```
