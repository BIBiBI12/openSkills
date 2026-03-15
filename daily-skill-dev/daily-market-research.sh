#!/bin/bash
# Daily skill development automation
# Run at 3:00 AM every day

set -e

# Configuration
REPO_ROOT=$(dirname "$0")
DATE=$(date +%Y-%m-%d)
LOG_FILE="$REPO_ROOT/logs/$DATE.log"

mkdir -p "$REPO_ROOT/logs"

echo "=== Daily Skill Development Started at $(date) ===" >> "$LOG_FILE"

# Step 1: Check GitHub token
if [ -z "$GH_TOKEN" ]; then
  export GH_TOKEN=$(cat ~/.openclaw/openclaw.json | jq -r '.skills.entries["gh-issues"].apiKey // empty')
fi

if [ -z "$GH_TOKEN" ] || [ "$GH_TOKEN" = "empty" ]; then
  echo "ERROR: GH_TOKEN not found" >> "$LOG_FILE"
  exit 1
fi

# Step 2: Check existing issues/comments on all repos
echo "[+] Checking existing issues and pull requests for feedback..." >> "$LOG_FILE"
curl -s -H "Authorization: Bearer $GH_TOKEN" -H "Accept: application/vnd.github+json" \
  "https://api.github.com/user/repos?per_page=100&sort=updated" > "$REPO_ROOT/current-repos.json"

# Extract issue counts and check for new comments
echo "$(jq '.[] | select(.open_issues_count > 0) | "\(.full_name): \(.open_issues_count) open issues"' < "$REPO_ROOT/current-repos.json")" >> "$LOG_FILE"

# Step 3: Research trending GitHub projects and popular pain points
echo "[+] Researching market demands and pain points..." >> "$LOG_FILE"

# Get trending repositories by programming language
curl -s -H "Authorization: Bearer $GH_TOKEN" -H "Accept: application/vnd.github+json" \
  "https://api.github.com/search/repositories?q=stars:>1000&sort=stars&order=desc&per_page=20" > "$REPO_ROOT/trending.json"

# Step 4: Identify pain points that can be solved with a skill/tool
echo "[+] Market research complete, ready for skill identification" >> "$LOG_FILE"
echo "=== Completed at $(date) ===" >> "$LOG_FILE"
