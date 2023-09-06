# Some of the Bash commands on linux
- pwd
- whoami
- cd .. (one level move up)
- cd .. .. (moving two levels up)
- cd .. .. .. (moving three levels up)
- ls /dir or just ls for the current directory
- ls -l long listing
- ls -a or ls -la to see hidden files -a switch
- with && you can join two bash commands
- man <utility> , prints the manual in vim mode
- locate <word> will go through filesystem to find every occurence
    - it requires a database local to the system and its updated once in a day so 
    recently created file will take some time to appear
    - and it tries to find the file with word ocurring in the name of the file:
- whereis <name of the binary> returns location of the binary , mannual and source
- which <name of the binary> will only return the location in the path variable 
- find <dir path> -type f -name <name of the file> 
    - find <path> -name <some name>.*, * will search for any no of character occuring in any sequence
    - find /usr/local -name nmap >> search.txt


- ps used to see all the processes running in the system with aux switch
    - it can give you lot of data , to find specific process you can pipe the result of ps command into grep like this: `ps aux | grep chrome`

- cat is used to display the contents of a file or add more contents to a file or create a new file
    - cat fileName :: shows contents of a file
    - cat > fileName :: creates a new file opens it in interactive mode in the terminal
    - cat >> fileName :: will append content to the file in ther interactive mode
        - single > creates new content and double >> appends the content

- touch will touch the file by updating the date it was created or modified, if file doesnt exist then it will be created

- cp command creates the dup file in a given location
    - cp oldfile /new/path
- mv can rename the oldfile or move it to a new location
    - mv oldfile newfile
    - mv oldfile /path/to/new/location

- rm to remove a file
- rmdir to remove an empty directory
- rm -r to remove the directory and all its contents




