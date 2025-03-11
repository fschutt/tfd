#!/bin/bash
set -e

# Build and run the example in iOS simulator
# This script must be run on macOS with Xcode installed

# Install iOS target if needed
rustup target add x86_64-apple-ios

# Get the iOS simulator SDK path
SDK_PATH=$(xcrun --sdk iphonesimulator --show-sdk-path)

# Create linker shim
echo "#!/bin/sh" > linker_shim.sh
echo "cc -isysroot $SDK_PATH \"\$@\"" >> linker_shim.sh
chmod +x linker_shim.sh

# Set environment variable for linker
export CARGO_TARGET_X86_64_APPLE_IOS_LINKER=$(pwd)/linker_shim.sh

# Build the example for iOS simulator
cargo build --example test --target x86_64-apple-ios

# Create app bundle
APP_DIR="app/tinyfiledialogs.app"
mkdir -p "$APP_DIR"
cp target/x86_64-apple-ios/debug/examples/test "$APP_DIR/tinyfiledialogs"

# Create Info.plist
cat > "$APP_DIR/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key>
  <string>tinyfiledialogs</string>
  <key>CFBundleIdentifier</key>
  <string>com.example.tinyfiledialogs</string>
  <key>CFBundleName</key>
  <string>TinyFileDialogs</string>
  <key>CFBundleVersion</key>
  <string>1.0</string>
  <key>CFBundleShortVersionString</key>
  <string>1.0</string>
  <key>UIRequiredDeviceCapabilities</key>
  <array>
    <string>x86_64</string>
  </array>
</dict>
</plist>
EOF

# Start the iOS simulator
echo "Starting iOS Simulator..."
open -a Simulator

# Wait for simulator to boot
sleep 5

# Get first booted simulator
DEVICE_ID=$(xcrun simctl list devices | grep Booted | head -1 | sed -E 's/.*\(([A-Z0-9-]+)\).*/\1/')

if [ -z "$DEVICE_ID" ]; then
  echo "No booted simulator found. Starting iPhone 14..."
  xcrun simctl boot "iPhone 14"
  DEVICE_ID=$(xcrun simctl list devices | grep Booted | head -1 | sed -E 's/.*\(([A-Z0-9-]+)\).*/\1/')
fi

echo "Using simulator: $DEVICE_ID"

# Install and run the app
xcrun simctl install "$DEVICE_ID" app/
xcrun simctl launch --console "$DEVICE_ID" com.example.tinyfiledialogs