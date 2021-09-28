Todos
=====

1. If a bunch of changes to different files happen in close timing, then a bunch of notifications are generated.
   We want to batch these. Basically, we want to use the file event as a trigger, and possibly even only re-check those files,
	 but debounce the notification sending on the file events. We also want to combine this with #2, so that the notification is a reflection
   of all the tests in the specified directory.
      - looks like hotwatch is already debouncing at 2 seconds, so maybe we don't want to debounce, perhaps just combining is enough.
2. Maybe: Store last state of reports in memory, only emit notification if status changed.
3. Will need to detect that a file has been removed and remove that from the current results. I assume there's a FileDelete event akin to FileWrite.
4. More useful notification title, perhaps the basename of the directory, or the dir where executed, and/or take it as an optional arg?
