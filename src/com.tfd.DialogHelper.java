package com.example.tinyfiledialogs;

import android.app.Activity;
import android.app.AlertDialog;
import android.content.DialogInterface;
import android.text.InputType;
import android.widget.EditText;
import android.content.Intent;
import android.net.Uri;
import android.os.Environment;
import android.provider.DocumentsContract;
import android.content.ContentResolver;
import androidx.core.app.NotificationCompat;
import androidx.core.app.NotificationManagerCompat;
import android.app.NotificationChannel;
import android.app.NotificationManager;
import android.os.Build;
import android.graphics.Color;
import android.app.PendingIntent;
import android.content.Context;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.atomic.AtomicReference;

public class DialogHelper {
    private static final String NOTIFICATION_CHANNEL_ID = "tinyfiledialogs_channel";
    private static final int NOTIFICATION_ID = 1;
    
    // Initialize notification channel (call once during app startup)
    public static void createNotificationChannel(Context context) {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            String name = "TinyFileDialogs Notifications";
            String description = "Notifications from TinyFileDialogs";
            int importance = NotificationManager.IMPORTANCE_DEFAULT;
            
            NotificationChannel channel = new NotificationChannel(
                NOTIFICATION_CHANNEL_ID, name, importance);
            channel.setDescription(description);
            
            NotificationManager notificationManager = 
                context.getSystemService(NotificationManager.class);
            if (notificationManager != null) {
                notificationManager.createNotificationChannel(channel);
            }
        }
    }
    
    // Shows a simple message box (OK button only)
    public static void showMessageBox(final Activity activity, final String title, 
                                     final String message, final int iconType) {
        if (activity == null) return;
        
        activity.runOnUiThread(() -> {
            AlertDialog.Builder builder = new AlertDialog.Builder(activity)
                .setTitle(title)
                .setMessage(message)
                .setPositiveButton("OK", null);
                
            switch (iconType) {
                case 1: // Warning
                    builder.setIcon(android.R.drawable.ic_dialog_alert);
                    break;
                case 3: // Info
                    builder.setIcon(android.R.drawable.ic_dialog_info);
                    break;
            }
            
            builder.show();
        });
    }
    
    // Shows OK/Cancel dialog and returns result (1 = OK, 0 = Cancel)
    public static int showOkCancelDialog(final Activity activity, final String title,
                                        final String message, final int iconType,
                                        final int defaultButton) {
        if (activity == null) return 0;
        
        final CountDownLatch latch = new CountDownLatch(1);
        final AtomicReference<Integer> result = new AtomicReference<>(0);
        
        activity.runOnUiThread(() -> {
            AlertDialog.Builder builder = new AlertDialog.Builder(activity)
                .setTitle(title)
                .setMessage(message)
                .setPositiveButton("OK", (dialog, which) -> {
                    result.set(1);
                    latch.countDown();
                })
                .setNegativeButton("Cancel", (dialog, which) -> {
                    result.set(0);
                    latch.countDown();
                })
                .setOnCancelListener(dialog -> {
                    result.set(0);
                    latch.countDown();
                });
                
            switch (iconType) {
                case 1: // Warning
                    builder.setIcon(android.R.drawable.ic_dialog_alert);
                    break;
                case 3: // Info
                    builder.setIcon(android.R.drawable.ic_dialog_info);
                    break;
            }
            
            AlertDialog dialog = builder.create();
            dialog.show();
            
            // Set default button focus
            if (defaultButton == 1) {
                dialog.getButton(DialogInterface.BUTTON_POSITIVE).requestFocus();
            } else {
                dialog.getButton(DialogInterface.BUTTON_NEGATIVE).requestFocus();
            }
        });
        
        try {
            latch.await();
            return result.get();
        } catch (InterruptedException e) {
            return 0;
        }
    }
    
    // Shows Yes/No dialog and returns result (1 = Yes, 0 = No)
    public static int showYesNoDialog(final Activity activity, final String title,
                                     final String message, final int iconType,
                                     final int defaultButton) {
        if (activity == null) return 0;
        
        final CountDownLatch latch = new CountDownLatch(1);
        final AtomicReference<Integer> result = new AtomicReference<>(0);
        
        activity.runOnUiThread(() -> {
            AlertDialog.Builder builder = new AlertDialog.Builder(activity)
                .setTitle(title)
                .setMessage(message)
                .setPositiveButton("Yes", (dialog, which) -> {
                    result.set(1);
                    latch.countDown();
                })
                .setNegativeButton("No", (dialog, which) -> {
                    result.set(0);
                    latch.countDown();
                })
                .setOnCancelListener(dialog -> {
                    result.set(0);
                    latch.countDown();
                });
                
            switch (iconType) {
                case 1: // Warning
                    builder.setIcon(android.R.drawable.ic_dialog_alert);
                    break;
                case 3: // Info
                    builder.setIcon(android.R.drawable.ic_dialog_info);
                    break;
            }
            
            AlertDialog dialog = builder.create();
            dialog.show();
            
            // Set default button focus
            if (defaultButton == 1) {
                dialog.getButton(DialogInterface.BUTTON_POSITIVE).requestFocus();
            } else {
                dialog.getButton(DialogInterface.BUTTON_NEGATIVE).requestFocus();
            }
        });
        
        try {
            latch.await();
            return result.get();
        } catch (InterruptedException e) {
            return 0;
        }
    }
    
    // Shows Yes/No/Cancel dialog and returns result (1 = Yes, 2 = No, 0 = Cancel)
    public static int showYesNoCancelDialog(final Activity activity, final String title,
                                           final String message, final int iconType,
                                           final int defaultButton) {
        if (activity == null) return 0;
        
        final CountDownLatch latch = new CountDownLatch(1);
        final AtomicReference<Integer> result = new AtomicReference<>(0);
        
        activity.runOnUiThread(() -> {
            AlertDialog.Builder builder = new AlertDialog.Builder(activity)
                .setTitle(title)
                .setMessage(message)
                .setPositiveButton("Yes", (dialog, which) -> {
                    result.set(1);
                    latch.countDown();
                })
                .setNegativeButton("No", (dialog, which) -> {
                    result.set(2);
                    latch.countDown();
                })
                .setNeutralButton("Cancel", (dialog, which) -> {
                    result.set(0);
                    latch.countDown();
                })
                .setOnCancelListener(dialog -> {
                    result.set(0);
                    latch.countDown();
                });
                
            switch (iconType) {
                case 1: // Warning
                    builder.setIcon(android.R.drawable.ic_dialog_alert);
                    break;
                case 3: // Info
                    builder.setIcon(android.R.drawable.ic_dialog_info);
                    break;
            }
            
            AlertDialog dialog = builder.create();
            dialog.show();
            
            // Set default button focus
            switch (defaultButton) {
                case 1: // Yes
                    dialog.getButton(DialogInterface.BUTTON_POSITIVE).requestFocus();
                    break;
                case 2: // No
                    dialog.getButton(DialogInterface.BUTTON_NEGATIVE).requestFocus();
                    break;
                case 0: // Cancel
                    dialog.getButton(DialogInterface.BUTTON_NEUTRAL).requestFocus();
                    break;
            }
        });
        
        try {
            latch.await();
            return result.get();
        } catch (InterruptedException e) {
            return 0;
        }
    }
    
    // Shows input dialog and returns entered text or null if canceled
    public static String showInputDialog(final Activity activity, final String title,
                                        final String message, final String defaultValue,
                                        final boolean isPassword) {
        if (activity == null) return null;
        
        final CountDownLatch latch = new CountDownLatch(1);
        final AtomicReference<String> result = new AtomicReference<>(null);
        
        activity.runOnUiThread(() -> {
            final EditText input = new EditText(activity);
            input.setText(defaultValue);
            
            if (isPassword) {
                input.setInputType(InputType.TYPE_CLASS_TEXT | 
                                  InputType.TYPE_TEXT_VARIATION_PASSWORD);
            }
            
            AlertDialog.Builder builder = new AlertDialog.Builder(activity)
                .setTitle(title)
                .setMessage(message)
                .setView(input)
                .setPositiveButton("OK", (dialog, which) -> {
                    result.set(input.getText().toString());
                    latch.countDown();
                })
                .setNegativeButton("Cancel", (dialog, which) -> {
                    result.set(null);
                    latch.countDown();
                })
                .setOnCancelListener(dialog -> {
                    result.set(null);
                    latch.countDown();
                });
                
            builder.show();
        });
        
        try {
            latch.await();
            return result.get();
        } catch (InterruptedException e) {
            return null;
        }
    }
    
    // Shows file save dialog using Storage Access Framework
    public static String showSaveFileDialog(final Activity activity, final String title,
                                           final String defaultPath, final String[] filterPatterns) {
        if (activity == null) return null;
        
        final CountDownLatch latch = new CountDownLatch(1);
        final AtomicReference<String> result = new AtomicReference<>(null);
        
        activity.runOnUiThread(() -> {
            Intent intent = new Intent(Intent.ACTION_CREATE_DOCUMENT);
            intent.addCategory(Intent.CATEGORY_OPENABLE);
            intent.setType("*/*");
            intent.putExtra(Intent.EXTRA_TITLE, getFileNameFromPath(defaultPath));
            
            if (title != null && !title.isEmpty()) {
                intent.putExtra(DocumentsContract.EXTRA_PROMPT, title);
            }
            
            // Start file picker activity
            try {
                // Use a request code to identify the result in onActivityResult
                activity.startActivityForResult(intent, 1001);
                
                // In real implementation, you'd handle the result in your Activity's onActivityResult
                // For this example, we'll simulate a response after a delay
                new android.os.Handler().postDelayed(() -> {
                    // In real implementation, this would be the URI from onActivityResult
                    // Simulating no selection for this example
                    result.set(null);
                    latch.countDown();
                }, 1000);
            } catch (Exception e) {
                result.set(null);
                latch.countDown();
            }
        });
        
        try {
            latch.await();
            return result.get();
        } catch (InterruptedException e) {
            return null;
        }
    }
    
    // Shows file open dialog using Storage Access Framework
    public static String[] showOpenFileDialog(final Activity activity, final String title,
                                            final String defaultPath, final String[] filterPatterns,
                                            final boolean allowMultiple) {
        if (activity == null) return null;
        
        final CountDownLatch latch = new CountDownLatch(1);
        final AtomicReference<String[]> result = new AtomicReference<>(null);
        
        activity.runOnUiThread(() -> {
            Intent intent = new Intent(Intent.ACTION_OPEN_DOCUMENT);
            intent.addCategory(Intent.CATEGORY_OPENABLE);
            intent.setType("*/*");
            
            if (allowMultiple) {
                intent.putExtra(Intent.EXTRA_ALLOW_MULTIPLE, true);
            }
            
            if (title != null && !title.isEmpty()) {
                intent.putExtra(DocumentsContract.EXTRA_PROMPT, title);
            }
            
            // Start file picker activity
            try {
                // Use a request code to identify the result in onActivityResult
                activity.startActivityForResult(intent, 1002);
                
                // In real implementation, you'd handle the result in your Activity's onActivityResult
                // For this example, we'll simulate a response after a delay
                new android.os.Handler().postDelayed(() -> {
                    // In real implementation, this would be the URI from onActivityResult
                    // Simulating no selection for this example
                    result.set(null);
                    latch.countDown();
                }, 1000);
            } catch (Exception e) {
                result.set(null);
                latch.countDown();
            }
        });
        
        try {
            latch.await();
            return result.get();
        } catch (InterruptedException e) {
            return null;
        }
    }
    
    // Shows folder dialog using Storage Access Framework
    public static String showFolderDialog(final Activity activity, final String title,
                                         final String defaultPath) {
        if (activity == null) return null;
        
        final CountDownLatch latch = new CountDownLatch(1);
        final AtomicReference<String> result = new AtomicReference<>(null);
        
        activity.runOnUiThread(() -> {
            Intent intent = new Intent(Intent.ACTION_OPEN_DOCUMENT_TREE);
            
            if (title != null && !title.isEmpty()) {
                intent.putExtra(DocumentsContract.EXTRA_PROMPT, title);
            }
            
            // Start directory picker activity
            try {
                // Use a request code to identify the result in onActivityResult
                activity.startActivityForResult(intent, 1003);
                
                // In real implementation, you'd handle the result in your Activity's onActivityResult
                // For this example, we'll simulate a response after a delay
                new android.os.Handler().postDelayed(() -> {
                    // In real implementation, this would be the URI from onActivityResult
                    // Simulating no selection for this example
                    result.set(null);
                    latch.countDown();
                }, 1000);
            } catch (Exception e) {
                result.set(null);
                latch.countDown();
            }
        });
        
        try {
            latch.await();
            return result.get();
        } catch (InterruptedException e) {
            return null;
        }
    }
    
    // Shows color chooser dialog
    public static int[] showColorChooser(final Activity activity, final String title,
                                        final int defaultR, final int defaultG, final int defaultB) {
        if (activity == null) return null;
        
        final CountDownLatch latch = new CountDownLatch(1);
        final AtomicReference<int[]> result = new AtomicReference<>(null);
        
        activity.runOnUiThread(() -> {
            // For Android 10+ we'd use the ColorPickerDialog from androidx
            // For demonstration, we'll use a simple dialog with RGB values
            
            final EditText rInput = new EditText(activity);
            final EditText gInput = new EditText(activity);
            final EditText bInput = new EditText(activity);
            
            rInput.setInputType(InputType.TYPE_CLASS_NUMBER);
            gInput.setInputType(InputType.TYPE_CLASS_NUMBER);
            bInput.setInputType(InputType.TYPE_CLASS_NUMBER);
            
            rInput.setText(String.valueOf(defaultR));
            gInput.setText(String.valueOf(defaultG));
            bInput.setText(String.valueOf(defaultB));
            
            android.widget.LinearLayout layout = new android.widget.LinearLayout(activity);
            layout.setOrientation(android.widget.LinearLayout.VERTICAL);
            
            layout.addView(new android.widget.TextView(activity) {{ setText("Red (0-255):"); }});
            layout.addView(rInput);
            layout.addView(new android.widget.TextView(activity) {{ setText("Green (0-255):"); }});
            layout.addView(gInput);
            layout.addView(new android.widget.TextView(activity) {{ setText("Blue (0-255):"); }});
            layout.addView(bInput);
            
            // Add color preview
            android.widget.LinearLayout previewLayout = new android.widget.LinearLayout(activity);
            previewLayout.setBackgroundColor(Color.rgb(defaultR, defaultG, defaultB));
            previewLayout.setMinimumHeight(100);
            layout.addView(previewLayout);
            
            AlertDialog.Builder builder = new AlertDialog.Builder(activity)
                .setTitle(title)
                .setView(layout)
                .setPositiveButton("OK", (dialog, which) -> {
                    try {
                        int r = Integer.parseInt(rInput.getText().toString());
                        int g = Integer.parseInt(gInput.getText().toString());
                        int b = Integer.parseInt(bInput.getText().toString());
                        
                        // Clamp values to 0-255
                        r = Math.max(0, Math.min(255, r));
                        g = Math.max(0, Math.min(255, g));
                        b = Math.max(0, Math.min(255, b));
                        
                        result.set(new int[] { r, g, b });
                    } catch (NumberFormatException e) {
                        result.set(new int[] { defaultR, defaultG, defaultB });
                    }
                    latch.countDown();
                })
                .setNegativeButton("Cancel", (dialog, which) -> {
                    result.set(null);
                    latch.countDown();
                })
                .setOnCancelListener(dialog -> {
                    result.set(null);
                    latch.countDown();
                });
                
            builder.show();
        });
        
        try {
            latch.await();
            return result.get();
        } catch (InterruptedException e) {
            return null;
        }
    }
    
    // Shows a notification
    public static boolean showNotification(final Activity activity, final String title,
                                          final String message, final String subtitle) {
        if (activity == null) return false;
        
        try {
            // Create notification channel for Android O and above
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                createNotificationChannel(activity);
            }
            
            // Build notification
            NotificationCompat.Builder builder = new NotificationCompat.Builder(activity, NOTIFICATION_CHANNEL_ID)
                .setSmallIcon(android.R.drawable.ic_dialog_info)
                .setContentTitle(title)
                .setContentText(message)
                .setPriority(NotificationCompat.PRIORITY_DEFAULT);
                
            if (subtitle != null && !subtitle.isEmpty()) {
                builder.setSubText(subtitle);
            }
            
            // Add tap action that opens the app
            Intent intent = new Intent(activity, activity.getClass());
            intent.setFlags(Intent.FLAG_ACTIVITY_NEW_TASK | Intent.FLAG_ACTIVITY_CLEAR_TASK);
            PendingIntent pendingIntent = PendingIntent.getActivity(
                activity, 0, intent, 
                Build.VERSION.SDK_INT >= Build.VERSION_CODES.M 
                    ? PendingIntent.FLAG_IMMUTABLE 
                    : 0);
            builder.setContentIntent(pendingIntent)
                   .setAutoCancel(true);
            
            // Show notification
            NotificationManagerCompat notificationManager = NotificationManagerCompat.from(activity);
            notificationManager.notify(NOTIFICATION_ID, builder.build());
            
            return true;
        } catch (Exception e) {
            return false;
        }
    }
    
    // Helper method to extract file name from path
    private static String getFileNameFromPath(String path) {
        if (path == null || path.isEmpty()) {
            return "untitled";
        }
        
        int lastSeparator = Math.max(path.lastIndexOf('/'), path.lastIndexOf('\\'));
        if (lastSeparator != -1) {
            return path.substring(lastSeparator + 1);
        } else {
            return path;
        }
    }
}