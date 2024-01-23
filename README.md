(Work in progress)

# Vision

The purpose of this tool is to automate the maintenance of stacked pull requests on github.
Learn more about stacking here: https://stacking.dev/

# Features
- Create and manage stacks (groups of dependent branches)
- Automate rebases and sync with remote github branches and pull requests
- Automated goodies on pull requests (stack graph with links for instance)

# Short tutorial

Short example of how you can use this tool to develop a feature you wish to split in 3 stacked pull requests.

Create and checkout to a new stack
```bash
stacky stack --create MySuperFeature 
```

Create and checkout to a new branch on current stack. 
```bash
stacky branch --create DBMigration 
```

Do some changes to your code, and commit it. 

Then, create a new branch
```bash
stacky branch --create BackendAPI 
```

Do some changes to your code, and commit it. 

Then, create a new branch
```bash
stacky branch --create Frontend 
```

Do some changes to your code, and commit it. 

You can inspect your current stack with
```bash
stacky stack
```

When you are ready to open Pull Requests for your stack, run
```bash
stacky sync
```

This will rebase your branches into each other, push the branches, and open Pull Requests.
If your branches are already pushed on remote, they will be pulled before rebase.
If you have any conflicts, solve them and run this command again.


