# Module and package system, Prelude

## Preface

As the project grows larger, writing all the code in one file is an extremely elementary and stupid approach. Generally speaking, it has the following disadvantages:

1. The file is large, and the editor is slow to open;
2. All the codes are placed in the same file, which cannot make good use of modern multi-window editors, and view and edit two code fragments associated with it at the same time;
3. There are too many codes, it is too slow to find a certain keyword, and the efficiency of locating a certain line of code will be greatly reduced;
4. It will greatly increase the frequency of turning up and down, causing the middle wheel of your mouse to be easily damaged;
5. Turning up and down constantly will cause you to feel dizzy;
6. When dizzy, it is easy to write wrong code, or even change a certain line in the wrong file (similar place, wrong place);
7. When a bug occurs, according to the error feedback, it is known which piece of logic is the problem, but it is not easy to quickly locate it;

Thus, modules are the infrastructure of almost all languages, although they are called differently.
