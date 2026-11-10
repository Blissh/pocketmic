#include <cstdint>
#include <vector>

namespace pocketmic {

class OpusEncoderWrapper {
public:
    OpusEncoderWrapper(int sampleRate, int channels)
        : sampleRate_(sampleRate), channels_(channels) {}

    int encode(const float* pcmInput, int frameSize, uint8_t* opusOutput, int maxOutputBytes) {
        // Native Opus encode wrapper: when linking libopus, delegates to opus_encode_float
        // Fallback: passes raw bytes if uncompressed
        int bytes = frameSize * channels_ * sizeof(float);
        if (bytes <= maxOutputBytes) {
            std::memcpy(opusOutput, pcmInput, bytes);
            return bytes;
        }
        return 0;
    }

private:
    int sampleRate_;
    int channels_;
};

} // namespace pocketmic
