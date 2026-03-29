## Full Architecture List

|    Operating System     |  Architecture |             Status            |
|-------------------------|---------------|-------------------------------|
| **Windows**             | x86_64/AMD64  | :white_check_mark: Supported  |
|                         | i686 (32-bit) | :white_check_mark: Supported  |
|                         | aarch64       | :soon: Maybe Soon             |
| **macOS**               | aarch64       | :soon: Coming Soon            |
|                         | x86_64        | :soon: Coming Soon            |
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

Sorry for the lack of macOS support! The next release will partly use GitHub Actions which has native macOS runners, so both targets are coming in 0.2.0 and above.

For Windows ARM PCs, I'm currently waiting on GitHub Actions to add runner support there. Maybe soon.
