package io.pocketmic.core

import android.app.*
import android.content.Context
import android.content.Intent
import android.content.pm.ServiceInfo
import android.os.Build
import android.os.IBinder
import androidx.core.app.NotificationCompat

class PocketMicService : Service() {

    companion object {
        const val CHANNEL_ID = "pocketmic_streaming_channel"
        const val NOTIFICATION_ID = 1001
        const val ACTION_START = "ACTION_START"
        const val ACTION_STOP = "ACTION_STOP"
        const val EXTRA_HOST = "EXTRA_HOST"
        const val EXTRA_PORT = "EXTRA_PORT"
    }

    override fun onCreate() {
        super.onCreate()
        createNotificationChannel()
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        when (intent?.action) {
            ACTION_START -> {
                val host = intent.getStringExtra(EXTRA_HOST) ?: "192.168.1.100"
                val port = intent.getIntExtra(EXTRA_PORT, 37719)
                startForegroundStreaming(host, port)
            }
            ACTION_STOP -> {
                stopForegroundStreaming()
            }
        }
        return START_STICKY
    }

    private fun startForegroundStreaming(host: String, port: Int) {
        val notification = NotificationCompat.Builder(this, CHANNEL_ID)
            .setContentTitle("PocketMic En Vivo 🎙️")
            .setContentText("Transmitiendo voz a $host:$port")
            .setSmallIcon(android.R.drawable.ic_btn_speak_now)
            .setOngoing(true)
            .setPriority(NotificationCompat.PRIORITY_HIGH)
            .build()

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
            startForeground(
                NOTIFICATION_ID,
                notification,
                ServiceInfo.FOREGROUND_SERVICE_TYPE_MICROPHONE
            )
        } else {
            startForeground(NOTIFICATION_ID, notification)
        }

        AudioEngineBridge.nativeInit(48000, 1)
        AudioEngineBridge.nativeStart(host, port)
    }

    private fun stopForegroundStreaming() {
        AudioEngineBridge.nativeStop()
        stopForeground(STOP_FOREGROUND_REMOVE)
        stopSelf()
    }

    private fun createNotificationChannel() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel(
                CHANNEL_ID,
                "PocketMic Audio Stream",
                NotificationManager.IMPORTANCE_LOW
            ).apply {
                description = "Notificación persistente de transmisión de micrófono"
            }
            val manager = getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
            manager.createNotificationChannel(channel)
        }
    }

    override fun onBind(intent: Intent?): IBinder? = null
}
