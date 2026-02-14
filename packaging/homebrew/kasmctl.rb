# typed: false
# frozen_string_literal: true

class Kasmctl < Formula
  desc "Command-line tool for managing Kasm Workspaces"
  homepage "https://github.com/PhilipCramer/kasmctl"
  version "0.1.0"
  license "Apache-2.0"

  on_macos do
    on_intel do
      url "https://github.com/PhilipCramer/kasmctl/releases/download/v#{version}/kasmctl-darwin-amd64.tar.gz"
      sha256 "PLACEHOLDER"
    end

    on_arm do
      url "https://github.com/PhilipCramer/kasmctl/releases/download/v#{version}/kasmctl-darwin-arm64.tar.gz"
      sha256 "PLACEHOLDER"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/PhilipCramer/kasmctl/releases/download/v#{version}/kasmctl-linux-amd64.tar.gz"
      sha256 "PLACEHOLDER"
    end
  end

  def install
    bin.install "kasmctl"
  end

  test do
    assert_match "kasmctl", shell_output("#{bin}/kasmctl --help")
  end
end
