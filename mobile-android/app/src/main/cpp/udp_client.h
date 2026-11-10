#pragma once

#include <cstdint>
#include <string>
#include <netinet/in.h>

namespace pocketmic {

#pragma pack(push, 1)
struct AudioPacketHeader {
    uint16_t magic;          // 0x504D ("PM")
    uint8_t version;         // 0x01
    uint8_t packet_type;     // 0x01: Audio
    uint32_t sequence_number;
    uint32_t timestamp_us;
    uint8_t flags;           // 0x01: Mute, 0x02: FEC, 0x04: Clipping
    uint8_t channels;        // 1 = Mono
    uint16_t payload_len;
};
#pragma pack(pop)

class UdpClient {
public:
    UdpClient();
    ~UdpClient();

    bool connect(const std::string& host, int port);
    void disconnect();
    bool sendAudioFrame(const uint8_t* payload, uint16_t length, uint8_t flags);
    bool sendFxCommand(uint8_t effectId, uint8_t mode, uint16_t param1);

private:
    int socketFd_;
    struct sockaddr_in serverAddr_;
    uint32_t sequenceNumber_;
    bool isConnected_;
};

} // namespace pocketmic
