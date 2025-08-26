# mc-server-management-rust

rust tool for managing multiple selfohosted minecraft server

**THIS APP IS CURRENTLY WORK IN PROGRESS**

**NOT ALL FEATURES FULLY WORK**

**FOR MORE INFO LOOK [HERE](https://github.com/Delfi-CH/mc-server-manager-rust/blob/main/TODO.md)**

This app belongs to [delfi-ch/mc-server-panel](https://github.com/Delfi-CH/mc-server-panel). For a general overview of the project, please look there.

## Requirements

### Processor:

x86-64 based Processor.
(Any Intel/AMD Processor released in the last 15 years should work)

### RAM:

Minimum: 4 Gigabytes

Recommended: 8 Gigabytes and more.

### Operating Systems

**Only Windows and popular Linux Distributions are supported and have downloadable exectuables.**
**However: Compiling on the BSD variants and MacOS should be possible but is not supported.**

**Beta/Testing/Unstable Versions of any Operating System are not supported.**

#### Windows:

- Windows 10
- Windows 11

#### Linux: 

#### **Debian-based:**
- Ubuntu 24.04 LTS or Ubuntu 25.04 or later
- Debian 13 or higher


#### **RedHat-based:**

- Fedora 42 or higher
- RHEL 9 or higher
- "RHEL-Clone (Oracle Linux, AlmaLinux, etc..)" compatible with RHEL 9 or higher

#### **Other Linux Distributions:**

- Alpine Linux
- Arch Linux
- openSUSE Tumbleweed
- openSUSE Leap 15 or higher.

### Java:

Supported Java Runtimes:

- Oracle Java
- OpenJDK
- Zulu Java

## Installation

### Windows

#### Downloading and Installing Java
- Visit the official JDK website: [https://jdk.java.net/](https://jdk.java.net/)
- Download the ZIP archive for Windows.
- Unzip the downloaded archive to a safe and convenient location on your computer (eg: `C:\Java\jdk?`).
- Locate the `java.exe` file inside the extracted folder (typically in the `bin` directory).
- Copy the full path to the `bin` folder.
- Add this path to your system's **PATH** environment variable:
  - Open the **Start Menu** and search for `Environment Variables`.
  - Click **"Edit the system environment variables"**.
  - In the **System Properties** window, click **"Environment Variables..."**.
  - Under **System Variables**, select **Path** and click **Edit**.
  - Click **New** and paste the path to the `bin` directory.
  - Click **OK** to apply the changes.

- Open **Command Prompt**.
- Run the following command:

  ```bash
  java -version
  ```

- If Java is installed correctly, you’ll see the version information displayed.

- If the command doesn't work right away, try restarting your computer and run the command again.

This App uses [delfi-ch/mc-server-downloader-py:](https://github.com/Delfi-CH/mc-server-downloader-py) to download the server.jar files, which is automaticly downloaded on startup.

### Linux

#### Downloading and Installing Java

- Install Java via your Package Manager

  **Debian and Ubuntu:**
  
   ```bash
   sudo apt update
   sudo apt upgrade
   sudo apt install default-jdk
  ```

  **Fedora, RHEL and "RHEL-Clones":**
  
   ```bash
  sudo dnf update
  sudo dnf install java
  ```

  **Arch Linux:**
  
   ```bash
  sudo pacman -Syu jdk-openjdk
  ```


  **Alpine Linux:**
  
   ```bash
   sudo apk update
   sudo apk upgrade
   sudo apk search openjdk
   sudo apk add openjdk[PREFERED-VERSION-HERE]
  ```

  **openSUSE:**
  
   ```bash
  sudo zypper update
  sudo zypper refresh
  sudo zypper search openjdk
  sudo zypper install java-[PREFERED-VERSION-HERE]-openjdk
  ```


  **Verify the Installation by running**
    ```bash
  java -version
  ```

This App uses [delfi-ch/mc-server-downloader-py:](https://github.com/Delfi-CH/mc-server-downloader-py) to download the server.jar files, which is automaticly downloaded on startup.

## Minecraft Support

### Game Versions

ONLY MINECRAFT: JAVA EDITION IS SUPPORTED!

- Version 1.21.x
    - 1.21.6
    - 1.21.5
    - 25w14craftmine (April Fools)
    - 1.21.4
    - 1.21.3
    - 1.21.1
- Minecraft 1.20.x
  - 1.20.6
  - 24w14potato (April Fools)
  - 1.20.4
  - 1.20.2
  - 1.20.1 (pre Villager Rebalancing)
  - 23w13a_or_b (April Fools)
- Minecraft 1.19.x
  - 1.19.4
  - 22w13oneBlockAtATime (April Fools)
- Minecraft 1.18.x
  - 1.18.2
- Minecraft 1.17.x
  - 1.17.1
- Minecraft 1.16.x
  - 1.16.5
  - 1.16.2
  - 1.16.1
  - 20w14infinite (April Fools)
- Minecraft 1.15.x
  - 1.15.2
- Minecraft 1.14.x
  - 1.14.4
- Minecraft 1.13.x
  - 1.13.2
- Minecraft 1.12.x
  - 1.12.2
- Minecraft 1.9.x - 1.11.x
  - 1.11.2
  - 1.10.2
  - 1.9.4
  - 1.RV-Pre1 (April Fools)
- Minecraft 1.7.x - 1.8.x
  - 1.8.9
  - 15w14a (April Fools)
  - 1.7.10

- Minecraft 1.0.x - 1.7.x
    - We dont support these versions.

### Modloaders

- Forge:
  - Forge is a open-source Minecraft Modding API and Modloader. It has existed for a long time and is compatible with most Minecraft versions. 
  - Supports: Minecraft 1.7.10 - 1.21.6
- NeoForge:
  - NeoForge is a fork of Forge, developed by most of the Team behind Forge. It aims for better features for Mod-Developers and only supports newer Versions of Minecraft.
  - Supports: Minecraft 1.20.2 - 1.21.6
- Fabric
  - Fabric is a open-source Modloader. It is more lightweight and modular than Forge and supports many more recent versions of the Game.
  - Supports: Minecraft 1.14.4 - 1.21.6
- PaperMC
  - Paper is a modified Server-Software which aims to improve Performance and introduces a Plugin API. It is compatible with most Versions of the game and doesnt require a modified Client.
  - Supports: Minecraft 1.9.4 - 1.21.5
- Folia
  - Folia is a fork of Paper, developed by mostly the same Team behind Paper, which adds Multithreading for improved Performance.
  - Supports: Minecraft 1.19.4 - 1.21.6

Note that Versions labled "April Fools" cannot be run with any modloaders.
## Configuration

For configuring the app, TOML (Tom's Obvious Minimal Language) is beeing used. You can read more about it [here](https://toml.io/)

## FAQ

- Q: I have Java installed, but i get something like "Java wasn't found or is missing!"
- A: Check if Java is in your PATH variable. You can do this by running ```java -version```in a Terminal
- Q: Is this affiliated or endorsed by Mojang / Microsoft?
- A: No, this Software is not affiliated, endorsed or connected to Mojang  and/or Microsoft.


## Compiling it yourself

See [here](https://github.com/Delfi-CH/mc-server-management/blob/main/COMPILING.md)

## Legal

THIS SOFTWARE IS NOT AFFILIATED OR ENDORSED WITH MOJANG AB OR MICROSOFT.

This program is licensed under the terms of the GNU General Public License Version 3 (GPLv3).
For more information, please visit https://www.gnu.org/licenses/gpl-3.0
However, this program can download and execute the proprietary licensed Minecraft server.jar via seperate processes.
These functions require agreeing to the Minecraft End User License Agreement (EULA).
For more information, please visit https://www.minecraft.net/en-us/eula.
