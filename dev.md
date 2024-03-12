# Improvements 
diff: 
Currently git add do not compare differences, it just add everything. If same file change content  
-> new hash -> when insert to tree, overwrite that hash. 
-> when we commit we update that tree 
=> If we can filter out to push to tree only new files, would it be more efficient ? 
It's comparing 2 BTree. 
=> Check cost of BTree insert vs searching/comparing 2 trees   


Live update or not?
Parse to whole tree then traverse write seems better 
We can live update, which means no need for TreeDir but it's fking hard

more trees? 
AVL, red black


overlap of index and treedir struct