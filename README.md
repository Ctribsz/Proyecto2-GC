
# Diorama Minecraft
Teclas para poder navegar y cambiar iluminación del diorama

## Cambios de luz

```javascript
1 = luz calida 
2 = luz azul 
3 = luz verde
```
## Movimiento de camara
```javascript
W = Movimiento hacia arriba
S = Movimiento hacia abajo
D = Movimiento hacia la derecha
A = Movimiento hacia la izquierda
```

## Deployment
To deploy this project run

```bash
cargo run --release
```

## Environment Variables

Para poder correr el proyecto en menor calidad o mejor calidad para computadoras de diferentes caracteristicas se puede cambiar `FRAMEBUFFER_SCALE_FACTOR` en el archivo de `constants.rs`. 
Para poder cambiar la intensidad de luz puedes incrementar o reducir `INTENSITY` en el archivo de `constants.rs`.

## Demostración diorama
![Demo del proyecto](assets/Diorama.gif)

