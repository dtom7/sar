## A simple search and replace (sar) tool for UTF-8 encoded text files


    USAGE:
        sar.exe [OPTIONS] -s <SEARCH>
    
    OPTIONS:
        -d <DIRECTORY>                 Directory to search for files recursively. If omitted, current
                                       directory (".") will be taken. Usage: -d "C:\Temp"
            --dry                      Dry run option. No files will be modified. Just displays the
                                       files containing the search text. Usage: --dry
        -h, --help                     Print help information
        -r <REPLACE>                   Text to replace in files. If omitted, blank ("") value will be
                                       taken. Usage: -r test
        -s <SEARCH>                    Text to search in files, cannot be a blank ("") value. Usage: -s
                                       test
        -V, --version                  Print version information
        -x <FILE_EXTENSIONS>...        File extension(s) to include in the search. If omitted, all file
                                       extensions will be included. Usage: -x txt (single file
                                       extension) or -x json -x txt (multiple file extensions) or -x
                                       json txt (multiple file extensions)
