# Clone

The whole repository, one line:

```sh
curl -fsSL https://cdn.mrly.net/mrlyprod.tar.gz | tar xz
```

That leaves a `mrlyprod/` directory holding every tracked file.
`clone.sh` at the repo root is the same line with a name of your
choosing:

```sh
./clone.sh            # into mrlyprod/
./clone.sh myrepo     # into myrepo/
```

## Integrity

`mrlyprod.tar.gz.sha256` sits beside the tarball:

```sh
curl -fsSL -O https://cdn.mrly.net/mrlyprod.tar.gz
curl -fsSL -O https://cdn.mrly.net/mrlyprod.tar.gz.sha256
shasum -a 256 -c mrlyprod.tar.gz.sha256
```

## What you get

Raw files. No `.git`, no history, no remote, nothing to pull.
The tarball is rebuilt on every publish and always holds the
current state, so there is no version to be behind - run the line
again and you have the new now. Real git lives on
[github](https://github.com/mrlyprod/mrlyprod).
