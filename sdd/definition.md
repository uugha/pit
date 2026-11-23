
## Implementation 0 - Commands definitions

- help: shows a string of all the commands.
- exit: close program (it can be optional).
- diff: show lines were text lines were changed
- add: add the modified text to the .pit folder

## Implementing add command.


### First iteration
The `add` command adds a file to a tracking folder.
Validate if a document is already being tracked or if its the first time being added.
The file selected is going to be the only file that is going to be tracked.
The added file is going to be added into the .pit folder.

### Second iteration
After adding the file a second time, the program should be able to check what were the lines that changed, store them in chunks in a new file and by line and by column, meaning the starting character were the change was detected.
Each `add` command should create and unique identifier to be able to track it. Probably a identifier with an id related to time. The identifiers can also be stored in a separated file.


# Rationalizing everything

- validate if the folder .pit exit.
- hash the file content, with time stamp and author name.
- store changes in a file using the hash created as the name file.
- store all the changes in that file created.


## How should everything be stored?

My first proposal is that we need to store every change upwards, meaning that the last file is the one with the full code, each file we leave behind are the additions a subtractions.

We'll first start by storing the column and row of each character removed, from the starting character until the finishing character of the modification.

We're worried about how are we going to change between each version and reproduce the exact copy of what was being added in that specific commit.
We may have to iterate over every change from start to the specific commit.

We are we, we is us. We are pendejus.

