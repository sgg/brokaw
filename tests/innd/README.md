# inndocker

A containerized [INN](https://www.eyrie.org/~eagle/software/inn/) for integration testing!

The [`root`](./root) directory contains a filesystem tree that will be copied into the container.

## Running

`innd` is hooked up with PAM and a test user `newsreader` with password `readthenews` is available.
Note that `innd` only seems to adhere to `SIGQUIT` so you must kill it with (CTRL + \)

1. Start the server

    ```shell script
    ❯ docker build -t sgg/innd . \
       && docker run -it -rm --name innd -p 119:119 sgg/innd
    ```
2. Interact with it via netcat telnet (or your favorite news client)

    ```shell script
    ❯  echo "AUTHINFO USER newsreader
    AUTHINFO PASS readthenews
    CAPABILITIES
    LIST" \
    | nc localhost 119
    ```