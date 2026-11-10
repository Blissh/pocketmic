package io.pocketmic.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import io.pocketmic.core.AudioEngineBridge

data class FxPad(
    val id: Int,
    val name: String,
    val icon: String,
    val mode: Int,
    val param1: Int,
    val color: Color
)

@Composable
fun StreamDeckGrid(
    activeEffectId: Int,
    onEffectSelected: (Int) -> Unit,
    modifier: Modifier = Modifier
) {
    var isBleepActive by remember { mutableStateOf(false) }

    val pads = remember {
        listOf(
            FxPad(0x00, "Voz Normal", "🎙️", 0x01, 0, Color(0xFF2C3E50)),
            FxPad(0x01, "Autotune", "🎵", 0x01, 0x0309, Color(0xFF8E44AD)),
            FxPad(0x02, "Anime Girl", "🎀", 0x01, 120, Color(0xFFE91E63)),
            FxPad(0x03, "Ardilla", "🐿️", 0x01, 80, Color(0xFFF39C12)),
            FxPad(0x04, "Demonio", "👹", 0x01, 0xFFB0, Color(0xFFC0392B)),
            FxPad(0x05, "Radio Táctica", "📻", 0x01, 1, Color(0xFF27AE60)),
            FxPad(0x06, "Robot", "🤖", 0x01, 75, Color(0xFF16A085)),
            FxPad(0x08, "Reverb Catedral", "🏛️", 0x01, 85, Color(0xFF2980B9))
        )
    }

    Column(
        modifier = modifier.fillMaxWidth(),
        verticalArrangement = Arrangement.spacedBy(10.dp)
    ) {
        Text(
            text = "Efectos en Tiempo Real (Stream Deck)",
            style = MaterialTheme.typography.labelLarge,
            color = Color.LightGray
        )

        // 4x2 Grid of Effect Pads
        Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
            pads.chunked(2).forEach { rowPads ->
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    rowPads.forEach { pad ->
                        val isSelected = activeEffectId == pad.id
                        Button(
                            onClick = {
                                onEffectSelected(pad.id)
                                AudioEngineBridge.nativeSendFxCommand(pad.id, pad.mode, pad.param1)
                            },
                            colors = ButtonDefaults.buttonColors(
                                containerColor = if (isSelected) pad.color else pad.color.copy(alpha = 0.35f)
                            ),
                            shape = RoundedCornerShape(12.dp),
                            modifier = Modifier
                                .weight(1f)
                                .height(56.dp)
                        ) {
                            Row(
                                verticalAlignment = Alignment.CenterVertically,
                                horizontalArrangement = Arrangement.Center
                            ) {
                                Text(text = pad.icon, fontSize = 20.sp)
                                Spacer(modifier = Modifier.width(8.dp))
                                Text(
                                    text = pad.name,
                                    fontSize = 13.sp,
                                    fontWeight = if (isSelected) FontWeight.Bold else FontWeight.Normal,
                                    color = if (isSelected) Color.White else Color.LightGray,
                                    textAlign = TextAlign.Start
                                )
                            }
                        }
                    }
                }
            }
        }

        Spacer(modifier = Modifier.height(6.dp))

        // Giant Push-to-Bleep Button
        Box(
            contentAlignment = Alignment.Center,
            modifier = Modifier
                .fillMaxWidth()
                .height(64.dp)
                .background(
                    color = if (isBleepActive) Color(0xFFFF1744) else Color(0xFF7F0000),
                    shape = RoundedCornerShape(14.dp)
                )
                .pointerInput(Unit) {
                    detectTapGestures(
                        onPress = {
                            isBleepActive = true
                            AudioEngineBridge.nativeSendFxCommand(0x09, 0x02, 1) // Bleep ON
                            try {
                                tryAwaitRelease()
                            } finally {
                                isBleepActive = false
                                AudioEngineBridge.nativeSendFxCommand(0x09, 0x02, 0) // Bleep OFF
                            }
                        }
                    )
                }
        ) {
            Row(
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.Center
            ) {
                Text(
                    text = if (isBleepActive) "⚠️ PITIDO ACTIVO (CENSURANDO)" else "🤬 MANTENER PARA CENSURAR (BLEEP)",
                    color = Color.White,
                    fontWeight = FontWeight.Bold,
                    fontSize = 14.sp
                )
            }
        }
    }
}
