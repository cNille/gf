# gf  (git-fast)
A tool for quicker git-handling for terminal nerds.


Build and run `gf` and it will listen to changes in the git-repo.
It will continuously show the latest `git status` in that folder. 

If you want to see the diff for a file, then just enter the index of the file and
press enter.

```
Status ($WORKING_DIR):	(A=add, M=modified, R=rename, D=deleted, ??=not-tracked)
0:  M README.md
1:  M src/main.rs 

(? for help)
Action: 
```

Pressing `0<ENTER>` in the example above would output the diff for `README.md`.

You can also run git commands in the program. No space is needed! 
Example:

```
gfetch -v  
# Is the same as `git fetch -v`

glog -2  
# Is the same as `git log -2`
```

More and faster features will be soon to come.
🏃🏻‍♂️💨
