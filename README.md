# rusty_ASA_proj2
This is my first time programming in Rust, a language that I've wanted to learn for a while now. I 
decided to try to reproduce a project that I once developed for a subject in university. The 
obective is pretty simple: you give a series of relations between people (represented by integers) 
via stdin, preceeded by a special pair of people (will explain later). For example, the
input: 
``
2 3
1 2
1 3
``
means the special pair are the people 2 and 3 and that 1 is both a parent of 2 and 3.
The objective is to output to the stdout the closest common ansestor(s) between the "special" pair
of people (in this example, 1). Also, it has to be the most efficient program possible, time and
memory-wise. (take note that we had to be able to process +10 million relations in 0.1s)
"Side quests": we also had to check there weren't any cycles in the tree and there was no person
with more than two parents.

(I will later create a git repo with my univerity projects and will link the original version of
this here)
