package io.pocketmic.ui.components

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp

@Composable
fun CapsuleSelector(
    selectedCapsule: Int, // 0 = Inferior, 1 = Superior
    onCapsuleSelected: (Int) -> Unit,
    modifier: Modifier = Modifier
) {
    Column(modifier = modifier.fillMaxWidth()) {
        Text(
            text = "Orientación de Micrófono Físico",
            style = MaterialTheme.typography.labelMedium,
            color = Color.LightGray
        )
        Spacer(modifier = Modifier.height(6.dp))
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            Button(
                onClick = { onCapsuleSelected(0) },
                colors = ButtonDefaults.buttonColors(
                    containerColor = if (selectedCapsule == 0) MaterialTheme.colorScheme.primary else Color(0xFF333333)
                ),
                modifier = Modifier.weight(1f)
            ) {
                Text("📱 Mano Alzada\n(Inferior)")
            }
            Button(
                onClick = { onCapsuleSelected(1) },
                colors = ButtonDefaults.buttonColors(
                    containerColor = if (selectedCapsule == 1) MaterialTheme.colorScheme.primary else Color(0xFF333333)
                ),
                modifier = Modifier.weight(1f)
            ) {
                Text("🎙️ En Soporte\n(Frontal/Superior)")
            }
        }
    }
}
