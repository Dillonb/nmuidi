# nmuidi

Deletes stuff, hopefully quickly

To download, check out the [releases page](https://github.com/Dillonb/nmuidi/releases) or download [the latest development build](https://nightly.link/Dillonb/nmuidi/workflows/build/main/nmuidi-windows.zip).

[This video](https://www.youtube.com/watch?v=G8BdXgBdaOA) benchmarks several popular suggestions for deleting files quickly on Windows and compares them to nmuidi.

## How to use

### As a command-line tool

You can download using the link above. The easiest way to use it in Windows is to make a folder (something like `C:\bin`) and add that folder to your path. Then add `nmuidi.exe` file you downloaded to that folder and restart any terminals you have open.

Then you can run `nmuidi /path/to/some/dir` and you should see some output like the following:

```PS
→ ~\repos\nmuidi [main ≡ +0 ~1 -0 !]› nmuidi test
Cleaning test
```

To change the log level, set the `RUST_LOG` environment variable: 

PowerShell: `$env:RUST_LOG = 'trace'`

CMD: `set RUST_LOG=trace`

The output will then look something like:

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
