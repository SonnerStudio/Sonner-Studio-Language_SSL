# SSL v8.0 Homebrew Formula
class Ssl < Formula
  desc "SSL v8.0 - The Ultimate Programming Language"
  homepage "https://github.com/SonnerStudio/SSL-v8"
  version "8.0.0"
  license "Apache-2.0"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/SonnerStudio/SSL-v8/releases/download/v8.0.0/ssl-macos-x64"
      sha256 "SHA256_CHECKSUM_PLACEHOLDER_X64"
    elsif Hardware::CPU.arm?
      url "https://github.com/SonnerStudio/SSL-v8/releases/download/v8.0.0/ssl-macos-arm64"
      sha256 "SHA256_CHECKSUM_PLACEHOLDER_ARM64"
    end
  end

  on_linux do
    url "https://github.com/SonnerStudio/SSL-v8/releases/download/v8.0.0/ssl-linux-x64"
    sha256 "8342c3666b46d32864aea4029f42599011d5c91ce9a8d7ba464f7fffd21a6ed6"
  end

  def install
    # Determine the downloaded binary name
    if OS.mac?
      if Hardware::CPU.intel?
        binary_name = "ssl-macos-x64"
      else
        binary_name = "ssl-macos-arm64"
      end
    else
      binary_name = "ssl-linux-x64"
    end

    # Install the binary
    bin.install binary_name => "ssl"
  end

  test do
    # Test that ssl runs and outputs version
    assert_match "SSL v8.0.0", shell_output("#{bin}/ssl --version")
  end

  def caveats
    <<~EOS
      SSL v8.0 has been installed successfully!

      Features:
      - 37 stdlib modules
      - 160+ features
      - 16 NLP languages
      - 3D graphics engine
      - Blockchain support
      - Quantum computing

      Get started:
        ssl --help          Show help
        ssl --version       Show version
        ssl run file.ssl    Run a program

      Documentation: https://github.com/SonnerStudio/SSL-v8
    EOS
  end
end
