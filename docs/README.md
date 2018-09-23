# DART FROG

## Overview

Dart frog is a very simple program that is written in Rust to make running scripts a little easier. I wanted to be able to provide a YAML file that describes 
what I want to run on a system and it would take care of the rest. I don't want to have to install modules, remember a bunch of switches, or make sure that I am
on the right version of an interpreter. I also would like this to be able to use docker containers, flatpacks or snaps at some point which would make running scripts very clean
and not muddy up the machine that the program is run on. This is one of my first projects in Rust, but it has been a lot of fun and if you have suggestions please let me know!

## Goals

* Uses a very simple yaml stucture
* Validates that structure
* Validates that Apps can be installed or that Docker is installed and installs it
* Installs the required Apps or Downloads the appropriate docker container
* Runs the commands in the plan
* Gives helpful feedback
* Has a good library of common Plans for different OS's


## Usage

1. The first thing that you have to do is clone this project and compile it. I will provide binary files in the future, but for now it's a little DIY. There is nothing special that you need to know
and it doens't need `nightly`.
2. Define a project file. See the section below on how to define the project file.
3. Run the binary giving the project file as an argument. 
   example: 
   `dart-frog project.yml`

## Project File

Project files that describe your execution, including:

* Target OS
* Applications
* Package manager
* Commands

Example:
```yaml
---
name: downloader
version: 0.1.0
os:
  name: linux
  flavor: debian
apps:
  - name: curl
    version: 7.58.0
    package: curl
    manager: apt
plan:
  - curl -o example.txt http://www.example.com/
  - ls
  ```

More examples can be found in the [examples](examples) directory of this repository.
