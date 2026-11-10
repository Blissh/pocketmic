#include "udp_client.h"
#include <sys/socket.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <cstring>
#include <chrono>

namespace pocketmic {

UdpClient::UdpClient()
    : socketFd_(-1), sequenceNumber_(0), isConnected_(false) {}

UdpClient::~UdpClient() {
    disconnect();
}

bool UdpClient::connect(const std::string& host, int port) {
    disconnect();

    socketFd_ = socket(AF_INET, SOCK_DGRAM, 0);
    if (socketFd_ < 0) {
        return false;
    }

    std::memset(&serverAddr_, 0, sizeof(serverAddr_));
    serverAddr_.sin_family = AF_INET;
    serverAddr_.sin_port = htons(port);
    if (inet_pton(AF_INET, host.c_str(), &serverAddr_.sin_addr) <= 0) {
        close(socketFd_);
        socketFd_ = -1;
        return false;
    }

    sequenceNumber_ = 0;
    isConnected_ = true;
    return true;
}

void UdpClient::disconnect() {
    if (socketFd_ >= 0) {
        close(socketFd_);
        socketFd_ = -1;
    }
    isConnected_ = false;
}

bool UdpClient::sendAudioFrame(const uint8_t* payload, uint16_t length, uint8_t flags) {
    if (!isConnected_ || socketFd_ < 0) return false;

    auto now = std::chrono::steady_clock::now().time_since_epoch();
    uint32_t ts_us = static_cast<uint32_t>(std::chrono::duration_cast<std::chrono::microseconds>(now).count());

    AudioPacketHeader header{};
    header.magic = htons(0x504D);
    header.version = 1;
    header.packet_type = 1; // Audio
    header.sequence_number = htonl(sequenceNumber_++);
    header.timestamp_us = htonl(ts_us);
    header.flags = flags;
    header.channels = 1; // Mono
    header.payload_len = htons(length);

    uint8_t buffer[16 + length];
    std::memcpy(buffer, &header, sizeof(header));
    if (payload && length > 0) {
        std::memcpy(buffer + sizeof(header), payload, length);
    }

    ssize_t sent = sendto(socketFd_, buffer, sizeof(header) + length, 0,
                          reinterpret_cast<struct sockaddr*>(&serverAddr_), sizeof(serverAddr_));
    return sent > 0;
}

bool UdpClient::sendFxCommand(uint8_t effectId, uint8_t mode, uint16_t param1) {
    if (!isConnected_ || socketFd_ < 0) return false;

    #pragma pack(push, 1)
    struct FxPacket {
        uint16_t magic;
        uint8_t version;
        uint8_t packet_type;
        uint8_t effect_id;
        uint8_t mode;
        uint16_t param1;
    } fx{};
    #pragma pack(pop)

    fx.magic = htons(0x504D);
    fx.version = 1;
    fx.packet_type = 0x10; // FX_CMD
    fx.effect_id = effectId;
    fx.mode = mode;
    fx.param1 = htons(param1);

    ssize_t sent = sendto(socketFd_, &fx, sizeof(fx), 0,
                          reinterpret_cast<struct sockaddr*>(&serverAddr_), sizeof(serverAddr_));
    return sent > 0;
}

} // namespace pocketmic
