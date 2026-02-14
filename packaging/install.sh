#!/bin/sh
# kasmctl installer
#
# Usage:
#   curl -sSL https://raw.githubusercontent.com/PhilipCramer/kasmctl/main/packaging/install.sh | sh
#
# Options (environment variables):
#   KASMCTL_VERSION   Version to install (default: latest)
#   KASMCTL_INSTALL   Install directory (default: /usr/local/bin)

set -eu

REPO="PhilipCramer/kasmctl"
VERSION="${KASMCTL_VERSION:-latest}"
INSTALL_DIR="${KASMCTL_INSTALL:-/usr/local/bin}"

log() {
    printf '%s\n' "$@"
}

err() {
    printf 'Error: %s\n' "$@" >&2
    exit 1
}

detect_os() {
    case "$(uname -s)" in
        Linux*)  echo "linux" ;;
        Darwin*) echo "darwin" ;;
        *)       err "Unsupported operating system: $(uname -s)" ;;
    esac
}

detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64)   echo "amd64" ;;
        aarch64|arm64)  echo "arm64" ;;
        *)              err "Unsupported architecture: $(uname -m)" ;;
    esac
}

has_cmd() {
    command -v "$1" >/dev/null 2>&1
}

download() {
    if has_cmd curl; then
        curl -fsSL -o "$2" "$1"
    elif has_cmd wget; then
        wget -qO "$2" "$1"
    else
        err "Neither curl nor wget found. Please install one and retry."
    fi
}

resolve_version() {
    if [ "$VERSION" = "latest" ]; then
        url="https://github.com/${REPO}/releases/latest"
        if has_cmd curl; then
            VERSION=$(curl -fsSLI -o /dev/null -w '%{url_effective}' "$url" | rev | cut -d'/' -f1 | rev)
        elif has_cmd wget; then
            VERSION=$(wget -qO- --server-response "$url" 2>&1 | grep -i "location:" | tail -1 | rev | cut -d'/' -f1 | rev | tr -d '\r')
        fi
        if [ -z "$VERSION" ] || [ "$VERSION" = "latest" ]; then
            err "Could not determine latest version. Set KASMCTL_VERSION explicitly."
        fi
    fi
    # Strip leading 'v' for consistency, then add it back for the URL
    VERSION="${VERSION#v}"
}

main() {
    os="$(detect_os)"
    arch="$(detect_arch)"

    log "Detecting platform... ${os}/${arch}"

    resolve_version

    log "Installing kasmctl v${VERSION}..."

    archive="kasmctl-${os}-${arch}.tar.gz"
    base_url="https://github.com/${REPO}/releases/download/v${VERSION}"
    archive_url="${base_url}/${archive}"
    checksum_url="${base_url}/checksums-sha256.txt"

    tmpdir="$(mktemp -d)"
    trap 'rm -rf "$tmpdir"' EXIT

    log "Downloading ${archive_url}"
    download "$archive_url" "${tmpdir}/${archive}"

    # Verify checksum if checksums-sha256.txt is available
    if download "$checksum_url" "${tmpdir}/checksums-sha256.txt" 2>/dev/null; then
        log "Verifying checksum..."
        expected=$(grep "${archive}" "${tmpdir}/checksums-sha256.txt" | awk '{print $1}')
        if [ -n "$expected" ]; then
            if has_cmd sha256sum; then
                actual=$(sha256sum "${tmpdir}/${archive}" | awk '{print $1}')
            elif has_cmd shasum; then
                actual=$(shasum -a 256 "${tmpdir}/${archive}" | awk '{print $1}')
            else
                log "Warning: No sha256 tool found, skipping checksum verification"
                actual="$expected"
            fi
            if [ "$actual" != "$expected" ]; then
                err "Checksum mismatch: expected ${expected}, got ${actual}"
            fi
            log "Checksum verified."
        fi
    else
        log "Warning: checksums-sha256.txt not available, skipping verification"
    fi

    tar -xzf "${tmpdir}/${archive}" -C "${tmpdir}"

    if [ ! -f "${tmpdir}/kasmctl" ]; then
        err "Binary not found in archive"
    fi

    chmod +x "${tmpdir}/kasmctl"

    if [ -w "$INSTALL_DIR" ]; then
        mv "${tmpdir}/kasmctl" "${INSTALL_DIR}/kasmctl"
    else
        log "Installing to ${INSTALL_DIR} (may require sudo)..."
        sudo mv "${tmpdir}/kasmctl" "${INSTALL_DIR}/kasmctl"
    fi

    log "kasmctl v${VERSION} installed to ${INSTALL_DIR}/kasmctl"
}

main
