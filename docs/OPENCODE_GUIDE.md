# OpenCode Automated Code Review Guide 🤖

DhabaTUI integrates **[OpenCode](https://opencode.ai)** (`anomalyco/opencode`) to provide interactive, automated code reviews and agentic coding assistance directly inside GitHub Pull Requests and Issues.

---

## ⚡ How It Works

The workflow [`.github/workflows/opencode-review.yml`](../.github/workflows/opencode-review.yml) triggers on:
1. **Pull Requests**: Whenever a PR is `opened`, `reopened`, `ready_for_review`, or updated (`synchronize`).
2. **Interactive Comments**: Whenever a user mentions `/oc` or `/opencode` in an issue comment or pull request review comment.

---

## 🔐 OIDC Token Exchange Authentication

OpenCode uses GitHub's OpenID Connect (OIDC) authentication with OpenCode Console (`https://opencode.ai/console`), eliminating the need for long-lived static secrets:

```yaml
permissions:
  id-token: write
  contents: read

steps:
  - name: Sign in to OpenCode
    env:
      AUDIENCE: https://opencode.ai/console
    run: |
      oidc=$(curl -sSf --oauth2-bearer "$ACTIONS_ID_TOKEN_REQUEST_TOKEN" \
        "$ACTIONS_ID_TOKEN_REQUEST_URL&audience=$AUDIENCE" | jq -r .value)
      token=$(curl -sSf https://opencode.ai/console/auth/oauth/token \
        -d grant_type=urn:ietf:params:oauth:grant-type:token-exchange \
        -d subject_token_type=urn:ietf:params:oauth:token-type:jwt \
        -d client_id=sk_242642b3-b9a2-48cb-a5d5-b983a41fdb0f \
        -d subject_token="$oidc" | jq -r .access_token)
      echo "::add-mask::$token"
      echo "OPENCODE_API_KEY=$token" >> "$GITHUB_ENV"
```

---

## 💬 Interactive Slash Commands

Contributors and maintainers can invoke OpenCode directly on any PR by commenting:

| Command | Action | Example |
| :--- | :--- | :--- |
| **`/oc review`** | Request a focused code review of the PR | `/oc review this PR for memory leaks and race conditions` |
| **`/oc fix`** | Ask OpenCode to implement a suggested fix | `/oc fix the off-by-one error in table pagination` |
| **`/oc explain`** | Ask for an explanation of a complex diff | `/oc explain the changes in the release cycle workflow` |
| **`/opencode <prompt>`** | Full prompt to OpenCode agent | `/opencode check if paise arithmetic invariant is satisfied` |

---

## 🛡️ Review Invariants Checked
OpenCode specifically inspects:
- **Zero-Float Math**: Strictly integer paise (`u64`) for all monetary values.
- **Offline Architecture**: Ensuring core billing pathways remain 100% offline-ready.
- **ACID Reliability**: Proper transactional commit and error handling with `redb`.
- **Cleanliness & Formatting**: `cargo test`, `cargo clippy`, and `cargo fmt` standards.
