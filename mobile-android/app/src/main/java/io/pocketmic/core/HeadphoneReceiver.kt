package io.pocketmic.core

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.media.AudioManager

class HeadphoneReceiver(
    private val onHeadphonesChanged: (connected: Boolean) -> Unit
) : BroadcastReceiver() {

    override fun onReceive(context: Context?, intent: Intent?) {
        if (intent?.action == Intent.ACTION_HEADSET_PLUG) {
            val state = intent.getIntExtra("state", -1)
            val isConnected = state == 1
            if (!isConnected) {
                // Safety: immediately disable sidetone if unplugged to prevent speaker feedback
                AudioEngineBridge.nativeSetSidetone(false, 0f)
            }
            onHeadphonesChanged(isConnected)
        }
    }
}
