#!/bin/bash
set -e

# Build and run the example in Android emulator
# This script requires Android SDK, NDK, and cargo-ndk to be installed

# Check if ANDROID_SDK_ROOT is set
if [ -z "$ANDROID_SDK_ROOT" ]; then
  echo "Error: ANDROID_SDK_ROOT environment variable not set."
  echo "Please set it to your Android SDK path."
  exit 1
fi

# Check if ANDROID_NDK_HOME is set
if [ -z "$ANDROID_NDK_HOME" ]; then
  echo "Looking for NDK installation..."
  # Try to find latest NDK
  LATEST_NDK=$(find "$ANDROID_SDK_ROOT/ndk" -maxdepth 1 -mindepth 1 -type d | sort -r | head -1)
  if [ -n "$LATEST_NDK" ]; then
    export ANDROID_NDK_HOME="$LATEST_NDK"
    echo "Set ANDROID_NDK_HOME to $ANDROID_NDK_HOME"
  else
    echo "Error: ANDROID_NDK_HOME environment variable not set and no NDK found in SDK."
    echo "Please install NDK using Android Studio or sdkmanager."
    exit 1
  fi
fi

# Install cargo-ndk if not already installed
if ! command -v cargo-ndk &> /dev/null; then
  echo "Installing cargo-ndk..."
  cargo install cargo-ndk
fi

# Install Android targets
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android

# Create Android project structure
mkdir -p android/app/src/main/java/com/example/tinyfiledialogs
mkdir -p android/app/src/main/jniLibs/{armeabi-v7a,arm64-v8a,x86,x86_64}
mkdir -p android/app/src/main/res/{layout,values}

# Build for Android architectures
echo "Building Rust library for Android..."
cargo ndk -t armeabi-v7a -t arm64-v8a -t x86 -t x86_64 -o android/app/src/main/jniLibs build --release --example test

# Create basic Android app files
cat > android/app/src/main/AndroidManifest.xml << EOF
<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    package="com.example.tinyfiledialogs">

    <application
        android:allowBackup="true"
        android:label="TinyFileDialogs"
        android:supportsRtl="true">
        <activity
            android:name=".MainActivity"
            android:exported="true">
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>
    </application>

</manifest>
EOF

# Create activity layout
cat > android/app/src/main/res/layout/activity_main.xml << EOF
<?xml version="1.0" encoding="utf-8"?>
<LinearLayout xmlns:android="http://schemas.android.com/apk/res/android"
    android:layout_width="match_parent"
    android:layout_height="match_parent"
    android:orientation="vertical"
    android:padding="16dp">

    <Button
        android:id="@+id/button_test"
        android:layout_width="match_parent"
        android:layout_height="wrap_content"
        android:text="Run Test" />

</LinearLayout>
EOF

# Create MainActivity.java
cat > android/app/src/main/java/com/example/tinyfiledialogs/MainActivity.java << EOF
package com.example.tinyfiledialogs;

import android.app.Activity;
import android.os.Bundle;
import android.view.View;
import android.widget.Button;

public class MainActivity extends Activity {
    static {
        System.loadLibrary("test");
    }
    
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        
        // Initialize DialogHelper
        DialogHelper.createNotificationChannel(this);
        
        Button testButton = findViewById(R.id.button_test);
        testButton.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                runTest();
            }
        });
    }
    
    // Native method defined in Rust
    public native void runTest();
}
EOF

# Copy your DialogHelper class
cat > android/app/src/main/java/com/example/tinyfiledialogs/DialogHelper.java << EOF
package com.example.tinyfiledialogs;

import android.app.Activity;
import android.app.AlertDialog;
import android.content.DialogInterface;
// ... Copy the rest of your DialogHelper.java here
// For brevity, this is truncated in this script

public class DialogHelper {
    // Paste your DialogHelper implementation here
    public static void createNotificationChannel(Context context) {
        // Implementation
    }
    
    // Other methods
}
EOF

echo "Android project created at $(pwd)/android"
echo ""
echo "To build and run the app:"
echo "1. Open the android folder in Android Studio"
echo "2. Let Gradle sync the project"
echo "3. Run the app on an emulator or device"