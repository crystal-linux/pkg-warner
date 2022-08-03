# Maintainer: Michal S <michal[at]tar[dot]black>
# Developer:  Michal S <michal[at]tar[dot]black>

pkgname=pkg-warner
pkgver=0.1.2
pkgrel=1
pkgdesc="Simple package manager warner tool for distribution developers"
arch=('x86_64')
url="https://github.com/crystal-linux/pkg-warner"
license=('GPL3')
source=("git+$url?rev=v0.1.2")
sha256sums=('SKIP')
depends=('coreutils')
makedepends=('cargo')

prepare() {
    cd "$srcdir/$pkgname"
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "$srcdir/$pkgname"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
        
    # These following envvars are Crystal-specific, please adjust for your own distro!
    export PKG_WARNER_PACKAGES="apt,apt-get,dnf,pkg,rpm,yum,zypper,eopkg"
    export PKG_WARNER_DISTRO="Crystal"
    export PKG_WARNER_PMAN="ame/pacman"
    
    cargo build --frozen --release
}

package() {
    cd "$srcdir/$pkgname"
    cargo run --frozen --release -- -ivd "${pkgdir}/usr/bin"
}