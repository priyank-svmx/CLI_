# Text Manipulation

- `head` will show the first 10 lines of a given file
    - `head -20 /file/path` this will first 20 lines

- `tail` would give the tail lines of the file
    - `tail -20 /file/path` will print last 20 lines of the file

- `nl` prints all the lines with the line numbers / not the newlines characters
    - `nl /file/path`

- `grep` can be used to search a given string, however it is always used in conjunction with pipe 
    - `nl /file/path | grep some_string_to_find`
- `sed` (stream editor) is used to stream a file or given input through a transformation andoutput can be piped to a file or std::output
    - cat /file/path/ | sed s/stringToFind/replacementString/g

- `more` takes you through the file one page at a time 
- `less` just like `more`, however gives you `/` to search any term in the file





































