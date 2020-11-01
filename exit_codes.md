### A review of all possible exit codes.

In an ideal world the program never panics and always returns a exit code.

| Exit code | Explaination |
| -- | -- |
| 1x | Error reading in data |
| 10 | IO error |
| 11 | Invalid float value in file |
| 12 | Error reading line from file |
| 13 | File was empty |
| 14 | Not all points in file where the same length |
| 15 | Dimensionality of the reduced data is not supported |
| 16 | Could not parse the argument passed for p as float |
| 17 | Invalid enum value was passed in the cli |
| 2x | Error writing data |
| 3x | Error processing data |
| 41 | Program entered an invalid state |
| 42 | Threadpool could not be build |
