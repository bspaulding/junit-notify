# junit-notify
Watch a directory of junit xml reports, sending OS notifications on changes.

## Installation

```fish
cargo install junit-notify
```

## Usage

Pass a target directory to watch:

```fish
junit-notify $HOME/code/my-project/test-reports
```

With an optional title to be displayed in the notification:

```fish
cd $HOME/code/my-project
junit-notify ./my-project --title (basename (pwd))
```
