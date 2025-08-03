#!/bin/bash

# 1inch Fusion+ Event Watcher
# Runs the event listener every 30 seconds to check for new SrcEscrowCreated events

echo "🚀 Starting 1inch Fusion+ Event Watcher"
echo "📅 Checking every 30 seconds for new events..."
echo "🛑 Press Ctrl+C to stop"
echo ""

while true; do
    echo "⏰ $(date '+%Y-%m-%d %H:%M:%S') - Checking for events..."
    
    # Run the event listener
    node listen-src-escrow.js
    
    echo "💤 Waiting 30 seconds..."
    echo "----------------------------------------"
    
    # Wait 30 seconds before next check
    sleep 30
done