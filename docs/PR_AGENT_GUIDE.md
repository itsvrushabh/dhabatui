# Automated Pull Request Reviews with PR-Agent 🤖

DhabaTUI uses **[PR-Agent](https://github.com/Codium-ai/pr-agent)** (by Qodo / CodiumAI) to provide automated, AI-assisted code reviews, security checks, and code suggestions directly on every pull request.

---

## ⚡ How It Works

Whenever a Pull Request is **opened**, **reopened**, or **updated (`synchronize`)**, the GitHub Actions workflow in [`.github/workflows/pr-agent.yml`](../.github/workflows/pr-agent.yml) triggers PR-Agent to analyze the changes.

### Automated Actions on Every PR:
1. **`/describe`**: Generates an informative PR title, type tags, and structured summary.
2. **`/review`**: Analyzes the diff for potential bugs, concurrency race conditions, zero-float financial compliance, and security considerations, leaving inline review comments.
3. **`/improve`**: Proposes targeted code improvements and refactoring suggestions as ready-to-commit suggestions.

---

## 💬 Interactive Commands in PR Comments

Maintainers and contributors can interactively invoke PR-Agent by posting comments on any Pull Request:

| Command | Purpose | Example |
| :--- | :--- | :--- |
| **`/review`** | Re-run full code review and quality checks | `/review` |
| **`/describe`** | Update PR description and changelog | `/describe` |
| **`/improve`** | Suggest inline code improvements | `/improve` |
| **`/ask <question>`** | Ask a specific technical question about the PR diff | `/ask is the redb transaction properly committed?` |
| **`/generate_labels`** | Auto-categorize PR with relevant labels | `/generate_labels` |

---

## 🔐 Required Repository Secret Setup

To enable PR-Agent on your fork or repository:

1. Navigate to **Settings** ➔ **Secrets and variables** ➔ **Actions**.
2. Click **New repository secret**.
3. Add at least one of the following API keys:
   - **`OPENAI_KEY`** or **`OPENAI_API_KEY`**: OpenAI API key (GPT-4o / GPT-4o-mini).
   - **`GEMINI_API_KEY`** or **`GEMINI_KEY`**: Google AI Studio Gemini API key.
   - **`ANTHROPIC_KEY`** or **`ANTHROPIC_API_KEY`**: Anthropic Claude API key.
4. If no AI secret is configured, the workflow gracefully posts a GitHub Notice without failing the build pipeline.

---

## 🛠️ Repository Configuration (`.pr_agent.toml`)

Custom rules and constraints for this codebase are configured in [`.pr_agent.toml`](../.pr_agent.toml):
- **Zero Floating-Point Constraint**: All monetary figures, subtotals, GST taxes, and totals must strictly use integer paise (`u32` or `u64`). Never introduce `f32`/`f64`.
- **Rust Idioms & Safety**: Safe concurrency, Redb transaction commits, non-blocking UI rendering, and LAN network boundary validation.
