# Glossary

## AEAD (Authenticated Encryption with Associated Data)

A cryptographic scheme that provides both confidentiality and authenticity. ChaCha20Poly1305 is an AEAD cipher that encrypts data while also providing authentication to detect tampering.

## Argon2id

A memory-hard password hashing function and winner of the Password Hashing Competition. It's designed to be resistant to both GPU and ASIC attacks by requiring significant memory usage.

## ASIC (Application-Specific Integrated Circuit)

Specialized hardware designed for specific tasks. In cryptography, ASICs can be used to accelerate password cracking, which is why Argon2 is designed to be ASIC-resistant.

## Authentication Tag

A cryptographic value that verifies the integrity and authenticity of encrypted data. In ChaCha20Poly1305, this is a 128-bit tag that detects any tampering with the ciphertext.

## ChaCha20Poly1305

A modern authenticated encryption algorithm combining the ChaCha20 stream cipher with the Poly1305 message authentication code. It's faster than AES on systems without hardware AES acceleration.

## Checksum

A small-sized hash or value used to verify the integrity of data. It ensures that data has not been tampered with or corrupted during storage or transmission.

## CI/CD

Stands for Continuous Integration and Continuous Deployment/Delivery. In the context of a Rust secure notes app, CI/CD automates testing, building, and deploying updates to ensure code reliability and fast delivery.

## Ciphertext

The encrypted form of data that cannot be read without decryption. In the app, notes are converted to ciphertext using an encryption key before being stored on disk.

## Deterministic

A process that always produces the same output given the same input. Used in hardware fingerprinting to ensure consistent results across application runs.

## Hardware Fingerprinting

A technique that creates a unique identifier based on hardware and system characteristics. Used to bind encrypted data to specific machines for additional security.

## Hash

A deterministic output of a hash function, producing a fixed-size value from arbitrary input. Hashes are used for verifying integrity, storing passwords securely, and comparing data without revealing the original input.

## Key Derivation

A cryptographic process that generates a strong encryption key from a password or passphrase. Typically used with algorithms like PBKDF2, Argon2, or scrypt to protect against brute-force attacks.

## Lightweight

Describes a program or library with minimal resource usage (e.g., memory, CPU). A lightweight secure notes app in Rust would be fast, efficient, and suitable for low-power or embedded environments.

## MAC (Message Authentication Code)

A cryptographic checksum that verifies both the integrity and authenticity of a message. Poly1305 is the MAC component in ChaCha20Poly1305.

## Memory-hard

A cryptographic property where the algorithm requires significant memory to execute, making it expensive to attack with specialized hardware. Argon2 is memory-hard.

## Metadata

Data that provides information about other data. In the app, this includes timestamps, hardware fingerprints, and encryption version information.

## Nonce

A "number used once" in cryptography to ensure that encryption results are unique each time. Used in encryption schemes like AES-GCM to prevent replay attacks and ensure data security.

## Obfuscation

The practice of making data or code difficult to understand or analyze. The app uses fake SQLite headers to disguise encrypted files.

## Parallelism

The ability to perform multiple operations simultaneously. Argon2 can use multiple CPU threads to increase security while maintaining reasonable performance.

## Password Hashing

The process of converting a password into a fixed-size string (hash) using a cryptographic hash function. In a secure notes app, this is used to securely store and verify user passwords without keeping them in plain text.

## PBKDF2 (Password-Based Key Derivation Function 2)

An older key derivation function that applies a hash function multiple times to derive keys from passwords. Less secure than Argon2 against modern attacks.

## Poly1305

A cryptographic message authentication code (MAC) designed by Daniel J. Bernstein. It's used with ChaCha20 to provide authenticated encryption.

## Replay Attack

A security attack where valid data transmission is maliciously repeated. Nonces prevent replay attacks by ensuring each encryption is unique.

## Salt

A random value added to passwords before hashing to ensure unique hashes for identical passwords. This prevents precomputed hash attacks (e.g., rainbow tables).

## Semantic Security

A cryptographic property where identical plaintexts produce different ciphertexts when encrypted multiple times. Achieved through random nonces.

## Serialization

The process of converting data structures (e.g., Rust structs) into a format that can be stored or transmitted, such as JSON, TOML, or binary. Used in the app to save and load notes securely.

## Stream Cipher

A type of encryption that encrypts data one bit or byte at a time. ChaCha20 is a stream cipher that's faster than block ciphers like AES in software.

## System Keyring

A secure storage mechanism provided by the operating system for storing secrets such as passwords or keys. The secure notes app can optionally use the system keyring to store encryption keys safely.

## Thread Safety

The property of code that can be safely executed by multiple threads simultaneously without data corruption. The app uses message passing for thread safety.

## Timing Attack

A security attack that analyzes the time taken to execute cryptographic operations to extract secret information. Argon2 is designed to be resistant to timing attacks.

## UUID (Universally Unique Identifier)

A 128-bit identifier that's unique across space and time. The app uses UUID v4 for note IDs to prevent conflicts between users.
