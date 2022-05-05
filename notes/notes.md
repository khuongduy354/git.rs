# Features working on

- Add
  - add multiple files
- Commit

# Final directories

.dgit
-HEAD file
-index file
-branches dir
-objects dir

# ISSUEs

- problem when git add, then change then add again, the previous blob isn't deleted
-> write blobs when commit ?
# SOLVED

- update deleted file when add -> when read from index, remove all directory that's isnt exist
a
# Algorithms for files manipulation

1. Write to staging area (index file)

- scan depend on path(recursion)
  if it's a path to dir, take all files inside it
  or
  if it's a path to file, take that file
  or
  if it's a path to an empty dir, take the dir
  -> Insert them into BTreeMap as hash-path key-value

  After finished,
  -> Write blobs according to BTreeMap

# File representation

Blob file
data

Tree file
blob hash name
tree hash name

Commit file
commit
message
tree_hash
parent_hash (if available)

index file
hash path

branch file (default master)
commit_hash
