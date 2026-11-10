package io.pocketmic.ui

import android.Manifest
import android.content.Intent
import android.content.IntentFilter
import android.content.pm.PackageManager
import android.media.AudioManager
import android.os.Build
import android.os.Bundle
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Mic
import androidx.compose.material.icons.filled.MicOff
import androidx.compose.material.icons.filled.Refresh
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.core.content.ContextCompat
import io.pocketmic.core.AudioEngineBridge
import io.pocketmic.core.HeadphoneReceiver
import io.pocketmic.core.PocketMicService
import io.pocketmic.net.DiscoveredServer
import io.pocketmic.net.DiscoveryClient
import io.pocketmic.ui.components.CapsuleSelector
import io.pocketmic.ui.components.StreamDeckGrid
import io.pocketmic.ui.components.VuMeter
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch

class MainActivity : ComponentActivity() {

    private val headphoneReceiver = HeadphoneReceiver()
    private var isHeadphonesPlugged by mutableStateOf(false)

    private val permissionLauncher = registerForActivityResult(
        ActivityResultContracts.RequestMultiplePermissions()
    ) { permissions ->
        val recordAudioGranted = permissions[Manifest.permission.RECORD_AUDIO] ?: false
        if (!recordAudioGranted) {
            Toast.makeText(this, "Permiso de micrófono requerido para transmitir voz", Toast.LENGTH_LONG).show()
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        checkPermissions()

        headphoneReceiver.onHeadphoneStateChanged = { connected ->
            isHeadphonesPlugged = connected
            if (!connected) {
                AudioEngineBridge.nativeSetSidetone(false, 0.0f)
            }
        }

        setContent {
            PocketMicTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    PocketMicApp(
                        isHeadphonesPlugged = isHeadphonesPlugged,
                        onStartStreaming = { host, port -> startService(host, port) },
                        onStopStreaming = { stopService() }
                    )
                }
            }
        }
    }

    override fun onResume() {
        super.onResume()
        val filter = IntentFilter(AudioManager.ACTION_HEADSET_PLUG)
        registerReceiver(headphoneReceiver, filter)
    }

    override fun onPause() {
        super.onPause()
        try {
            unregisterReceiver(headphoneReceiver)
        } catch (_: Exception) {}
    }

    private fun checkPermissions() {
        val permissions = mutableListOf(Manifest.permission.RECORD_AUDIO)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            permissions.add(Manifest.permission.POST_NOTIFICATIONS)
        }
        val needed = permissions.filter {
            ContextCompat.checkSelfPermission(this, it) != PackageManager.PERMISSION_GRANTED
        }
        if (needed.isNotEmpty()) {
            permissionLauncher.launch(needed.toTypedArray())
        }
    }

    private fun startService(host: String, port: Int) {
        val intent = Intent(this, PocketMicService::class.java).apply {
            action = PocketMicService.ACTION_START
            putExtra(PocketMicService.EXTRA_HOST, host)
            putExtra(PocketMicService.EXTRA_PORT, port)
        }
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            startForegroundService(intent)
        } else {
            startService(intent)
        }
    }

    private fun stopService() {
        val intent = Intent(this, PocketMicService::class.java).apply {
            action = PocketMicService.ACTION_STOP
        }
        startService(intent)
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun PocketMicApp(
    isHeadphonesPlugged: Boolean,
    onStartStreaming: (host: String, port: Int) -> Unit,
    onStopStreaming: () -> Unit
) {
    val coroutineScope = rememberCoroutineScope()
    var isStreaming by remember { mutableStateOf(false) }
    var isMuted by remember { mutableStateOf(false) }
    var selectedCapsule by remember { mutableIntStateOf(0) }
    var activeEffectId by remember { mutableIntStateOf(0) }
    var peakLevel by remember { mutableFloatStateOf(0f) }

    var sidetoneEnabled by remember { mutableStateOf(false) }
    var sidetoneVolume by remember { mutableFloatStateOf(0.7f) }

    var targetIp by remember { mutableStateOf("192.168.1.100") }
    var targetPort by remember { mutableIntStateOf(37719) }
    var discoveredServers by remember { mutableStateOf<List<DiscoveredServer>>(emptyList()) }
    var isSearchingHost by remember { mutableStateOf(false) }
    var showConnectDialog by remember { mutableStateOf(false) }

    // Auto-discovery function
    val triggerDiscovery = {
        coroutineScope.launch {
            isSearchingHost = true
            val discovered = DiscoveryClient().discoverServer()
            if (discovered != null) {
                targetIp = discovered.ip
                targetPort = discovered.port
                discoveredServers = listOf(discovered)
            }
            isSearchingHost = false
        }
    }

    // Auto-trigger discovery on launch
    LaunchedEffect(Unit) {
        triggerDiscovery()
    }

    // Polling peak level for VU meter when streaming
    LaunchedEffect(isStreaming) {
        while (isStreaming) {
            peakLevel = AudioEngineBridge.nativeGetPeakLevel()
            delay(33) // ~30 fps update
        }
        peakLevel = 0f
    }

    Scaffold(
        topBar = {
            TopAppBar(
                title = {
                    Row(verticalAlignment = Alignment.CenterVertically) {
                        Text("PocketMic", fontWeight = FontWeight.Bold, fontSize = 20.sp)
                        Spacer(modifier = Modifier.width(8.dp))
                        Surface(
                            shape = RoundedCornerShape(12.dp),
                            color = if (isStreaming) Color(0xFF00E676) else Color.DarkGray
                        ) {
                            Text(
                                text = if (isStreaming) "EN VIVO" else "STANDBY",
                                color = if (isStreaming) Color.Black else Color.LightGray,
                                fontSize = 10.sp,
                                fontWeight = FontWeight.Bold,
                                modifier = Modifier.padding(horizontal = 8.dp, vertical = 2.dp)
                            )
                        }
                    }
                },
                actions = {
                    IconButton(onClick = { triggerDiscovery() }, enabled = !isSearchingHost) {
                        Icon(
                            imageVector = Icons.Default.Refresh,
                            contentDescription = "Buscar PC Host",
                            tint = if (isSearchingHost) Color.Gray else Color.White
                        )
                    }
                    IconButton(onClick = { showConnectDialog = true }) {
                        Icon(
                            imageVector = Icons.Default.Settings,
                            contentDescription = "Configurar IP",
                            tint = Color.White
                        )
                    }
                },
                colors = TopAppBarDefaults.topAppBarColors(
                    containerColor = Color(0xFF181818)
                )
            )
        }
    ) { paddingValues ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(paddingValues)
                .padding(horizontal = 16.dp, vertical = 8.dp)
                .verticalScroll(rememberScrollState()),
            verticalArrangement = Arrangement.spacedBy(16.dp)
        ) {
            // Host Information Banner
            Surface(
                modifier = Modifier.fillMaxWidth(),
                shape = RoundedCornerShape(10.dp),
                color = Color(0xFF222222)
            ) {
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(12.dp),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Column {
                        Text(
                            text = "Servidor PC Destino",
                            fontSize = 11.sp,
                            color = Color.Gray
                        )
                        Text(
                            text = "$targetIp:$targetPort",
                            fontSize = 15.sp,
                            fontWeight = FontWeight.SemiBold,
                            color = Color.White
                        )
                    }
                    OutlinedButton(
                        onClick = { showConnectDialog = true },
                        contentPadding = PaddingValues(horizontal = 12.dp, vertical = 4.dp)
                    ) {
                        Text("Cambiar", fontSize = 12.sp)
                    }
                }
            }

            // VU Meter
            VuMeter(level = if (isMuted) 0f else peakLevel)

            // Main Controls: Streaming Start/Stop & Mute
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(12.dp)
            ) {
                Button(
                    onClick = {
                        if (isStreaming) {
                            onStopStreaming()
                            isStreaming = false
                        } else {
                            onStartStreaming(targetIp, targetPort)
                            isStreaming = true
                        }
                    },
                    colors = ButtonDefaults.buttonColors(
                        containerColor = if (isStreaming) Color(0xFFD32F2F) else MaterialTheme.colorScheme.primary
                    ),
                    shape = RoundedCornerShape(12.dp),
                    modifier = Modifier
                        .weight(2f)
                        .height(52.dp)
                ) {
                    Text(
                        text = if (isStreaming) "⏹ DETENER STREAM" else "▶ EMPEZAR STREAM",
                        fontWeight = FontWeight.Bold,
                        fontSize = 15.sp
                    )
                }

                Button(
                    onClick = {
                        isMuted = !isMuted
                        AudioEngineBridge.nativeSetMute(isMuted)
                    },
                    colors = ButtonDefaults.buttonColors(
                        containerColor = if (isMuted) Color(0xFFE65100) else Color(0xFF333333)
                    ),
                    shape = RoundedCornerShape(12.dp),
                    modifier = Modifier
                        .weight(1f)
                        .height(52.dp)
                ) {
                    Icon(
                        imageVector = if (isMuted) Icons.Default.MicOff else Icons.Default.Mic,
                        contentDescription = "Silenciar Micrófono"
                    )
                }
            }

            // Capsule Selector
            CapsuleSelector(
                selectedCapsule = selectedCapsule,
                onCapsuleSelected = { capsule ->
                    selectedCapsule = capsule
                    AudioEngineBridge.nativeSetMicrophoneDirection(capsule)
                }
            )

            // Sidetone Monitoring Section
            Surface(
                modifier = Modifier.fillMaxWidth(),
                shape = RoundedCornerShape(10.dp),
                color = Color(0xFF1E1E1E)
            ) {
                Column(modifier = Modifier.padding(12.dp)) {
                    Row(
                        modifier = Modifier.fillMaxWidth(),
                        horizontalArrangement = Arrangement.SpaceBetween,
                        verticalAlignment = Alignment.CenterVertically
                    ) {
                        Column {
                            Text(
                                text = "Retorno en Audífonos (Sidetone)",
                                fontWeight = FontWeight.SemiBold,
                                fontSize = 14.sp
                            )
                            Text(
                                text = if (isHeadphonesPlugged) "🎧 Auriculares detectados (< 2 ms)" else "⚠️ Conecta auriculares para activar",
                                fontSize = 12.sp,
                                color = if (isHeadphonesPlugged) Color(0xFF00E676) else Color.Gray
                            )
                        }
                        Switch(
                            checked = sidetoneEnabled && isHeadphonesPlugged,
                            onCheckedChange = { enabled ->
                                if (isHeadphonesPlugged) {
                                    sidetoneEnabled = enabled
                                    AudioEngineBridge.nativeSetSidetone(enabled, sidetoneVolume)
                                }
                            },
                            enabled = isHeadphonesPlugged
                        )
                    }

                    if (sidetoneEnabled && isHeadphonesPlugged) {
                        Spacer(modifier = Modifier.height(8.dp))
                        Row(
                            modifier = Modifier.fillMaxWidth(),
                            verticalAlignment = Alignment.CenterVertically
                        ) {
                            Text("Vol:", fontSize = 12.sp, color = Color.Gray)
                            Slider(
                                value = sidetoneVolume,
                                onValueChange = { vol ->
                                    sidetoneVolume = vol
                                    AudioEngineBridge.nativeSetSidetone(true, vol)
                                },
                                valueRange = 0f..1f,
                                modifier = Modifier.padding(start = 8.dp)
                            )
                        }
                    }
                }
            }

            // Stream Deck Interactive FX Grid
            StreamDeckGrid(
                activeEffectId = activeEffectId,
                onEffectSelected = { effectId ->
                    activeEffectId = effectId
                }
            )
        }
    }

    // Connect / Fallback Dialog
    if (showConnectDialog) {
        ConnectDialog(
            initialIp = targetIp,
            initialPort = targetPort,
            discoveredServers = discoveredServers,
            onConnect = { ip, port ->
                targetIp = ip
                targetPort = port
                showConnectDialog = false
                if (isStreaming) {
                    onStopStreaming()
                    onStartStreaming(ip, port)
                }
            },
            onDismiss = { showConnectDialog = false }
        )
    }
}

@Composable
fun PocketMicTheme(content: @Composable () -> Unit) {
    MaterialTheme(
        colorScheme = darkColorScheme(
            primary = Color(0xFF6200EE),
            secondary = Color(0xFF03DAC6),
            background = Color(0xFF121212),
            surface = Color(0xFF1E1E1E),
            surfaceVariant = Color(0xFF2A2A2A)
        ),
        content = content
    )
}
