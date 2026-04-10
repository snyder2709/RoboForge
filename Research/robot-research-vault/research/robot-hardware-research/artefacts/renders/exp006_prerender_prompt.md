# Промт пререндера — EXP-006 Hardware-in-the-Loop

Связан с: [[../../experiments/EXP-006_hardware_in_the_loop|EXP-006]]
Модель: Gemini Imagen / Gemini Flash (image generation)

---

## Промт (EN)

```
Photorealistic technical render, studio lighting, dark background.

SCENE: A wooden workbench. Two setups side by side, connected by the same story.

LEFT SIDE — PHYSICAL HARDWARE PLATFORM:
A flat wooden board (~20×10 cm, 15 mm thick) screwed to the bench.
Mounted on the board: two MG996R metal-gear servos (55g each, brown/black body, orange-yellow top cap, three-wire connector: red=power, brown=GND, orange=signal) connected by aluminum U-shaped servo brackets forming a simple leg:
- Bottom servo (hip) is bolted horizontally to the board via U-bracket. Its output shaft points upward.
- A short aluminum link (~10 cm) connects hip servo horn to the second U-bracket.
- Top servo (knee) is mounted at the far end of the link via second U-bracket. Its shaft points sideways.
- A small flat acrylic piece (~5×8 cm) is attached to the knee servo horn as the foot.
The leg is in a slightly bent position (hip ~30°, knee ~45°) to show movement.

The servos' three-wire cables (red, brown, orange) run along the board with small zip ties and connect to a PCA9685 PWM driver module (blue PCB, 16 channels, PCA9685 chip visible, two 2×8 pin headers for outputs, one 4-pin I2C header).

The PCA9685 sits beside the board. Its power terminal (two-screw block labeled V+ and GND) connects to a small external DC power supply brick (5V 3A, black box, labeled "5V 3A", with a barrel jack connector — NOT a USB cable).

The PCA9685 I2C header (4 pins: VCC, GND, SDA, SCL) connects via four Dupont female-to-female jumper wires (red=VCC, black=GND, yellow=SDA, blue=SCL) to a Raspberry Pi 2 Model B+ (green PCB, ~85×56 mm, 4× USB-A ports on right side, Ethernet port, 40-pin GPIO header on top edge).

CRITICAL: The Dupont jumper wires connect ONLY to the 40-PIN GPIO HEADER on the Raspberry Pi. The USB-A ports are COMPLETELY EMPTY — no cables plugged into USB ports. The GPIO header pins used are: Pin 1 (3.3V), Pin 6 (GND), Pin 3 (SDA), Pin 5 (SCL).

The Raspberry Pi is powered separately by a micro-USB cable going off-frame (its own charger, NOT the servo PSU).
An Ethernet cable from the Pi's RJ45 port goes off to the right (toward the router).

Small handwritten label stickers on the board: "HIP CH6", "KNEE CH7".

RIGHT SIDE — LAPTOP SCREEN:
A Lenovo laptop (open, 15-inch) showing split-screen:
- LEFT HALF of screen: Webots R2025a 3D simulation — a small 28 cm humanoid robot in T-pose on a flat grey ground, the left leg slightly bent matching the physical hardware pose, robot body is white/grey with colored joint indicators
- RIGHT HALF of screen: Browser (Chrome) with a dark-themed debug UI showing two vertical sliders labeled "left_hip_pitch" and "left_knee_pitch" with green value indicators showing "-0.52 rad" and "0.79 rad"
- A small terminal window in the corner shows scrolling Zenoh log output: "robot/joints received | hip: -0.523 | knee: 0.785"

COMPOSITION: Shallow depth of field. Physical hardware in sharp focus in foreground. Laptop slightly behind. A thin green arrow overlay (like a diagram annotation) connects from the laptop screen → to the PCA9685 → to the servos, labeled "Zenoh pub robot/joints → PWM → motion".

LIGHTING: Warm workbench lamp from above-left. Dark background. Professional product photography style. No clutter.

STYLE: Photorealistic, 4K quality, technical documentation render.
```

---

## Ключевые точности (не нарушать)

| Компонент | Деталь |
|-----------|--------|
| RPi GPIO | Jumper wires → 40-pin header ТОЛЬКО, USB порты пустые |
| MG996R | brown/black body, orange cap, 3 wire: red/brown/orange |
| Питание серво | DC barrel-jack PSU 5V 3A, НЕ USB кабель |
| PCA9685 | V+ terminal → серво питание, I2C 4-pin → RPi GPIO |
| Нога | Два U-кронштейна + алюминиевое звено, сгибание бедро+колено |
| Экран | Webots слева + Browser UI справа, одинаковый угол ноги |

## Итерации

| Версия | Дата | Изменение |
|--------|------|-----------|
| v1 | 2026-04-10 | Первый промт EXP-006, деревянная платформа + ноги |
