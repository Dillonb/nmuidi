# nmuidi

Deletes stuff, hopefully quickly

[Download for Windows](https://nightly.link/Dillonb/nmuidi/workflows/build/main/nmuidi-windows.zip)

## How to use

You can download using the link above. The easiest way to use it in Windows is to make a folder (something like `C:\bin`) and add that folder to your path. Then add `nmuidi.exe` file you downloaded to that folder and restart any terminals you have open.

Then you can run `nmuidi /path/to/some/dir` and you should see some output like the following:

```PS
→ ~\repos\nmuidi [main ≡ +0 ~1 -0 !]› nmuidi test
Cleaning test
Cleaning with 3200 threads.
Done cleaning files, took 0 seconds. Starting on dirs
Done sorting, took 0 seconds. Starting to delete directories.
Done deleting directories, took 0 seconds. Entire process took 0 seconds.
Done.
```

## Why the dumb name

1. It's an inside joke <https://steamcommunity.com/app/570/discussions/0/558748653730465633/>
2. Having a complicated name makes it harder to accidentally nuke a folder. This program does NOT ask you to confirm, if you tell it to delete something it will start deleting things immediately.
