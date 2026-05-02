pub fn print_help() {
    println!(
        "\
\x1b[1;36mAvailable commands:\x1b[0m

  \x1b[1;32mhelp\x1b[0m
      Show this help message

  \x1b[1;32mexit\x1b[0m
      Exit the shell

  \x1b[1;32mpwd\x1b[0m
      Print the current directory

  \x1b[1;32mcd <dir>\x1b[0m
      Change current directory
      \x1b[33mcd\x1b[0m           -> go to home directory
      \x1b[33mcd folder\x1b[0m    -> go to folder

  \x1b[1;32mecho <text>\x1b[0m
      Print text
      \x1b[33mecho hello\x1b[0m

  \x1b[1;32mcat <file>\x1b[0m
      Print file content
      \x1b[33mcat -\x1b[0m        -> read from keyboard until Ctrl+D

  \x1b[1;32mmkdir <dir>\x1b[0m
      Create a directory

  \x1b[1;32mrm <file>\x1b[0m
      Remove a file

  \x1b[1;32mrm -r <dir>\x1b[0m
      Remove a directory and everything inside it

  \x1b[1;32mcp <src> <dest>\x1b[0m
      Copy a file

  \x1b[1;32mcp <file1> <file2> <dir>\x1b[0m
      Copy many files into a directory

  \x1b[1;32mmv <src> <dest>\x1b[0m
      Move or rename a file

  \x1b[1;32mls\x1b[0m
      List files in current directory

  \x1b[1;32mls -a\x1b[0m
      Show hidden files

  \x1b[1;32mls -F\x1b[0m
      Add / after directories

\x1b[1;36mSpecial keys:\x1b[0m
  \x1b[33mCtrl+C\x1b[0m   Stop current command
  \x1b[33mCtrl+D\x1b[0m   Exit cat - or exit shell
"
    );
}