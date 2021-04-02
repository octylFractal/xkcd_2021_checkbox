xkcd_2021_checkbox
==================
Client for XKCD's 2021 April Fools, "Checkbox".

Usage is pretty simple, just type some stuff in the box. It'll panic if you give it anything
not translatable to morse, though, so be careful!

There is a special syntax for commands controlling the client itself, rather than querying:
Prefixed with `%`.
Currently, there is only one command:

|Name|Args|Description|
|----|----|-----------|
|state|`[state_id]`|Sets/clears the internal state ID (used to track progress, printed on change)|
