# Homebrew Formula for kasmctl

## Setting Up a Homebrew Tap

1. Create a GitHub repository named `homebrew-tap` (e.g. `PhilipCramer/homebrew-tap`).

2. Copy the formula into the tap repository:

   ```sh
   cp kasmctl.rb /path/to/homebrew-tap/Formula/kasmctl.rb
   ```

3. Update the `sha256` values in the formula with the actual checksums from the release artifacts:

   ```sh
   shasum -a 256 kasmctl-darwin-amd64.tar.gz
   shasum -a 256 kasmctl-darwin-arm64.tar.gz
   shasum -a 256 kasmctl-linux-amd64.tar.gz
   ```

4. Commit and push the tap repository.

## Installing via Homebrew

```sh
brew tap PhilipCramer/tap
brew install kasmctl
```

## Updating the Formula

When releasing a new version:

1. Update the `version` field in `kasmctl.rb`.
2. Update the `sha256` values with checksums of the new release artifacts.
3. Commit and push to the tap repository.
