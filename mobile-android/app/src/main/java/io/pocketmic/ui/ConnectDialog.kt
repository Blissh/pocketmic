package io.pocketmic.ui

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import io.pocketmic.net.DiscoveredServer

@Composable
fun ConnectDialog(
    initialIp: String,
    initialPort: Int,
    discoveredServers: List<DiscoveredServer>,
    onConnect: (ip: String, port: Int) -> Unit,
    onDismiss: () -> Unit
) {
    var ipText by remember { mutableStateOf(initialIp) }
    var portText by remember { mutableStateOf(initialPort.toString()) }
    var errorMessage by remember { mutableStateOf<String?>(null) }

    AlertDialog(
        onDismissRequest = onDismiss,
        title = {
            Text(
                text = "Conectar a PC Host",
                fontWeight = FontWeight.Bold,
                fontSize = 18.sp
            )
        },
        text = {
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(vertical = 4.dp),
                verticalArrangement = Arrangement.spacedBy(12.dp)
            ) {
                if (discoveredServers.isNotEmpty()) {
                    Text(
                        text = "Servidores detectados en red local:",
                        style = MaterialTheme.typography.labelMedium,
                        color = Color.LightGray
                    )
                    discoveredServers.forEach { srv ->
                        Surface(
                            modifier = Modifier
                                .fillMaxWidth()
                                .clickable {
                                    ipText = srv.ip
                                    portText = srv.port.toString()
                                },
                            shape = RoundedCornerShape(8.dp),
                            color = MaterialTheme.colorScheme.surfaceVariant
                        ) {
                            Row(
                                modifier = Modifier
                                    .padding(8.dp)
                                    .fillMaxWidth(),
                                horizontalArrangement = Arrangement.SpaceBetween,
                                verticalAlignment = Alignment.CenterVertically
                            ) {
                                Column {
                                    Text(
                                        text = srv.serverName,
                                        fontWeight = FontWeight.SemiBold,
                                        fontSize = 14.sp
                                    )
                                    Text(
                                        text = "${srv.ip}:${srv.port}",
                                        fontSize = 12.sp,
                                        color = Color.Gray
                                    )
                                }
                                Text(
                                    text = "Usar",
                                    color = MaterialTheme.colorScheme.primary,
                                    fontSize = 12.sp,
                                    fontWeight = FontWeight.Bold
                                )
                            }
                        }
                    }
                    Divider(color = Color.DarkGray, modifier = Modifier.padding(vertical = 4.dp))
                }

                Text(
                    text = "Configuración Manual (Fallback IP):",
                    style = MaterialTheme.typography.labelMedium,
                    color = Color.LightGray
                )

                OutlinedTextField(
                    value = ipText,
                    onValueChange = {
                        ipText = it
                        errorMessage = null
                    },
                    label = { Text("Dirección IP de la PC") },
                    singleLine = true,
                    placeholder = { Text("192.168.1.100") },
                    keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
                    modifier = Modifier.fillMaxWidth()
                )

                OutlinedTextField(
                    value = portText,
                    onValueChange = {
                        portText = it
                        errorMessage = null
                    },
                    label = { Text("Puerto UDP") },
                    singleLine = true,
                    placeholder = { Text("37719") },
                    keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
                    modifier = Modifier.fillMaxWidth()
                )

                if (errorMessage != null) {
                    Text(
                        text = errorMessage!!,
                        color = MaterialTheme.colorScheme.error,
                        fontSize = 12.sp
                    )
                }
            }
        },
        confirmButton = {
            Button(
                onClick = {
                    val port = portText.toIntOrNull()
                    if (ipText.isBlank()) {
                        errorMessage = "La dirección IP no puede estar vacía"
                    } else if (port == null || port !in 1024..65535) {
                        errorMessage = "Puerto inválido (rango 1024 - 65535)"
                    } else {
                        onConnect(ipText.trim(), port)
                    }
                }
            ) {
                Text("Conectar")
            }
        },
        dismissButton = {
            TextButton(onClick = onDismiss) {
                Text("Cancelar")
            }
        }
    )
}
