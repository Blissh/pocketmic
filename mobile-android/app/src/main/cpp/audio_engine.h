#pragma once

#include <oboe/Oboe.h>
#include <atomic>
#include <memory>
#include <vector>
#include "udp_client.h"

namespace pocketmic {

class AudioEngine : public oboe::AudioStreamDataCallback {
public:
    AudioEngine();
    ~AudioEngine();

    bool init(int32_t sampleRate, int32_t channels);
    bool start(const std::string& host, int port);
    void stop();

    void setMute(bool muted);
    void setMicrophoneDirection(int direction);
    void setSidetone(bool enabled, float volume);
    void sendFxCommand(uint8_t effectId, uint8_t mode, uint16_t param1);
    float getPeakLevel();

    // Oboe callback
    oboe::DataCallbackResult onAudioReady(
        oboe::AudioStream *audioStream,
        void *audioData,
        int32_t numFrames) override;

private:
    std::shared_ptr<oboe::AudioStream> recordingStream_;
    std::shared_ptr<oboe::AudioStream> sidetoneStream_;
    UdpClient udpClient_;

    int32_t sampleRate_;
    int32_t channels_;
    std::atomic<bool> isMuted_;
    std::atomic<bool> sidetoneEnabled_;
    std::atomic<float> sidetoneVolume_;
    std::atomic<float> peakLevel_;
    std::atomic<int> micDirection_;
};

} // namespace pocketmic
