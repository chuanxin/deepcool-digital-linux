> **This is a fork of [Nortank12/deepcool-digital-linux](https://github.com/Nortank12/deepcool-digital-linux)**
> with added support for the **SK700V MACH** (Vendor ID: `0x381C`).
> The SK700V MACH uses a different USB vendor and a customized HID protocol
> that required reverse engineering — see [Changes from Upstream](#changes-from-upstream) for details.

# DeepCool Digital Linux

A Linux driver for DeepCool / SK digital display devices, providing real-time
CPU temperature, power, usage, and frequency monitoring directly on your cooler's screen.

## Supported Devices

### CPU Air Coolers
<table>
    <tr>
        <th>Name</th>
        <th>Supported</th>
    </tr>
    <tr>
        <td>AG300 DIGITAL</td>
        <td align="center">✅</td>
    </tr>
    <tr>
        <td>AG400 DIGITAL</td>
        <td align="center">✅</td>
    </tr>
    <tr>
        <td>AG500 DIGITAL</td>
        <td align="center">✅</td>
    </tr>
    <tr>
        <td>AG620 DIGITAL</td>
        <td align="center">✅</td>
    </tr>
    <tr>
        <td>AK400 DIGITAL</td>
        <td align="center">✅</td>
    </tr>
    <tr>
        <td>AK400 DIGITAL PRO</td>
        <td align="center">✅</td>
    </tr>
    <tr>
        <td>AK400 G2 DIGITAL NYX</td>
        <td align="center">❓</td>
    </tr>
    <tr>
        <td>AK500 DIGITAL</td>
        <td align="center">✅</td>
    </tr>
    <tr>
        <td>AK500 DIGITAL PRO</td>
        <td align="center">✅</td>
    </tr>
    <tr>
        <td>AK500 G2 DIGITAL NYX</td>
        <td align="center">❓</td>
    </tr>
    <tr>
        <td>AK500S DIGITAL</td>
        <td align="center">✅</td>
    </tr>
    <tr>
        <td>AK620 DIGITAL</td>
        <td align="center">✅</td>
    </tr>
    <tr>
        <td>AK620 DIGITAL PRO</td>
        <td align="center">✅</td>
    </tr>
    <tr>
        <td>AK620 G2 DIGITAL NYX</td>
        <td align="center">✅</td>
    </tr>
    <tr>
        <td>AK700 DIGITAL NYX</td>
        <td align="center">✅</td>
    </tr>
    <tr>
        <td>ASSASSIN IV VC VISION</td>
        <td align="center">✅</td>
    </tr>
    <tr>
        <td><b>SK700V MACH</b> 🆕</td>
        <td align="center">✅</td>
    </tr>
</table>

### CPU Liquid Coolers
<table>
    <tr>
        <th>Name</th>
        <th>Supported</th>
    </tr>
    <tr>
        <td>LD240</td>
        <td align="center">✅</td>
    </tr>
    <tr>
        <td>LD360</td>
        <td align="center">✅</td>
    </tr>
    <tr>
        <td>LP240</td>
        <td align="center">✔️</td>
    </tr>
    <tr>
        <td>LP360</td>
        <td align="center">✔️</td>
    </tr>
    <tr>
        <td>LQ240</td>
        <td align="center">✅</td>
    </tr>
    <tr>
        <td>LQ360</td>
        <td align="center">✅</td>
    </tr>
    <tr>
        <td>LS520 SE DIGITAL</td>
        <td align="center">✅</td>
    </tr>
    <tr>
        <td>LS720 SE DIGITAL</td>
        <td align="center">✅</td>
    </tr>
</table>

### Cases
<table>
    <tr>
        <th>Name</th>
        <th>Supported</th>
    </tr>
    <tr>
        <td>CH170 DIGITAL</td>
        <td align="center">✔️</td>
    </tr>
    <tr>
        <td>CH270 DIGITAL</td>
        <td align="center">✔️</td>
    </tr>
    <tr>
        <td>CH360 DIGITAL</td>
        <td align="center">✅</td>
    </tr>
    <tr>
        <td>CH510 MESH DIGITAL</td>
        <td align="center">✅</td>
    </tr>
    <tr>
        <td>CH560 DIGITAL</td>
        <td align="center">✅</td>
    </tr>
    <tr>
        <td>CH690 DIGITAL</td>
        <td align="center">✔️</td>
    </tr>
    <tr>
        <td>MORPHEUS</td>
        <td align="center">✅</td>
    </tr>
</table>

**✅: Fully supported**

**✔️: Partially supported**<br>
*Some display modes are unavailable due to resource limitations.*

**⚠️: Not tested &nbsp; ❓: Not added**

> [!IMPORTANT]
> - If your device is not added yet, you can still run the program and see if it detects it.
> - If your device is not tested, please try to check all the features to see if they work as expected.
>
> In any case, you can create an issue or add a comment to an existing one.

### MYSTIQUE Series
These devices are unique since they have an LCD display, and I do not personally own one. However, DeadSurfer opened a [discussion](https://github.com/Nortank12/deepcool-digital-linux/discussions/18) and if you can figure out how to make it work, you can share it there or create a pull request.

# Usage
You can run the program with or without providing any options.
```bash
sudo ./deepcool-digital-linux [OPTIONS]
```
```
Options:
  -m, --mode <MODE>       Change the display mode of your device
  -s, --secondary <MODE>  Change the secondary display mode of your device (if supported)
      --pid <ID>          Specify the Product ID if multiple devices are connected
      --gpuid <VENDOR:ID> Specify the nth GPU of a specific vendor to monitor (use ID 0 for integrated GPU)

  -u, --update <MILLISEC> Change the update interval of the display [default: 1000]
  -f, --fahrenheit        Change the temperature unit to °F
  -a, --alarm             Enable the alarm
  -r, --rotate <DEGREE>   Rotate the display (LP Series only)
  -z, --zeros             Display leading zeros (LD Series only)

Commands:
  -l, --list         Print Product ID of the connected devices
  -g, --gpulist      Print all available GPUs
  -h, --help         Print help
  -v, --version      Print version
```

### SK700V MACH — Notes

The SK700V MACH uses a **separate USB Vendor ID** (`0x381C`) from DeepCool devices (`0x3633`),
so `--pid` is not required for detection. The program automatically identifies it by vendor.

The SK700V MACH displays the following on its screen:

| Display Field | Source | Notes |
|---|---|---|
| Power (W) | Intel RAPL (`energy_uj`) | EMA smoothed (α=0.4) |
| Power bar (%) | RAPL PL2 (burst limit) | Auto-detected at startup |
| Temperature | `coretemp` / `k10temp` | EMA smoothed (α=0.8) |
| CPU Usage (%) | `/proc/stat` | 1-second average |
| Frequency (MHz) | `/proc/cpuinfo` | Max core frequency |

**Power bar ceiling** is automatically read from RAPL at startup:
```
/sys/class/powercap/intel-rapl:0/constraint_1_power_limit_uw  (PL2, preferred)
/sys/class/powercap/intel-rapl:0/constraint_0_power_limit_uw  (PL1/TDP, fallback)
```
A startup message shows the detected value:
```
[SK700V] Progress bar ceiling: 219.0 W  [Intel PL2 (burst)]
```

### Using Multiple Devices <sup>(optional)</sup>
If you have multiple devices connected, you can run the following
command to detect them:
```bash
sudo ./deepcool-digital-linux --list
```
```
Device list [PID | Name]
-----
4 | AK500S-DIGITAL
7 | MORPHEUS
```
After identifying, you can run them separately by providing their Product ID:
```bash
sudo ./deepcool-digital-linux --pid 4
```
```bash
sudo ./deepcool-digital-linux --pid 7
```
If you want to run them automatically, you can create 2 services
instead of 1.

For example:
- `deepcool-digital-case.service`
- `deepcool-digital-cooler.service`

# Automatic Start

## Systemd (Arch, Debian, Ubuntu, Fedora, PVE, etc.)
1. Copy the `deepcool-digital-linux` to the `/usr/local/bin/` folder
```bash
sudo cp ./target/release/deepcool-digital-linux /usr/local/bin/
```
2. Create the service file in the `/etc/systemd/system/` folder
```bash
sudo nano /etc/systemd/system/deepcool-digital.service
```
3. Insert the following:
```properties
[Unit]
Description=DeepCool Digital

[Service]
ExecStart=/usr/local/bin/deepcool-digital-linux
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=multi-user.target
# On headless servers, use this target instead if your GPU is not detected
# WantedBy=graphical.target
```
4. Enable the service
```bash
sudo systemctl enable --now deepcool-digital
```

## OpenRC (Gentoo, Artix Linux, etc.)
1. Copy the `deepcool-digital-linux` to the `/usr/sbin/` folder
```bash
sudo cp ./deepcool-digital-linux /usr/sbin/
```
2. Create the service file in the `/etc/init.d/` folder
```bash
sudo nano /etc/init.d/deepcool-digital
```
3. Insert the following:
```properties
#!/sbin/openrc-run

description="DeepCool Digital"
command="/usr/sbin/deepcool-digital-linux"
command_args=""
command_background=1
pidfile="/run/deepcool-digital.pid"
```
4. Allow execution on the service file
```bash
sudo chmod +x /etc/init.d/deepcool-digital
```
5. Enable the service
```bash
sudo rc-update add deepcool-digital default
```

# Building from Source

## Dependencies
<details>
<summary><b>Arch-based distributions</b></summary>

1. Install the following packages
```bash
sudo pacman -S base-devel rustup
```
</details>

<details>
<summary><b>Debian-based distributions (including Proxmox VE)</b></summary>

1. Install the following packages
```bash
sudo apt install build-essential pkg-config libudev-dev curl
```
2. Install [rustup](https://rustup.rs/) (required to have the latest Rust compiler)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
3. Update your current shell
```bash
. "$HOME/.cargo/env"
```
</details>

## Building
1. Clone the repository
```bash
git clone https://github.com/chuanxin/deepcool-digital-linux
```
2. Open the folder
```bash
cd deepcool-digital-linux
```
3. Run the build command
```bash
cargo build --release
```
You can find the binary inside the `./target/release` folder.

# Changes from Upstream

This fork adds full support for the **SK700V MACH**, which was not previously supported.
The work is based on combining two projects:

- [gdedrouas/SK700V-display](https://github.com/gdedrouas/SK700V-display) — initial SK700V reverse engineering (MIT)
- [Nortank12/deepcool-digital-linux](https://github.com/Nortank12/deepcool-digital-linux) — base architecture (GPL v3)

### Reverse-Engineered SK700V MACH HID Protocol

The SK700V MACH shares a similar packet structure with the LQ/AK700 series but has key differences.
Full details are documented in [device-list/tables/sk700v.md](device-list/tables/sk700v.md).

**Summary of protocol differences vs LQ/AK700:**

| Byte | LQ / AK700 | SK700V MACH |
|:----:|-----------|------------|
| D3 | `8` | `4` |
| D4 | `12` | `13` |
| D7~D8 | Power (LE U16) | **Power (BE U16)** — bug fix |
| D9 | Temperature unit | **Power percentage** (progress bar) |
| D10 | Temperature F32 [0] | **Temperature unit** (0=°C, 1=°F) |
| D11~D14 | — | **Temperature F32** |
| D18 | Checksum | **Termination = 22** |
| Checksum | D1~D16 | **D1~D17** |
| Termination | D18 | **D19** |

### Bug Fix: Big-Endian Power Encoding

The original SK700V implementations encoded power as Little-Endian in `D8~D9`,
leaving `D7 = 0` as padding. The device firmware actually reads power as
**Big-Endian U16 from `D7~D8`**. This caused correct display only by accident
for power values below 256W.

### New Features vs Upstream

| Feature | Status |
|---------|--------|
| EMA smoothing for power (α=0.4) | ✅ Added |
| EMA smoothing for temperature (α=0.8) | ✅ Added |
| Auto-detect CPU TDP from RAPL PL2 | ✅ Added |
| Power percentage progress bar (D9) | ✅ Added |
| Temperature unit flag (D10) | ✅ Fixed |
| RAPL overflow protection | ✅ Added |

# More Information
[Device List and USB Mapping Tables](device-list/README.md)
