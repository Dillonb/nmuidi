# nmuidi

Deletes stuff, hopefully quickly

[Download for Windows](https://nightly.link/Dillonb/nmuidi/workflows/build/main/nmuidi-windows.zip)

## How to use

### As a command-line tool

You can download using the link above. The easiest way to use it in Windows is to make a folder (something like `C:\bin`) and add that folder to your path. Then add `nmuidi.exe` file you downloaded to that folder and restart any terminals you have open.

Then you can run `nmuidi /path/to/some/dir` and you should see some output like the following:

```PS
→ ~\repos\nmuidi [main ≡ +0 ~1 -0 !]› nmuidi test
Cleaning test
```

If you want to see the timings of your execution you'll need to set an environmental variable 
(Powershell: $env:RUST_LOG = ‘trace', CMD: set RUST_LOG=debug), the output would look something like:

```PS
→ ~\repos\nmuidi [main ≡ +0 ~1 -0 !]› nmuidi test1 test2
Cleaning test1
Cleaning test2
Total time: 10.00s
Directory timings:
    dir test1 took 5.00s
    dir test2 took 5.00s
Done.
```

### As a package

1. `cargo add nmuidi`
2. add `use nmuidi::nmuidi::Cleaner;`
3. Create a cleaner and clean `Cleaner::new("some/path").clean();`


## Why the dumb name

1. It's an inside joke <https://steamcommunity.com/app/570/discussions/0/558748653730465633/>
2. Having a complicated name makes it harder to accidentally nuke a folder. This program does NOT ask you to confirm, if you tell it to delete something it will start deleting things immediately.
