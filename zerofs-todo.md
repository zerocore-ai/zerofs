- [ ] Implement certain open flags and path flags
  - [x] OpenFlags::TRUNCATE
  - [ ] PathFlags::SYMLINK_FOLLOW

- [ ] Capability-based security
  - [ ] fs - path, read, write, delete, create (resource granularity = file, directory)

- [ ] API
  - [ ] Directory Entry API

- [ ] CLI
  - [ ] `zerofs shell` - interactive shell (`ls`, `cd`, `mkdir`, `cat`, `echo`, `rm`, `cp`, `mv`, `rmdir`)
  - [ ] `zerofs serve` - serve a filesystem over a network interface
  - [ ] `zerofs mount` - mount a filesystem from a remote address. Uses NFS
