# Proyecto 2: Diorama con Ray Tracing

## Descripción

Experiencia interactiva escrita en Rust que recrea un diorama voxel inspirado en una isla flotante y en los personajes Polar, Pardo y Panda. La imagen se calcula con un ray tracer CPU propio: no se utiliza un motor de juegos ni se pegan las imágenes de referencia como fondos.

El recorrido comienza en una isla flotante. Al acercar la cámara se abre la selección tridimensional de personajes; un clic sobre cada oso conduce a su mundo temático.

## Ejecución

Se recomienda compilar en modo release porque cada píxel lanza rayos contra la escena:

```powershell
cargo run --release
```

Validación del proyecto:

```powershell
cargo fmt --check
cargo check
cargo test
cargo clippy -- -D warnings
```

## Características

- Cinco escenas voxel: isla principal, selección y tres mundos individuales.
- Cámara orbital con mouse y teclado.
- Selección 3D mediante un rayo generado desde el cursor y volúmenes AABB.
- Iluminación ambient, diffuse y specular con luces direccionales y puntuales.
- Sombras mediante shadow rays con bias.
- Reflexiones recursivas.
- Refracción con Ley de Snell, reflexión interna total y Fresnel-Schlick.
- Skyboxes procedurales diferentes por ambiente.
- Texturas procedurales sin archivos externos.
- Cubos orientados para rotar únicamente al personaje.
- BVH para acelerar las intersecciones de los mundos voxel.

## Controles

| Entrada | Acción |
|---|---|
| Arrastrar con clic izquierdo | Orbitar alrededor del diorama |
| Rueda del mouse | Acercar o alejar |
| `A` / `D` | Rotar yaw |
| `Q` / `E` | Rotar pitch |
| `W` / `S` | Acercar o alejar |
| Clic sobre un personaje | Entrar a su mundo |
| `R` | Activar o desactivar la rotación automática del personaje |
| `ESC` | Mundo individual → selección; selección → isla principal |
| Cerrar ventana | Salir de la aplicación |

En la escena principal hay que acercarse hasta el diorama para activar la transición. En selección, el pedestal luminoso indica qué personaje recibirá el clic.

## Arquitectura

```text
src/
├── app/          ciclo de ventana, entradas y máquina de estados
├── camera/       cámara orbital y rayos desde pantalla
├── characters/   constructores modulares de Polar, Pardo y Panda
├── geometry/     AABB, cubos y cubos orientados
├── lighting/     luces puntuales y direccionales
├── material/     color, materiales y texturas procedurales
├── math/         Ray, reflexión, refracción y Fresnel
├── renderer/     trazador recursivo y framebuffer
└── scene/        BVH, paleta, helpers y cinco escenas
```

`Scene` contiene cubos, materiales, luces, ambiente, skybox y su BVH. Los constructores de escena usan `VoxelBuilder`; los modelos no son listas globales de llamadas y cada personaje separa cuerpo, cabeza, extremidades, rostro y accesorios.

La máquina de estados utiliza:

```text
MainWorld
   ↓ acercamiento
CharacterSelection
   ├── PolarWorld
   ├── PardoWorld
   └── PandaWorld
```

`Transition` actúa como estado de debounce para impedir cambios repetidos o accidentales.

## Ray Tracing

Para cada píxel, la cámara transforma su coordenada de pantalla en un rayo mundial. El BVH descarta grupos de voxels; las hojas prueban la intersección exacta contra cubos orientados. El impacto más cercano conserva punto, normal, UV procedural y material.

El sombreado directo suma:

1. ambiente;
2. diffuse con `max(N · L, 0)`;
3. specular tipo Phong;
4. emisión del material.

Cada luz lanza un shadow ray desde el punto desplazado por un pequeño bias. Una obstrucción anterior a la luz elimina sus componentes diffuse y specular.

## Reflexión

La dirección reflejada se calcula con:

```text
R = D - 2(D · N)N
```

Los rayos reflejados se trazan recursivamente hasta una profundidad máxima de tres rebotes. La reflectividad del material controla la mezcla. Agua, hielo, vidrio y metal utilizan valores distintos.

## Refracción

La transmisión implementa la Ley de Snell y diferencia si el rayo entra o sale del material. Si no existe solución real se produce reflexión interna total. Los índices principales son:

- agua: 1.333;
- hielo: 1.31;
- vidrio: 1.5.

Fresnel-Schlick distribuye la energía entre reflexión y refracción:

```text
R0 = ((n1 - n2) / (n1 + n2))²
R(θ) = R0 + (1 - R0)(1 - cos θ)⁵
```

## Materiales

Todos los materiales tienen albedo, textura, specular, transparencia, reflectividad e índice de refracción. La tabla resume los materiales principales; la paleta añade variantes de follaje, tierra, nieve, pelaje, bambú y emisión.

| Material | Textura | Albedo aproximado | Specular | Transparency | Reflectivity | IOR |
|---|---|---:|---:|---:|---:|---:|
| Grass | Checker | `#4A7E2D` | 0.15 | 0.00 | 0.00 | 1.00 |
| Stone | Speckled | `#747674` | 0.18 | 0.00 | 0.04 | 1.00 |
| Wood | Grain | `#8B4F26` | 0.22 | 0.00 | 0.02 | 1.00 |
| Metal | Checker | `#767E89` | 1.00 | 0.00 | 0.68 | 1.00 |
| Ice | Checker | `#A5E0EF` | 0.95 | 0.78 | 0.28 | 1.31 |
| Water | Ripples | `#2E8BAD` | 0.90 | 0.72 | 0.22 | 1.333 |
| Glass | Solid | `#CDEBEB` | 1.00 | 0.82 | 0.20 | 1.50 |
| Snow | Speckled | `#EBF4F4` | 0.35 | 0.00 | 0.10 | 1.00 |

## Escenas

### Main World

Isla flotante estratificada con cabaña, techo, porche, ventanas, chimenea, pinos, rocas, vegetación, laguna, cascada y farol.

### Character Selection

Polar, Pardo y Panda sobre pedestales independientes. El hover se determina mediante ray casting y activa un halo emisivo; no hay botones 2D.

### Polar World

Nieve, lago, placas de hielo, cueva glaciar, pinos nevados y cristales. Es la demostración principal de refracción en agua e hielo.

### Pardo World

Bosque cálido con cabaña, sendero, cerca, piedra, madera, metal, ventanas y faroles.

### Panda World

Bosque de bambú segmentado, arroyo, puente de madera, camino de losas, árboles, rocas y linternas.

## Rendimiento

Las escenas construyen un BVH por mediana con hojas pequeñas. En la muestra de desarrollo de 83 cubos y 32,400 píxeles, el tiempo bajó de aproximadamente 0.077 s a 0.025 s, manteniendo exactamente los mismos píxeles. El BVH se reconstruye al rotar el personaje, no por cada rayo.

## Referencias visuales

Las seis imágenes proporcionadas se utilizaron únicamente como inspiración para composición, siluetas, colores y ambientación. Toda la geometría visible se genera con cubos, materiales y luces del ray tracer.

Los conceptos de rayos, materiales, cámara orbital, iluminación, sombras y reflexiones siguen el enfoque visto en las ramas `13-RT-01-RAYS` a `18-RT-06-REFLECTIONS` del material del curso, adaptado a una arquitectura modular y al proyecto final.

## Video de demostración

[Pendiente colocar enlace]
