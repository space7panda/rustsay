class Rustsay < Formula
  desc "A simple CLI tool in Rust that mimics the classic cowsay program"
  homepage "https://github.com/space7panda/rustsay"
  url "https://github.com/space7panda/rustsay/archive/refs/tags/0.1.4.tar.gz"
  sha256 "1361089c766b0fe7ee47e4fd31eb0b1fd8c7189eef9f103b01f19551adb1819f"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "rustsay", shell_output("#{bin}/rustsay --version")
  end
end
