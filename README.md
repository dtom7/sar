# A simple text search and replace (sar) tool for UTF-8 encoded text files


    USAGE:
        sar.exe [OPTIONS] -s <SEARCH>
    
    OPTIONS:
        -d <DIRECTORY>                 Directory to search for files recursively. If omitted, current
                                       directory (".") will be taken. Usage: -d "C:\Temp"
            --dry                      Dry run option. No files will be modified. Just displays the
                                       files containing the search text. Usage: --dry
        -h, --help                     Print help information
        -i <IGNORED_DIRS>...           Sub directory(s) to ignore in the search. Usage: -i node_modules
                                       (single directory) or -i node_modules -i target (multiple
                                       directories) or -i node_modules target (multiple directories)
        -r <REPLACE>                   Text to replace in files. If omitted, blank ("") value will be
                                       taken. Supports regex. Usage: -r test
        -s <SEARCH>                    Text to search in files, cannot be a blank ("") value. Supports regex. 
                                       Usage: -s test
        -V, --version                  Print version information
        -x <FILE_EXTENSIONS>...        File extension(s) to include in the search. If omitted, all file
                                       extensions will be included. Usage: -x txt (single file
                                       extension) or -x json -x txt (multiple file extensions) or -x
                                       json txt (multiple file extensions)

### Notes
Minimun Rust version to build this package is `1.63.0`

This tool has been tested in `Windows 10/11 64-bit` platform only

Guidelines on integration tests are in `tests/README.md` 

Make sure to use `--release` flag while building