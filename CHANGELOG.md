# v0.1.1

This release makes packfile generation deterministic by setting the committed and authored time to the unix epoch. Thanks @david-monroe for the contribution.

A small bugfix is also included for blobs under 16 bytes emitting an invalid file size in the serialised packfile.

# v0.1.0

Initial release
