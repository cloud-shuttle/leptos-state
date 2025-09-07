#!/bin/bash
# Setup script for pre-commit hooks
# This script installs and configures pre-commit hooks for the leptos-state project

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Setting up pre-commit hooks for leptos-state...${NC}"

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]] || [[ ! -f ".pre-commit-config.yaml" ]]; then
    echo -e "${RED}❌ Please run this script from the project root directory${NC}"
    exit 1
fi

# Check if pre-commit is installed
if ! command -v pre-commit &> /dev/null; then
    echo -e "${YELLOW}⚠️  pre-commit is not installed${NC}"
    echo -e "${YELLOW}💡 Installing pre-commit...${NC}"
    
    # Try different installation methods
    if command -v pip &> /dev/null; then
        pip install pre-commit
    elif command -v pip3 &> /dev/null; then
        pip3 install pre-commit
    elif command -v brew &> /dev/null; then
        brew install pre-commit
    elif command -v conda &> /dev/null; then
        conda install -c conda-forge pre-commit
    else
        echo -e "${RED}❌ Could not install pre-commit automatically${NC}"
        echo -e "${YELLOW}💡 Please install pre-commit manually: https://pre-commit.com/#installation${NC}"
        exit 1
    fi
fi

# Install pre-commit hooks
echo -e "${YELLOW}🔧 Installing pre-commit hooks...${NC}"
pre-commit install

# Install pre-commit hooks for commit-msg
echo -e "${YELLOW}🔧 Installing commit-msg hook...${NC}"
pre-commit install --hook-type commit-msg

# Run pre-commit on all files to set up the environment
echo -e "${YELLOW}🔧 Running pre-commit on all files (this may take a while)...${NC}"
pre-commit run --all-files || true

# Create secrets baseline if it doesn't exist
if [[ ! -f ".secrets.baseline" ]]; then
    echo -e "${YELLOW}🔧 Creating secrets baseline...${NC}"
    detect-secrets scan --baseline .secrets.baseline || true
fi

echo -e "${GREEN}✅ Pre-commit setup complete!${NC}"
echo -e "${BLUE}📋 What was installed:${NC}"
echo -e "  • Pre-commit hooks for code formatting and linting"
echo -e "  • Custom git hooks for version consistency and documentation"
echo -e "  • Commit message validation"
echo -e "  • Security scanning with detect-secrets"
echo ""
echo -e "${BLUE}🔧 Available commands:${NC}"
echo -e "  • ${YELLOW}pre-commit run --all-files${NC} - Run all hooks on all files"
echo -e "  • ${YELLOW}pre-commit run${NC} - Run hooks on staged files"
echo -e "  • ${YELLOW}pre-commit clean${NC} - Clean pre-commit cache"
echo -e "  • ${YELLOW}pre-commit uninstall${NC} - Remove pre-commit hooks"
echo ""
echo -e "${GREEN}🎉 You're all set! Pre-commit hooks will now run automatically on commits.${NC}"
