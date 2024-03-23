# .dgit structure 
index: point to blobs and path of file (or shortcut access), contains staging changes  
objects: blobs, stored as hash, and cut head for faster lookup   
branches: points to current commit
HEAD: points to branches 

# cli design  
git init creates a .dgit file (like .git) 
git add: stage changes: write blobs to disk -> update object tree -> write index file based on tree   
**todo**: delete write blobs to disk step, as it confusing, to write_files of tree_dir.rs 
**todo**: does not track changes yet, so may add both file despite identical blob

git commit: commit changes: create entire new tree from index file -> write files -> write commit based on that tree -> clear index file   
-> adjust branches to point to it
**write commit**: write commit as a blob   
**write files**: write content of staged files as blob




# Questions 
more trees? 
AVL, red black


overlap of index and treedir struct

# Todo: 
- remove git add write blobs done
- remove overlap of index write, only write hash done

git diff   
git log: commits  

branch: show branch 
branch <branch-name>: new branch
checkout <branch-name>: switch branch (must commit)
