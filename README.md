<!-- markdownlint-configure-file {
  "MD013": {
    "code_blocks": false,
    "tables": false
  },
  "MD033": false,
  "MD041": false
} -->

<div align="center">

# gt

gt is a super simple script to copy the last commit message of the current git repository.

[Getting started](#getting-started) •
[Installation](#installation)

</div>

## Getting started

```sh
cd into/your/repository   # you have to be in a git repository
gt                        # copy last commit message
```

## Installation

### *Step 1: Install gt*
Right now, gt is only installable via homebrew

<details>
<summary>MacOS</summary>

To install gt, run these commands in your terminal:

```sh
brew tap konrad-amtenbrink/tap
brew install gt
```
</details>


### *Step 2: Add gt to you shell*

<details>
<summary>MacOS</summary>

To add gt to you shell, run these commands in your terminal:

```sh
echo 'alias gt="/usr/local/Cellar/gt/0.1.1/bin/gt"' >> ~/.zshrc
```
</details>


