package io.pocketmic.core

object AudioEngineBridge {
    init {
        try {
            System.loadLibrary("pocketmic_engine")
        } catch (e: UnsatisfiedLinkError) {
            // Logged during unit tests without NDK
        }
    }

    external fun nativeInit(sampleRate: Int, channels: Int): Int
    external fun nativeStart(hostIp: String, port: Int): Int
    external fun nativeStop(): Int
    external fun nativeSetMicrophoneDirection(direction: Int): Int
    external fun nativeSetSidetone(enabled: Boolean, volume: Float): Int
    external fun nativeSetMute(muted: Boolean): Int
    external fun nativeSendFxCommand(effectId: Int, mode: Int, param1: Int): Int
    external fun nativeGetPeakLevel(): Float
}
