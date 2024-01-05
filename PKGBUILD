# Maintainer: Alexander F. Rødseth <xyproto@archlinux.org>
# Contributor: Wilhem Barbier <wilhem.b@free.fr>

pkgname=raylib
pkgver=5.0
pkgrel=1
pkgdesc='Simple and easy-to-use game programming library'
arch=(x86_64)
url='https://www.raylib.com'
license=(ZLIB)
makedepends=(cmake git libx11 libxcursor libxinerama libxrandr ninja vulkan-headers xorg-xinput)
source=("git+https://github.com/raysan5/raylib#commit=ae50bfa2cc569c0f8d5bc4315d39db64005b1b08") # tag: 5.0
b2sums=('SKIP')

build() {
  cmake \
    -B build \
    -D BUILD_EXAMPLES=OFF \
    -D CMAKE_BUILD_TYPE=Release \
    -D USE_STATIC_LIBS=ON \
    -D CMAKE_C_FLAGS="$CFLAGS -fPIC -w" \
    -D CMAKE_INSTALL_LIBDIR=lib \
    -D CMAKE_INSTALL_PREFIX=/usr \
    -D OpenGL_GL_PREFERENCE=GLVND \
    -D PLATFORM=Desktop \
    -D USE_EXTERNAL_GLFW=OFF \
    -D WITH_PIC=ON \
    -G Ninja \
    -S $pkgname \
    -W no-dev
  ninja -C build
}

package() {
  DESTDIR="$pkgdir" ninja -C build install
  # Extra include files that are used in the raylib examples
  for f in rcamera rgestures; do
    install -Dm644 $pkgname/src/$f.h "$pkgdir/usr/include/$f.h"
  done
  install -Dm644 $pkgname/LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}

# getver: github.com/raysan5/raylib/releases
