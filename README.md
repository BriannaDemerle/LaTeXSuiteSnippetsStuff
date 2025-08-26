# LaTeXSuiteSnippetsStuff
A quick and dirty way to manage my LaTeX Suite snippets in Obsidian in a way that makes adding and removing them easier
Compile it yourself if you want to use it
If you have the .exe then put it in the directory  {ObsidianVault}\.obsidian\plugins\obsidian-latex-suite, and make a anippets.txt there as well

## Snippets
the format goes like this:
\<trigger> := \<replacement> ! \<options>
basically trigger is replaced with replacement with the options specified, usually for math you would want "mA" as your options
### Example
```
xor := \oplus ! mA
<- := \gets ! mA
\leq> := \Leftrightarrow ! mA
```
Note that you can essentially replace any sequence of characters with any other one, even LaTeX commands/symbols
