Name:           janitor
Version:        0.1.0
Release:        1%{?dist}
Summary:        CCleaner-style Linux cleanup tool
License:        MIT
Source0:        %{name}-%{version}.tar.gz
BuildArch:      x86_64

%description
janitor finds and removes temp/dump/cache/log files, oversized files, and broken symlinks.

%prep
%setup -q

%build
cargo build --release --offline

%install
install -m 0755 -D target/release/janitor %{buildroot}%{_bindir}/janitor

%files
%{_bindir}/janitor

%changelog
* Tue Jan 01 2025 packager <packager@example.com> - 0.1.0-1
- initial package
