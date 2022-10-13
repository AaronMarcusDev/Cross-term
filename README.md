# cross-term
 A cross-platform terminal emulator.

### Why?
So you can focus on getting tasks done rather then remembering commands for every OS.

### How?
There is a [list of cross-platform commands](https://github.com/AaronMarcusDev/Cross-term/blob/main/md/list-of-commands.md) that execute the corresponding command for the user's operating system on the backgroud and returns the result.

**Example:**
```bash 
cwd> ls
```
What is being executed on the backgroud:
| Windows | Unix|
| --------------- | --------------- |
| dir | ls |

Note: All commands that are not recognized, will be executed as a system command.
