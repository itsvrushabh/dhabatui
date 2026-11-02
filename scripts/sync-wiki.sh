#!/usr/bin/env bash
# ==============================================================================
# DhabaTUI - Wiki Sync Script
#
# Usage:
#   ./scripts/sync-wiki.sh [--repo <url>]
# ==============================================================================

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WIKI_DIR="${REPO_ROOT}/wiki"
WIKI_REMOTE="https://github.com/itsvrushabh/dhabatui.wiki.git"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --repo)
      WIKI_REMOTE="$2"
      shift 2
      ;;
    *)
      WIKI_REMOTE="$1"
      shift
      ;;
  esac
done

TEMP_DIR=$(mktemp -d)

trap 'rm -rf "$TEMP_DIR"' EXIT

echo "Syncing DhabaTUI Wiki to: ${WIKI_REMOTE}"

if [ ! -d "$WIKI_DIR" ]; then
  echo "Error: Wiki source directory '${WIKI_DIR}' not found." >&2
  exit 1
fi

# Clone wiki repo or initialize if empty
if git clone "$WIKI_REMOTE" "$TEMP_DIR" 2>/dev/null; then
  echo "Successfully cloned wiki repository."
else
  echo "Wiki repository not initialized yet or authentication required."
  echo "Initializing local git repository for initial push..."
  cd "$TEMP_DIR"
  git init -b master
  git remote add origin "$WIKI_REMOTE"
fi

# Copy all markdown files from wiki/ into temp directory
cp -r "${WIKI_DIR}"/* "$TEMP_DIR/"

cd "$TEMP_DIR"
git config user.name "github-actions[bot]"
git config user.email "github-actions[bot]@users.noreply.github.com"

git add -A
if git diff --staged --quiet; then
  echo "Wiki is already up to date. No changes to push."
else
  git commit -m "docs(wiki): update wiki documentation from main repository"
  git push -u origin master || git push -u origin main
  echo "Wiki documentation successfully synced!"
fi
