- Add
  - add a directory, add multiple files
  - only take file name as path
- Commit

  - write file contents from the staging area

- END directories
  .dgit
  -HEAD file
  -index file
  -branches dir
  -objects dir

- ISSUEs

  - problem when git add, then change then add again, the previous blob isn't deleted
  - sanitize path (add root for every input path, check if it's abosulute or relative )

- SOLVED
  - 2 different file name with the same content, same hash -> adding file's name to hash
  - when read from index, remove all directory that's isnt exist

//commit metadata
father-tree hash
committer
message
parent

// add a dir
scan all files of dir -> add each of it into a tree hash, insert to index hash
do until all of the file in dir are blobs

//tree
-> tree
-> Vec<Blobs>

//git commit

Blob file
data

Tree file
blob hash path
blob hash path
blob hash path
tree hash path
tree hash path
