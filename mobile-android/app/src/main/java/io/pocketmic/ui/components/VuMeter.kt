package io.pocketmic.ui.components

import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp

@Composable
fun VuMeter(
    level: Float, // 0.0 to 1.0
    modifier: Modifier = Modifier
) {
    val animatedLevel by animateFloatAsState(targetValue = level.coerceIn(0f, 1f), label = "vuLevel")
    val isClipping = level > 0.95f

    val barColor = when {
        isClipping -> Color(0xFFFF3333) // Rojo alerta
        level > 0.70f -> Color(0xFFFFCC00) // Amarillo picos
        else -> Color(0xFF00E676) // Verde óptimo
    }

    Column(modifier = modifier.fillMaxWidth()) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text(
                text = if (isClipping) "⚠️ SATURACIÓN (CLIPPING)" else "Nivel de Entrada",
                fontSize = 12.sp,
                fontWeight = FontWeight.Bold,
                color = if (isClipping) Color.Red else Color.White
            )
            Text(
                text = "${(level * 100).toInt()}%",
                fontSize = 12.sp,
                color = Color.LightGray
            )
        }

        Spacer(modifier = Modifier.height(4.dp))

        Box(
            modifier = Modifier
                .fillMaxWidth()
                .height(14.dp)
                .background(Color(0xFF222222), RoundedCornerShape(7.dp))
        ) {
            Box(
                modifier = Modifier
                    .fillMaxHeight()
                    .fillMaxWidth(animatedLevel)
                    .background(barColor, RoundedCornerShape(7.dp))
            )
        }
    }
}
