#!/usr/bin/env bash
# ==============================================================================
# DhabaTUI - Automated Release Preparation Script
#
# Usage:
#   ./scripts/prepare-release.sh --type [patch|minor|major_lts] [--dry-run] [--version X.Y.Z]
# ==============================================================================

set -euo pipefail

RELEASE_TYPE="patch"
DRY_RUN="false"
CUSTOM_VERSION=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --type)
      RELEASE_TYPE="$2"
      shift 2
      ;;
    --dry-run)
      DRY_RUN="true"
      shift
      ;;
    --version)
      CUSTOM_VERSION="$2"
      shift 2
      ;;
    *)
      echo "Unknown argument: $1" >&2
      exit 1
      ;;
  esac
done

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

# Determine current latest tag
LATEST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "v0.1.0")
echo "Latest detected tag: ${LATEST_TAG}"

if [ -n "$CUSTOM_VERSION" ]; then
  NEW_VERSION="${CUSTOM_VERSION#v}"
else
  CURRENT_SEMVER="${LATEST_TAG#v}"
  IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT_SEMVER"
  MAJOR="${MAJOR:-0}"
  MINOR="${MINOR:-1}"
  PATCH="${PATCH:-0}"

  case "$RELEASE_TYPE" in
    patch)
      NEW_VERSION="${MAJOR}.${MINOR}.$((PATCH + 1))"
      RELEASE_TITLE="Weekly Patch Release"
      ;;
    minor)
      NEW_VERSION="${MAJOR}.$((MINOR + 1)).0"
      RELEASE_TITLE="Monthly Feature Release"
      ;;
    major_lts)
      NEW_VERSION="$((MAJOR + 1)).0.0"
      RELEASE_TITLE="Yearly LTS Release"
      ;;
    *)
      echo "Invalid release type: $RELEASE_TYPE. Expected patch, minor, or major_lts." >&2
      exit 1
      ;;
  esac
fi

NEW_TAG="v${NEW_VERSION}"
TODAY=$(date -u +%Y-%m-%d)

echo "=============================================="
echo " Preparing DhabaTUI Release: ${NEW_TAG} (${RELEASE_TITLE:-Manual Release})"
echo " Date: ${TODAY}"
echo " Dry Run: ${DRY_RUN}"
echo "=============================================="

# Collect commits since last tag
COMMITS=$(git log "${LATEST_TAG}..HEAD" --oneline --no-merges 2>/dev/null || echo "")

if [ -z "$COMMITS" ] && [ "$DRY_RUN" = "false" ] && [ -z "$CUSTOM_VERSION" ]; then
  echo "No new commits since ${LATEST_TAG}. Skipping release."
  exit 0
fi

# Categorize commits
FEATS=$(echo "$COMMITS" | grep -iE '^[a-f0-9]+ feat' | grep -ivE '\((android|ios|mobile)\)' || true)
FIXES=$(echo "$COMMITS" | grep -iE '^[a-f0-9]+ fix' | grep -ivE '\((android|ios|mobile)\)' || true)
MOBILE=$(echo "$COMMITS" | grep -iE '^[a-f0-9]+ (ci|feat|fix)\((android|ios|mobile)\)' || true)
CIS=$(echo "$COMMITS" | grep -iE '^[a-f0-9]+ (ci|chore|refactor)' | grep -ivE 'mobile|android|ios' || true)
DOCS=$(echo "$COMMITS" | grep -iE '^[a-f0-9]+ docs' || true)
OTHERS=$(echo "$COMMITS" | grep -ivE 'feat|fix|ci\(|chore\(|refactor\(|docs\(' || true)

# Generate Markdown Notes
NOTES=""
if [ -n "$FEATS" ]; then
  NOTES+=$'\n### 🚀 Features\n'
  while IFS= read -r line; do
    [ -n "$line" ] && NOTES+="- ${line}"$'\n'
  done <<< "$FEATS"
fi

if [ -n "$FIXES" ]; then
  NOTES+=$'\n### 🐛 Bug Fixes\n'
  while IFS= read -r line; do
    [ -n "$line" ] && NOTES+="- ${line}"$'\n'
  done <<< "$FIXES"
fi

if [ -n "$MOBILE" ]; then
  NOTES+=$'\n### 📱 Mobile (Android / iOS)\n'
  while IFS= read -r line; do
    [ -n "$line" ] && NOTES+="- ${line}"$'\n'
  done <<< "$MOBILE"
fi

if [ -n "$CIS" ]; then
  NOTES+=$'\n### ⚙️ CI / Automation\n'
  while IFS= read -r line; do
    [ -n "$line" ] && NOTES+="- ${line}"$'\n'
  done <<< "$CIS"
fi

if [ -n "$DOCS" ]; then
  NOTES+=$'\n### 📝 Documentation\n'
  while IFS= read -r line; do
    [ -n "$line" ] && NOTES+="- ${line}"$'\n'
  done <<< "$DOCS"
fi

if [ -n "$OTHERS" ]; then
  NOTES+=$'\n### 🔍 Other Changes\n'
  while IFS= read -r line; do
    [ -n "$line" ] && NOTES+="- ${line}"$'\n'
  done <<< "$OTHERS"
fi

if [ -z "$NOTES" ]; then
  NOTES+=$'\n- Maintenance release and dependency updates.\n'
fi

RELEASE_BLOCK="## [${NEW_VERSION}] - ${TODAY} - ${RELEASE_TITLE:-Release}
${NOTES}
"

echo "Generated Release Notes:"
echo "----------------------------------------"
echo "$RELEASE_BLOCK"
echo "----------------------------------------"

if [ "$DRY_RUN" = "true" ]; then
  echo "Dry run completed. No files modified or committed."
  exit 0
fi

# 1. Update Cargo.toml (scoped to [package] table)
python3 -c "
import re
with open('Cargo.toml', 'r') as f:
    c = f.read()
new_c = re.sub(r'(\[package\][^\[]*?version\s*=\s*\")[^\"]+(\")', r'\g<1>${NEW_VERSION}\g<2>', c, count=1)
with open('Cargo.toml', 'w') as f:
    f.write(new_c)
" 2>/dev/null || sed -i -E "0,/^version = \"[0-9]+\.[0-9]+\.[0-9]+\"/s//version = \"${NEW_VERSION}\"/" Cargo.toml
echo "Updated Cargo.toml to version ${NEW_VERSION}"

# 2. Synchronize Cargo.lock
cargo check --quiet
echo "Synchronized Cargo.lock"

# 3. Prepend release notes to CHANGELOG.md
if [ -f CHANGELOG.md ]; then
  # Insert right after the header block (after line containing "## [Unreleased]")
  if grep -q "## \[Unreleased\]" CHANGELOG.md; then
    awk -v block="$RELEASE_BLOCK" '
      /## \[Unreleased\]/ {
        print;
        print "";
        print block;
        next;
      }
      { print }
    ' CHANGELOG.md > CHANGELOG.md.tmp && mv CHANGELOG.md.tmp CHANGELOG.md
  else
    printf "# Changelog\n\n%s\n%s" "$RELEASE_BLOCK" "$(cat CHANGELOG.md)" > CHANGELOG.md
  fi
else
  cat <<EOF > CHANGELOG.md
# Changelog

All notable changes to DhabaTUI are documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

$RELEASE_BLOCK
EOF
fi
echo "Updated CHANGELOG.md"

echo "Release ${NEW_TAG} successfully prepared!"
