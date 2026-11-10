package io.pocketmic.net

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import org.json.JSONObject
import java.net.DatagramPacket
import java.net.DatagramSocket
import java.net.InetAddress

data class DiscoveredServer(
    val serverName: String,
    val ip: String,
    val port: Int,
    val sampleRate: Int
)

class DiscoveryClient {
    companion object {
        const val DISCOVERY_PORT = 37718
    }

    suspend fun discoverServer(timeoutMs: Int = 3000): DiscoveredServer? = withContext(Dispatchers.IO) {
        var socket: DatagramSocket? = null
        try {
            socket = DatagramSocket().apply {
                broadcast = true
                soTimeout = timeoutMs
            }

            val reqJson = JSONObject().apply {
                put("cmd", "DISCOVER_REQ")
                put("client_id", "android-client")
                put("device_name", android.os.Build.MODEL)
                put("protocol_version", 1)
            }

            val reqBytes = reqJson.toString().toByteArray(Charsets.UTF_8)
            val broadcastAddr = InetAddress.getByName("255.255.255.255")
            val packet = DatagramPacket(reqBytes, reqBytes.size, broadcastAddr, DISCOVERY_PORT)
            socket.send(packet)

            val recvBuf = ByteArray(2048)
            val recvPacket = DatagramPacket(recvBuf, recvBuf.size)
            socket.receive(recvPacket)

            val respJson = JSONObject(String(recvPacket.data, 0, recvPacket.length, Charsets.UTF_8))
            if (respJson.optString("cmd") == "DISCOVER_ACK") {
                return@withContext DiscoveredServer(
                    serverName = respJson.optString("server_name", "PocketMic Host"),
                    ip = recvPacket.address.hostAddress ?: "127.0.0.1",
                    port = respJson.optInt("audio_port", 37719),
                    sampleRate = respJson.optInt("sample_rate", 48000)
                )
            }
            null
        } catch (e: Exception) {
            null
        } finally {
            socket?.close()
        }
    }
}
