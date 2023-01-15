# Story files

example file:
```
[START]
You enter a forest.
\WEST1 (west){You think of going west} 
\EAST1 (east){You think of going east}
\Lay (lay){You decide to take a rest}

[WEST1]
You go west
\EAST1 (east){east}

[EAST1]
You reached the end
\EAST (e){go}

[Lay]
You reached the end
\ (){}

[EAST]
Game over
\ (){}
```

Each file starts with [START] identifier to indicate the beginning of the story. Square brackets indicate the Id of the story excerpt, therefore all of them should be [Unique] to be able to map correctly.

The next line is the content of the story.

Starting with the slash \, indicates a "command" or mapping.
\Id_to_the_next_block (label to be shown){description to be shown}.

If it is the end of the story and it is left empty: \\(){}.
(Note in the example it is shown as \\ (){}, that is because the syntax highlighting doesn't work correctly in the vs code extension I wrote).
