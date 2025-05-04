# C2 Framework

> [!CAUTION]
> This is malware and should be cautiously used. It is only for educational purposes and should not be used for malicious intent. Use at your own risk.

C2 (Command and Control) Framework is a tool used for post exploitation and red teaming

Consists of 3 main components:

- Server: Server that handles the communication with agents and the operator UI

- Implant: The client (malware) that runs on the target machine and communicates with the C2 server

- Operator UI: CLI to interact with the server and manage implants

## Core Features

- Support for Windows

- Command Execution: Send commands to agents and receive responses

## Additional Features

- Persistence: Maintain access to the target machine even after a reboot

- Encryption: Secure communication between the server and agents
