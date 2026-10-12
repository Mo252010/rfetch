class Rfetch < Formula
  desc "A fast, live system info TUI with a built-in config editor"
  homepage "https://github.com/Mo252010/rfetch"
  url "https://github.com/Mo252010/rfetch/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "6f00ae86af48235fdb791f47b61bbc945d66d6e40713a6671bff019d4e33bb66"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--root", libexec, "--path", "."
    bin.install libexec/"bin/rfetch"
  end

  test do
    assert_match "rfetch", shell_output("#{bin}/rfetch --version")
  end
end