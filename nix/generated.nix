{ fetchurl, ... }:
let
  repo = "juliamertz/rose-pine-build";
  tag = "v0.2.0";
in
{
  x86_64-linux = fetchurl {
    url = "https://github.com/${repo}/releases/download/${tag}/rose-pine-build-x86_64-unknown-linux-gnu.tar.gz";
    sha256 = "14ab35hv29kaw8pih0fvf5xhizaw5nw55d792v6vzqijhz31bbxb";
  };
  aarch64-linux = fetchurl {
    url = "https://github.com/${repo}/releases/download/${tag}/rose-pine-build-aarch64-unknown-linux-gnu.tar.gz";
    sha256 = "19in6v78qald5wd61j8iksggbkhb9kdas70vf604rqlk0c4dqg4c";
  };
  x86_64-darwin = fetchurl {
    url = "https://github.com/${repo}/releases/download/${tag}/rose-pine-build-x86_64-apple-darwin.tar.gz";
    sha256 = "0cac6dc3sj87ydzx92j0l4jyy7jv5x0fvan2vv70aks3wldrimc6";
  };
  aarch64-darwin = fetchurl {
    url = "https://github.com/${repo}/releases/download/${tag}/rose-pine-build-aarch64-apple-darwin.tar.gz";
    sha256 = "14zx92wdzicm37mp75iyz64pwfs4z4dx091dr3mn7iwrh2177xbl";
  };
}
