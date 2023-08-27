TTV v0.0.1
==========

<img src="https://s11.gifyu.com/images/SgQes.gif">

TTV (term-task-viewer) is a lightweight tool to view and manage active processes in Unix machines. It provides an easy interface with vim-like commands, which allows you to easily filter and monitor processes without leaving the terminal, and without wasting unnecessary resources.

⚠️ WARNING: Still under development and likely to contain bugs. ⚠️

Design Goals
------------
- Minimal interface with vim-like commands.
- Filter by memory usage and cpu usage, as well as by process name using a reactive UI.
- Update information in real-time.

Dependencies
------------
- [Cargo & Rust](https://www.rust-lang.org/tools/install)

How to use
----------
- Navigate the list using regular vim keybinds (h, j, k, l).
- Press / to enter search/filter mode. Press enter to navigate filtered list or Esc to return.
- Killing processes works like deleting a line in vim. Press D to select the process for deletion and confirm by pressing D again.

Todo before 0.1.0 
-------------------
- Implement filters for memory and CPU (ascending, descending.)
- Write tests.
- Write Makefile
