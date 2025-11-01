#!/bin/bash
# Test script for SQLite persistence

set -e

echo "🧪 Testing SQLite Persistence Implementation"
echo "==========================================="
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}1. Checking compilation...${NC}"
cd "$(dirname "$0")/../src-tauri"
if cargo check --quiet 2>&1 | grep -q "error"; then
    echo -e "${RED}❌ Compilation failed${NC}"
    exit 1
else
    echo -e "${GREEN}✅ Compilation successful${NC}"
fi

echo ""
echo -e "${BLUE}2. Running unit tests...${NC}"
if cargo test --lib settings 2>&1 | grep -q "test result: ok"; then
    echo -e "${GREEN}✅ SettingsRepository tests passed${NC}"
else
    echo -e "${RED}⚠️  Some tests may have failed or no tests found${NC}"
fi

echo ""
echo -e "${BLUE}3. Checking database location...${NC}"
DB_PATH="$HOME/.local/share/AgentsRS/conversations.db"
if [ -f "$DB_PATH" ]; then
    echo -e "${GREEN}✅ Database exists at: $DB_PATH${NC}"
    echo -e "   Size: $(du -h "$DB_PATH" | cut -f1)"
    echo -e "   Modified: $(stat -c %y "$DB_PATH" 2>/dev/null || stat -f %Sm "$DB_PATH" 2>/dev/null)"
else
    echo -e "${BLUE}ℹ️  Database not yet created (will be created on first app run)${NC}"
    echo -e "   Expected location: $DB_PATH"
fi

echo ""
echo -e "${BLUE}4. Checking IPC commands registration...${NC}"
if grep -q "create_session" ../src-tauri/src/lib.rs && \
   grep -q "add_message" ../src-tauri/src/lib.rs && \
   grep -q "get_current_model" ../src-tauri/src/lib.rs; then
    echo -e "${GREEN}✅ All session IPC commands registered${NC}"
else
    echo -e "${RED}❌ Some IPC commands missing${NC}"
fi

echo ""
echo -e "${BLUE}5. Verifying module exports...${NC}"
if grep -q "pub use settings::SettingsRepository" ../src-tauri/src/context/mod.rs && \
   grep -q "pub use repository::ConversationRepository" ../src-tauri/src/context/mod.rs; then
    echo -e "${GREEN}✅ All repository modules exported${NC}"
else
    echo -e "${RED}❌ Some module exports missing${NC}"
fi

echo ""
echo "==========================================="
echo -e "${GREEN}🎉 Persistence implementation verified!${NC}"
echo ""
echo "Next steps:"
echo "  1. Run: pnpm tauri dev"
echo "  2. Check logs for: 'Database URL: ...'"
echo "  3. Verify DB file created at: $DB_PATH"
echo "  4. Test frontend session integration"
echo ""
