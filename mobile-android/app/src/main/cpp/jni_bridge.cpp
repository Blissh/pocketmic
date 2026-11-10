#include <jni.h>
#include <string>
#include "audio_engine.h"

static std::unique_ptr<pocketmic::AudioEngine> gAudioEngine;

extern "C" {

JNIEXPORT jint JNICALL
Java_io_pocketmic_core_AudioEngineBridge_nativeInit(
    JNIEnv *env, jobject thiz, jint sampleRate, jint channels) {
    if (!gAudioEngine) {
        gAudioEngine = std::make_unique<pocketmic::AudioEngine>();
    }
    return gAudioEngine->init(sampleRate, channels) ? 0 : -1;
}

JNIEXPORT jint JNICALL
Java_io_pocketmic_core_AudioEngineBridge_nativeStart(
    JNIEnv *env, jobject thiz, jstring hostIp, jint port) {
    if (!gAudioEngine) return -1;
    const char *nativeHost = env->GetStringUTFChars(hostIp, nullptr);
    std::string hostStr(nativeHost);
    env->ReleaseStringUTFChars(hostIp, nativeHost);

    return gAudioEngine->start(hostStr, port) ? 0 : -2;
}

JNIEXPORT jint JNICALL
Java_io_pocketmic_core_AudioEngineBridge_nativeStop(
    JNIEnv *env, jobject thiz) {
    if (gAudioEngine) {
        gAudioEngine->stop();
    }
    return 0;
}

JNIEXPORT jint JNICALL
Java_io_pocketmic_core_AudioEngineBridge_nativeSetMicrophoneDirection(
    JNIEnv *env, jobject thiz, jint direction) {
    if (gAudioEngine) {
        gAudioEngine->setMicrophoneDirection(direction);
    }
    return 0;
}

JNIEXPORT jint JNICALL
Java_io_pocketmic_core_AudioEngineBridge_nativeSetSidetone(
    JNIEnv *env, jobject thiz, jboolean enabled, jfloat volume) {
    if (gAudioEngine) {
        gAudioEngine->setSidetone(enabled, volume);
    }
    return 0;
}

JNIEXPORT jint JNICALL
Java_io_pocketmic_core_AudioEngineBridge_nativeSetMute(
    JNIEnv *env, jobject thiz, jboolean muted) {
    if (gAudioEngine) {
        gAudioEngine->setMute(muted);
    }
    return 0;
}

JNIEXPORT jint JNICALL
Java_io_pocketmic_core_AudioEngineBridge_nativeSendFxCommand(
    JNIEnv *env, jobject thiz, jint effectId, jint mode, jint param1) {
    if (gAudioEngine) {
        gAudioEngine->sendFxCommand(
            static_cast<uint8_t>(effectId),
            static_cast<uint8_t>(mode),
            static_cast<uint16_t>(param1)
        );
    }
    return 0;
}

JNIEXPORT jfloat JNICALL
Java_io_pocketmic_core_AudioEngineBridge_nativeGetPeakLevel(
    JNIEnv *env, jobject thiz) {
    if (gAudioEngine) {
        return gAudioEngine->getPeakLevel();
    }
    return 0.0f;
}

} // extern "C"
