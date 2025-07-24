# TODO

## STAGE 1 

Make a CLI Tool for the basic functions

NO Multithreading + NO WebUI

### Features

- Agree to eula .txt [DONE]
- Set Min and Max Ram [DONE]
- Rework config file from app.cfg to config.toml [DONE]
- Rewrite start_manual into 2 Functions [DONE]
- server TOML and config.toml parsing [DONE]
- stopping servers with jps -l and kill \<PID> or taskkill /PID \ <PID> [DONE]
- download all versions of mc java aswell as modloaders [DONE]
- edit server.properties and user_jvm_args.txt
- reading server stdout and writing to server stdin
    - works fine while app runs 24/7 [DONE]
    - works fine after restarting app
        - small proxy application


## Stage 2

**Completly rewrite the app, because the current Version sucks**

Make the CLI Tool into a CLI-App and a Libary and Multithread it

Make Backend Entry for WebUI


## Stage 3

Make WebUI in JS

### Features

- Install Mods

## Stage 4

Make an actuall Name

Make Packages (exe, deb, rpm, etc.)

Make Website with GH-Pages
 
Make Easy Installer for Windows, Debian / Ubuntu, Fedora /RHEl

Make Docker Image
