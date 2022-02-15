# vsc-ssh-opener

Using `vsc-ssh-opener`, you can `code .` in SSH!

## Usage

1. Build server binary in Local

    Clone this repository and run as follows to build the server binary at root of the repository:
    ```bash
    $ cargo build --release
    ```

2. Set up the config file in Local

    Here is the path to the config file:
    - in Linux `/home/your-user/.config/code-open-server/table.json`
    - in Windows `C:\Users\your-user\AppData\Roaming\code-open-server\table.json`

    Write the mapping from the hostname to name in `.ssh/config` to `table.json`.
    For example:
    ```json
    {
        "hostname": "ssh-host"
    }

    ```

3. Run `code-open-server` in Local

    Run the following in the repository root:

    ```bash
    $ ./target/release/code-open-server
    ```

4. SSH with port forwarding

    ```bash
    $ ssh -R 3000:localhost:3000 ssh-host
    ```

5. Build and Install client binary in Remote

    ```bash
    $ cargo install --git https://github.com/azarashi2931/vsc_ssh_opener code-open
    ```

Now, you can open the local VSCode by running the following somewhere remotely!:

```bash
$ code-open .
```

