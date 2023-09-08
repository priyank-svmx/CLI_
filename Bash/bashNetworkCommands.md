# Network Commands

-`ifconfig` gives info on active network interfaces
- `iwconfig` doesnt exist for mac but there something called `airport` utility which can be helpful
- `dig` command you can examine DNS information for a target domain
    - one can add different options to get more info, `ns` `mx`
    - `dig google.com`

- use `dig` to get the dns information like ip address
    - then use `host` to reverse check the ip address for the domain name
    - `host <ipaddress>`

- `traceroute` see how many hops it took to reach the destination
    - if you see ``*`` in any of the hop it means at the step multiple attempts were made
    - `traceroute google.com`

- `curl google.com` will make a http request to the domain name and get the content
    - `curl -I google.com` to get the header only
    - `curl -L google.com` to get redirected to the site if there is redirect at the server you are hittig after the curl

-  you can use `curl` and json-server to develop api testing or just use curl to test how api is working

- 'history' commands gives you the history of all the commands palyed so far at the commandlie
- `stdout, stdin, stderr` there are three streams in a unix based systems
- `ls > /dev/null` to suppress the output of a command and not sending to any stream or a file and completely ignore it
 
- `< << , > >>` , `<<<` these are ?
- there is `awk` and there is `sort -rn` 
- `tee`, to put a t pipeline between the stream and the output stream
- `pbcopy` and `pbpaste`
    - pbcopy copies the stream to the clipboard
    - pbpaste copies the clipboard to the stream

- `grep` to search for a text in a given file or a stream piped to 
    - `grep -v` show lines that dont contain the search pattern
        - `grep search-string -v filePath`
    - we can use grep on multiple files
















