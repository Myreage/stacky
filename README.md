(Work in progress)

# Vision

The purpose of this tool is to automate the maintenance of stacked pull requests on github.
Learn more about stacking here: https://stacking.dev/

# Features
- Create and manage stacks (groups of dependent branches)
- Automate rebases and sync with remote github branches and pull requests
- Automated goodies on pull requests (stack graph with links for instance)

# Usage

## Stacks

Show the current stack
```bash
stacky stack
```

Checkout to a stack
```bash
stacky stack MyStack
```

Create and checkout to a new stack
```bash
stacky stack MyStack --create
```

## Branches

Checkout to a branch on current stack
```bash
stacky branch MyBranch
```

Create and checkout to a new branch on current stack
```bash
stacky branch MyBranch --create
```

