#include "audio_engine.h"
#include <android/log.h>
#include <cmath>

#define TAG "PocketMicEngine"
#define LOGI(...) __android_log_print(ANDROID_LOG_INFO, TAG, __VA_ARGS__)
#define LOGE(...) __android_log_print(ANDROID_LOG_ERROR, TAG, __VA_ARGS__)

namespace pocketmic {

AudioEngine::AudioEngine()
    : sampleRate_(48000), channels_(1), isMuted_(false),
      sidetoneEnabled_(false), sidetoneVolume_(0.8f), peakLevel_(0.0f), micDirection_(0) {}

AudioEngine::~AudioEngine() {
    stop();
}

bool AudioEngine::init(int32_t sampleRate, int32_t channels) {
    sampleRate_ = sampleRate;
    channels_ = channels;
    return true;
}

bool AudioEngine::start(const std::string& host, int port) {
    stop();

    if (!udpClient_.connect(host, port)) {
        LOGE("Failed to connect UDP client to %s:%d", host.c_str(), port);
        return false;
    }

    oboe::AudioStreamBuilder builder;
    builder.setDirection(oboe::Direction::Input)
           ->setPerformanceMode(oboe::PerformanceMode::LowLatency)
           ->setSharingMode(oboe::SharingMode::Exclusive)
           ->setFormat(oboe::AudioFormat::Float)
           ->setChannelCount(channels_)
           ->setSampleRate(sampleRate_)
           ->setDataCallback(this);

    oboe::Result result = builder.openStream(recordingStream_);
    if (result != oboe::Result::OK) {
        LOGE("Failed to open Oboe recording stream: %s", oboe::convertToText(result));
        return false;
    }

    result = recordingStream_->requestStart();
    if (result != oboe::Result::OK) {
        LOGE("Failed to start Oboe recording stream: %s", oboe::convertToText(result));
        recordingStream_->close();
        return false;
    }

    LOGI("AudioEngine started capturing @ %d Hz (Low Latency AAudio)", sampleRate_);
    return true;
}

void AudioEngine::stop() {
    if (recordingStream_) {
        recordingStream_->stop();
        recordingStream_->close();
        recordingStream_.reset();
    }
    if (sidetoneStream_) {
        sidetoneStream_->stop();
        sidetoneStream_->close();
        sidetoneStream_.reset();
    }
    udpClient_.disconnect();
    LOGI("AudioEngine stopped.");
}

void AudioEngine::setMute(bool muted) {
    isMuted_.store(muted);
}

void AudioEngine::setMicrophoneDirection(int direction) {
    micDirection_.store(direction);
    // On Android 9+, setPreferredMicrophoneDirection is called via JNI/Java
}

void AudioEngine::setSidetone(bool enabled, float volume) {
    sidetoneEnabled_.store(enabled);
    sidetoneVolume_.store(volume);
}

void AudioEngine::sendFxCommand(uint8_t effectId, uint8_t mode, uint16_t param1) {
    udpClient_.sendFxCommand(effectId, mode, param1);
}

float AudioEngine::getPeakLevel() {
    return peakLevel_.load();
}

oboe::DataCallbackResult AudioEngine::onAudioReady(
    oboe::AudioStream *audioStream,
    void *audioData,
    int32_t numFrames) {

    const float *inputSamples = static_cast<const float*>(audioData);
    float maxAmp = 0.0f;

    // Fast SIMD-friendly peak calculation
    for (int32_t i = 0; i < numFrames * channels_; ++i) {
        float absVal = std::abs(inputSamples[i]);
        if (absVal > maxAmp) {
            maxAmp = absVal;
        }
    }
    peakLevel_.store(maxAmp);

    uint8_t flags = 0;
    if (isMuted_.load()) {
        flags |= 0x01; // MUTE
    }
    if (maxAmp > 0.95f) {
        flags |= 0x04; // CLIPPING
    }

    // Direct UDP frame send (Float32 PCM or Opus)
    udpClient_.sendAudioFrame(
        reinterpret_cast<const uint8_t*>(inputSamples),
        static_cast<uint16_t>(numFrames * channels_ * sizeof(float)),
        flags
    );

    return oboe::DataCallbackResult::Continue;
}

} // namespace pocketmic
