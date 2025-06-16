# mc-server-management

rust tool for managing multiple selfohosted minecraft servers

CURRENTLY WORK IN PROGRESS!
DO NOT USE
NOT ALL FEATURES ARE IMPLEMENTED

## Requirements

### Processor:

x86-64 based Processor (Intel/Amd)

### RAM:

Minimum Amount of RAM needed to run the Operating System
(eg. Fedora 42 -> 2 Gigabytes) plus Minimum Amount of RAM needed for Minecraft: Java Edition Server -> 1 Gigabyte

Minimum: 4 Gigabytes

Recommended: 8 Gigabytes and more.

### Operating Systems

#### Windows:

- Windows 10
- Windows 11

#### Linux: 

#### **Debian-based:**
- Ubuntu 24.04 or higher
- Debian 12 or higher


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
- Visit the official JDK 24 website: [https://jdk.java.net/24/](https://jdk.java.net/24/)
- Download the ZIP archive for Windows.
- Unzip the downloaded archive to a safe and convenient location on your computer (eg:  `C:\Java\jdk-24`).
- Locate the `java.exe` file inside the extracted folder (typically in the `bin` directory).
- Copy the full path to the `bin` folder.
- Add this path to your system's **PATH** environment variable:
  - Open the **Start Menu** and search for `Environment Variables`.
  - Click **"Edit the system environment variables"**.
  - In the **System Properties** window, click **"Environment Variables..."**.
  - Under **System Variables**, select **Path** and click **Edit**.
  - Click **New** and paste the path to the `bin` directory.
  - Click **OK** to apply the changes.

- Open **Command Prompt** (or any terminal).
- Run the following command:

  ```bash
  java -version
  ```

- If Java is installed correctly, you’ll see the version information displayed.

- If the command doesn't work right away, try restarting your computer and run the command again.

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

  **Arch  Linux:**
  
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
  sudo zypper refresh
  sudo zypper search openjdk
  sudo zypper install java-[PREFERED-VERSION-HERE]-openjdk
  ```


  **Verify the Installation by running**
    ```bash
  java -version
  ```



## Exit Codes

- Exit Code 0: User terminated the Application.

## FAQ

- Q: I have Java installed, but i get "Java wasn't found or is missing!"
- A: Check if Java is in your PATH variable. You can do this by typing
    ```bash
  java -version
  ```
-   in a Terminal