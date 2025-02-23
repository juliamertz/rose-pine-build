{ fetchurl, ... }:
let
  repo = "juliamertz/rose-pine-build";
  tag = "v0.1.0";
in
{
  x86_64-linux = fetchurl {
    url = "https://github.com/${repo}/releases/download/${tag}/rose-pine-build-x86_64-unknown-linux-gnu.tar.gz";
    sha256 = "12rvl31lx3dllnm9r8arw7hm993yk5sb0ahbzgw2d968gqmhvwc7";
  };
  aarch64-linux = fetchurl {
    url = "https://github.com/${repo}/releases/download/${tag}/rose-pine-build-aarch64-unknown-linux-gnu.tar.gz";
    sha256 = "0jpj5rkbixk96g0b754pvqk1s2zqpyhvh9wbkm67qni41k0wpnv2";
  };
  x86_64-darwin = fetchurl {
    url = "https://github.com/${repo}/releases/download/${tag}/rose-pine-build-x86_64-apple-darwin.tar.gz";
    sha256 = "1sdx0i10zqysmsgcsgdyblpfrnh1vwxwxc7vqmaqqm7ch8pw5z94";
  };
  aarch64-darwin = fetchurl {
    url = "https://github.com/${repo}/releases/download/${tag}/rose-pine-build-aarch64-apple-darwin.tar.gz";
    sha256 = "19lxgzwswy65dl79jvp4lhpjbgjfvj93vb7skfrgh2iih6rirykk";
  };
}
