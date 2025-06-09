# mc-server-management

rust tool for managing multiple selfohosted minecraft servers

CURRENTLY WORK IN PROGRESS!
DO NOT USE
NOT ALL FEATURES ARE IMPLEMENTED

## Reqierments

### Processor:

x86-64 based Processor (Intel/Amd)

### Operating Systems

#### Windows:

- Windows 10
- Windows 11

#### Linux: 

#### Debian-based:
- Ubuntu 24.04 or higher
- Debian 12 or higher


#### RedHat-based

- Fedora 42 or higher
- RHEL 9 or higher
- "RHEL-Clone (Oracle Linux, AlmaLinux, etc..)" compatible with RHEL 9 or higher

#### Other Linux Distributions:

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
  Verify the Installation by running
  ```bash
  java -version
  ```
  **Fedora, RHEL and "RHEL-Clones":**
  
   ```bash
  sudo dnf update
  sudo dnf install java
  ```
  Verify the Installation by running
    ```bash
  java -version
  ```
  **Arch  Linux:**
  
   ```bash
  pacman -Syu jdk-openjdk
  ```
  Verify the Installation by running
    ```bash
  java -version
  ```

  **Arch  Linux:**
  
   ```bash
  pacman -Syu jdk-openjdk
  ```
  Verify the Installation by running
    ```bash
  java -version
  ```
  **openSUSE:**
  
   ```bash
  add stuff here
  ```
  Verify the Installation by running
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

If its not, add it.
    
  On Windows: [How to edit Enviroment Variables](https://www.google.com/search?q=how+to+edit+environment+variables+in+windows&sca_esv=87b41ab4477c98ab&sxsrf=AE3TifOTess3olJYauutliwMR8rU_Ub-yA%3A1749203581729&source=hp&ei=fbpCaKWsKrSI9u8Pr5_muQ8&iflsig=AOw8s4IAAAAAaELIjZ9Uqyh4psKbgFDI_4vq5JbJ5PAs&ved=0ahUKEwjl3oGRw9yNAxU0hP0HHa-POfcQ4dUDCBk&uact=5&oq=how+to+edit+environment+variables+in+windows&gs_lp=Egdnd3Mtd2l6Iixob3cgdG8gZWRpdCBlbnZpcm9ubWVudCB2YXJpYWJsZXMgaW4gd2luZG93czIIEAAYgAQYywEyCBAAGIAEGMsBMggQABiABBjLATIGEAAYFhgeMgYQABgWGB4yBhAAGBYYHjIGEAAYFhgeMgUQABjvBTIIEAAYgAQYogQyBRAAGO8FSJoDUABYAHAAeACQAQCYAUCgAUCqAQExuAEDyAEA-AEC-AEBmAIBoAJImAMAkgcBMaAHmQeyBwExuAdIwgcDMy0xyAcI&sclient=gws-wiz)

  Open a new Terminal

  On Linux, run:
    
  ```bash
  java -version
  ```

  If it returns a valid Path, please make an Issue

  If it doenst (re-) install Java
