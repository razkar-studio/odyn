## Full Architecture List

|    Operating System     |  Architecture |             Status            |
|-------------------------|---------------|-------------------------------|
| **Windows**             | x86_64/AMD64  | :white_check_mark: Supported  |
|                         | i686 (32-bit) | :white_check_mark: Supported  |
|                         | aarch64       | :white_check_mark: Supported  |
| **macOS**               | aarch64       | :white_check_mark: Supported  |
|                         | x86_64        | :white_check_mark: Supported  |
| **Linux**               | x86_64        | :white_check_mark: Supported  |
|                         | x86_64 musl   | :white_check_mark: Supported  |
|                         | aarch64       | :white_check_mark: Supported  |
|                         | aarch64 musl  | :white_check_mark: Supported  |
|                         | i686          | :white_check_mark: Supported  |
|                         | RISC-V 64     | :white_check_mark: Supported  |
|                         | RISC-V 32     | :x: Not Available             |
|                         | ARMv7         | :white_check_mark: Supported  |
|                         | ARMv6         | :white_check_mark: Supported  |
|                         | ARMv7 musl    | :white_check_mark: Supported  |
|                         | ARMv6 musl    | :white_check_mark: Supported  |
|                         | i686 musl     | :white_check_mark: Supported  |
|                         | POWERPC64     | :white_check_mark: Supported  |
|                         | POWERPC64 (LE)| :white_check_mark: Supported  |
|                         | s390x         | :white_check_mark: Supported  |
|                         | SPARC64       | :white_check_mark: Supported  |
|                         | MIPS          | :x: Not Available             |
|                         | MIPSEL        | :x: Not Available             |
|                         | MIPS64        | :x: Not Available             |
|                         | MIPS64EL      | :x: Not Available             |
| **Android** (via Termux)| aarch64       | :white_check_mark: Supported  |
|                         | ARMv7         | :white_check_mark: Supported  |
|                         | x86_64        | :white_check_mark: Supported  |
| *FreeBSD*               | x86_64        | :white_check_mark: Supported  |
|                         | aarch64       | :x: Not Available             |
|                         | i686          | :white_check_mark: Supported  |
| *NetBSD*                | x86_64        | :white_check_mark: Supported  |
| *Solaris*               | SPARCv9       | :x: Not Available (duh)       |
| *Others*                | Others        | Build from source / Use Cargo |

If macOS support isn't visible in the Releases page, the GitHub runner probably failed.
