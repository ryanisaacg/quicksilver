# Contributing to Quicksilver

Thank you for taking the time to contribute to Quicksilver! Your support really means a lot.

There are two main ways to contribute: [opening issues](#issues) and [submitting patches](#prs).

## Issues

When you're opening an issue, generally there are three main categories.

The first is a question: how do I build this, why is this structure named this, or what does this error message mean.
Please make sure to describe your environment if you're having a problem you need help with!
Including your operating system and its version, your rust compiler version (`rustc -vV`), and any commands you may have run, as well as their output.

The second category of issue is a bug report.
Make sure to describe your environment as above, and to include a snippet of code that demonstrates the bug.
Without a way to reproduce the bug, it will be very hard to fix, so make sure to provide as much information as possible.

The third category of issue is a feature request.
Quicksilver is an evolving project that can definitely use a lot more features, and the project is very open to suggestions.
However, there is a certain category of feature that (generally) won't be considered for work: anything that will only work on one of desktop or web.
A firm goal of quicksilver is unsurprising behavior when taking a web-first project and compiling for desktop and vice-versa.
Also, there's no guarantee anyone else will be up to working on your feature request; if that's the case, then feel free to [open a PR](#prs).


## PRs

The first rule of PRs to Quicksilver is to generally try to make only a few changes in a single PR.
This way each PR maps to about one commit message, which in turn maps to one set of atomic changes to the codebase.
(There are exceptions: if you are making a PR to replace a rendering backend, there is going to be a large diff.)

Below is more information about the project organization and styleguides for Quicksilver, which is helpful to read before submitting a change.

### Branch structure

The main branch of quicksilver is `master`, which generally tracks the previous release on crates.io. Changes that are intended for the next version
are merged into `development`, and then all merged into `master` when it is time for a release.

### Git styleguide

- Use present-tense commands ("do this thing" instead of "does this thing" or "did this thing") so the git log reads like a series of patches
- Keep line lengths at 72 characters or below (for good commit formatting)
- The first line of your commit should be a summary
- Any body text should be seperated from the summary by a blank line
- When referring to an issue or pull request, use its number so that Github auto-formats it correctly

### Documenting your changes

- Add an entry in `CHANGES.md`, which can generally be the summary(s) of your git commits
- If you changed the public API, note [BREAKING] in front of your change
- Additionally, make sure you didn't break the code snippet in the website, the README, or `src/lib.rs`. If you did, make sure to update them.

### Testing your changes

- Make sure to run `cargo test` which will both verify the tests pass and also make sure the crate and all examples compile
- Make sure to run `cargo check --target wasm32-unknown-unknown --examples` which will verify that the crate and all examples compile for web
- If you changed core code (rendering, sound playback, file loading) make sure to actually run any examples you affected,
on any platforms you have changed.

### Dealing with issues

- If an individual commit fixes an issue, note that at the bottom of the summary with Resolves #n, where n is the issue number.
- If a PR as a whole fixes an issue, note that at the bottom of the PR summary with the same syntax.
- If a commit or PR affects an issue but does not fix it, make sure to refer to it by number using the #n syntax so it is linked on Github.


### Quicksilver's Git Workflow

Quicksilver uses rebase and squash-and-merge to merge pull requests into the project. 
This allows for a clean history, where each commit is an atomic change from one working state to another (ideally.)
The only downside is that the branches that are merged in contain commits that do not hit the main branches, which causes 2 issues.
First, if you want to delete a branch for a merged PR, you must use the force delete `-D` flag to git branch.
Second, if you are contributing a PR from a fork, you need to make a branch on your fork, rather than using the fork's master. 
If you use the master branch on your fork, you will either need to do history editing or delete and re-create the fork every time you want to make a PR.

TL;DR for workflow: make a separate branch in your fork for your PRs, and then force-delete them when they've been merged.
