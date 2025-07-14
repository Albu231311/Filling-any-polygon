# Algoritmo de Relleno de Polígonos

Este laboratorio implementa un algoritmo de relleno de polígonos desarrollado como parte del curso de Gráficas por Computadora. El algoritmo es capaz de rellenar polígonos complejos de más de 4 puntos, incluyendo el manejo de polígonos con agujeros internos.

## Descripción

El programa renderiza y rellena los siguientes polígonos:

### Polígono 1
Polígono complejo de 10 vértices:
```
(165, 380) (185, 360) (180, 330) (207, 345) (233, 330) 
(230, 360) (250, 380) (220, 385) (205, 410) (193, 383)
```

### Polígono 2
Cuadrilátero:
```
(321, 335) (288, 286) (339, 251) (374, 302)
```

### Polígono 3
Triángulo:
```
(377, 249) (411, 197) (436, 249)
```

### Polígono 4
Polígono complejo de 18 vértices:
```
(413, 177) (448, 159) (502, 88) (553, 53) (535, 36) (676, 37) (660, 52)
(750, 145) (761, 179) (672, 192) (659, 214) (615, 214) (632, 230) (580, 230)
(597, 215) (552, 214) (517, 144) (466, 180)
```

### Polígono 5 (Agujero)
Cuadrilátero que actúa como agujero dentro del Polígono 4:
```
(682, 175) (708, 120) (735, 148) (739, 170)
```

## Instalación y Ejecución

### Prerrequisitos
- Rust (versión 1.70 o superior)
- Cargo (incluido con Rust)

### Pasos para ejecutar

1. **Clonar el repositorio**
   ```bash
   git clone https://github.com/Albu231311/Filling-any-polygon.git
   ```

2. **Navegar al directorio del proyecto**
   ```bash
   cd Filling-any-polygon
   ```

3. **Ejecutar el programa**
   ```bash
   cargo run
   ```

El programa se compilará automáticamente y ejecutará el algoritmo de relleno de polígonos, mostrando el resultado en la generación de una imagen representantiva como out.png.

## Tecnologías Utilizadas

- **Rust**: Lenguaje de programación principal
- **Cargo**: Sistema de gestión de paquetes y construcción
