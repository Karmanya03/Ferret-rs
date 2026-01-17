#!/usr/bin/env bash
# Security & Quality Verification Script for Ferret CLI
# Run this before publishing to GitHub

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

PASSED=0
FAILED=0

echo "═══════════════════════════════════════════"
echo "  Ferret CLI - Pre-Publication Verification"
echo "═══════════════════════════════════════════"
echo ""

# Test function
test_check() {
    local name="$1"
    local command="$2"
    
    echo -n "Checking $name... "
    if eval "$command" &> /dev/null; then
        echo -e "${GREEN}✓ PASS${NC}"
        ((PASSED++))
        return 0
    else
        echo -e "${RED}✗ FAIL${NC}"
        ((FAILED++))
        return 1
    fi
}

# Security Checks
echo -e "${BLUE}━━━ Security Checks ━━━${NC}"

# Check for sensitive files
echo -n "No credential files... "
if ! find . -type f \( -name "*.pem" -o -name "*.key" -o -name ".env" \) | grep -v ".env.example" | grep -q .; then
    echo -e "${GREEN}✓ PASS${NC}"
    ((PASSED++))
else
    echo -e "${RED}✗ FAIL - Found sensitive files${NC}"
    find . -type f \( -name "*.pem" -o -name "*.key" -o -name ".env" \) | grep -v ".env.example"
    ((FAILED++))
fi

# Check for secrets in code
echo -n "No hardcoded secrets... "
if ! grep -ri --include="*.rs" --include="*.toml" "password\|secret\|api[_-]key\|token" . | grep -v "Binary\|target\|SECURITY.md" | grep -q .; then
    echo -e "${GREEN}✓ PASS${NC}"
    ((PASSED++))
else
    echo -e "${RED}✗ FAIL - Possible secrets found${NC}"
    ((FAILED++))
fi

# Check .gitignore exists
test_check ".gitignore exists" "[ -f .gitignore ]"
test_check "SECURITY.md exists" "[ -f SECURITY.md ]"

echo ""

# Code Quality Checks
echo -e "${BLUE}━━━ Code Quality Checks ━━━${NC}"

test_check "Cargo.toml valid" "cargo metadata --format-version 1 > /dev/null"
test_check "Code formatted" "cargo fmt --check"
test_check "Clippy clean" "cargo clippy -- -D warnings"
test_check "Tests pass" "cargo test"
test_check "Release build" "cargo build --release"

echo ""

# Documentation Checks
echo -e "${BLUE}━━━ Documentation Checks ━━━${NC}"

test_check "README.md exists" "[ -f README.md ]"
test_check "LICENSE exists" "[ -f LICENSE ]"
test_check "CHANGELOG.md exists" "[ -f CHANGELOG.md ]"
test_check "CONTRIBUTING.md exists" "[ -f CONTRIBUTING.md ]"
test_check "EXAMPLES.md exists" "[ -f EXAMPLES.md ]"

echo ""

# CI/CD Checks
echo -e "${BLUE}━━━ CI/CD Configuration ━━━${NC}"

test_check "CI workflow exists" "[ -f .github/workflows/ci.yml ]"
test_check "Release workflow exists" "[ -f .github/workflows/release.yml ]"

echo ""

# Binary Checks
echo -e "${BLUE}━━━ Binary Verification ━━━${NC}"

if [ -f "target/release/fr" ]; then
    echo -n "Binary size... "
    SIZE=$(stat -f%z "target/release/fr" 2>/dev/null || stat -c%s "target/release/fr" 2>/dev/null)
    SIZE_MB=$((SIZE / 1024 / 1024))
    echo -e "${GREEN}${SIZE_MB} MB ✓${NC}"
    ((PASSED++))
    
    echo -n "Binary runs... "
    if ./target/release/fr --version > /dev/null; then
        echo -e "${GREEN}✓ PASS${NC}"
        ((PASSED++))
    else
        echo -e "${RED}✗ FAIL${NC}"
        ((FAILED++))
    fi
else
    echo -e "${YELLOW}Binary not found (run cargo build --release)${NC}"
fi

echo ""

# Placeholder Checks
echo -e "${BLUE}━━━ Repository URL Checks ━━━${NC}"

echo -n "Checking for placeholders... "
if grep -r "YOUR_USERNAME" . --include="*.md" --include="*.toml" --include="*.sh" | grep -q "YOUR_USERNAME"; then
    echo -e "${YELLOW}⚠ WARN - Placeholders found${NC}"
    echo ""
    echo "  Files with YOUR_USERNAME placeholders:"
    grep -r "YOUR_USERNAME" . --include="*.md" --include="*.toml" --include="*.sh" -l | sed 's/^/    - /'
    echo ""
    echo "  Update these with your GitHub username before publishing!"
else
    echo -e "${GREEN}✓ PASS${NC}"
    ((PASSED++))
fi

echo ""

# Summary
echo "═══════════════════════════════════════════"
echo -e "  ${GREEN}Passed: $PASSED${NC}  ${RED}Failed: $FAILED${NC}"
echo "═══════════════════════════════════════════"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✓ All checks passed! Ready to publish.${NC}"
    echo ""
    echo "Next steps:"
    echo "  1. Create GitHub repository"
    echo "  2. Update YOUR_USERNAME placeholders"
    echo "  3. git init && git add ."
    echo "  4. git commit -m 'feat: initial release'"
    echo "  5. git remote add origin <your-repo-url>"
    echo "  6. git push -u origin main"
    echo ""
    exit 0
else
    echo -e "${RED}✗ Some checks failed. Please fix before publishing.${NC}"
    echo ""
    exit 1
fi
