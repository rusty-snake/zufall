Name:           zufall
Version:        3.2.0
Release:        1%{?dist}
Summary:        A tool to help on decisions

License:        MIT
URL:            https://gtihub.com/rusty-snake/zufall
Source0:        zufall.tar.gz

BuildRequires:  gtk3-devel
# Comment this line if you use rustup.
BuildRequiers:  cargo

%define debug_package %{nil}

%description
A tool to help on decisions.


%prep
%autosetup -c


%build
cargo build --release


%install
/usr/bin/install -p -v -Dm755 target/release/zufall "$RPM_BUILD_ROOT"/usr/bin/zufall
/usr/bin/install -p -v -Dm644 data/zufall.desktop "$RPM_BUILD_ROOT"/usr/share/applications/zufall.desktop
/usr/bin/install -p -v -Dm644 data/icon.svg.png "$RPM_BUILD_ROOT"/usr/share/pixmaps/zufall.png


%files
%license LICENSE
/usr/bin/zufall
/usr/share/applications/zufall.desktop
/usr/share/pixmaps/zufall.png
