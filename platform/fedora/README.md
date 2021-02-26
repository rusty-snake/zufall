```terminal
$ sudo dnf install cargo gtk3-devel rpm-build rpmdevtools
$ cargo vendor
$ mkdir .cargo
$ cat > .cargo/config <<__EOF__
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
__EOF__
$ rm -rf ~/rpmbuild
$ rpmdev-setuptree
$ tar --exclude=".git" --exclude=target -czf ~/rpmbuild/SOURCES/zufall.tar.gz .
$ cp platform/fedora/zufall.spec ~/rpmbuild/SPECS
$ cd ~/rpmbuild/SPECS
$ rpmbuild -bb zufall.spec
```
