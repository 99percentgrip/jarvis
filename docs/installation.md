# Install and manage Vesper

[Documentation](README.md) · [Using Vesper](using-vesper.md) · [Zed setup](zed.md)

## Before you install

Supported targets are Linux x86_64 and ARM64, macOS Intel and Apple Silicon, and Windows x86_64. There is no public package to download. Do not pipe a remote script and do not point the installer at another product's archive.

A local archive and its SHA-256 digest are required. Python 3 is required on Linux and macOS because the installer validates the archive before touching a destination. Windows uses PowerShell 5.1 or 7 and does not download the package.

Compiling from source is separate from installing a prebuilt archive. On Fedora, a source build can require the `dbus-devel` package so `dbus-1.pc` exists. A prebuilt binary needs the matching shared libraries at runtime, not the compiler headers. Optional desktop credential storage uses the Secret Service when it is present. Missing optional services are reported; they are not installed silently.

## Install

Linux and macOS:

```sh
JARVIS_PACKAGE=./jarvis-linux-x86_64.tar.gz \
JARVIS_PACKAGE_SHA256=<64-hex-or-checksum-file> \
sh scripts/install.sh
```

Use the archive name that matches the machine: `jarvis-linux-x86_64.tar.gz`, `jarvis-linux-aarch64.tar.gz`, `jarvis-darwin-x86_64.tar.gz`, or `jarvis-darwin-aarch64.tar.gz`.

Windows PowerShell:

```powershell
$env:JARVIS_PACKAGE = '.\jarvis-windows-x86_64.zip'
$env:JARVIS_PACKAGE_SHA256 = '<digest>'
powershell.exe -NoProfile -ExecutionPolicy Bypass -File .\scripts\install.ps1
```

Without `JARVIS_PACKAGE` and `JARVIS_PACKAGE_SHA256`, the installer stops. A checksum only proves the archive matches the digest you supplied. It does not prove that an unsigned local file is an official release. Signing and publication are later work.

Package files go to the install root from the platform contract: `$XDG_DATA_HOME/jarvis/install` (or `~/.local/share/jarvis/install`), `~/Library/Application Support/Jarvis/Install`, or `%LOCALAPPDATA%/Programs/Jarvis`. Launchers on Linux and macOS are written only into the shared user bin directory. Memory stays at `JARVIS_MEMORY_ROOT` or `~/.jarvis/memory` and is not inside the package generation. The installer will not use a Vesper path, a symlink install root, or a directory it does not own.

### Verify and launch

Open a new terminal if necessary, then run:

```sh
jarvis-acp --version
jarvis-tui --version
cd path/to/your-project
jarvis-tui
```

Both commands should report a version and exit successfully. The terminal's first launch opens the welcome screen before sign-in. Open **Settings → Providers** when you want to authenticate. Cancelling returns to the caller.

A package must contain the terminal app and the ACP server. The web-fetch helper, sandbox supervisor, seed skills, and browser-driver archive are included when the package lists them. The installer does not import a container image by itself and does not start Docker or Podman.

## Optional dependencies

### Local models

Install [LM Studio](https://lmstudio.ai/docs/app), download and load a model suitable for coding and tool use, and start its local server. In Vesper's provider settings, select LM Studio and configure the server address and model. Model download, hardware requirements, and supported capabilities depend on the model you choose.

### Web tools and container workers

Starting with v0.23.0, open **Settings → Web tools → Set up features / repair**.
Setup needs the complete browser bundle; compiling the executable alone does
not include the browser archive. Release packaging supplies it automatically.
Confirm setup and approve any operating-system authorization prompt. Vesper reuses
an available local Docker/Podman engine, or offers Podman installation. It then
imports the bundled image and checks contained browser execution and cleanup.
Save your web choices separately and restart the host when prompted.

Automatic package plans cover Debian/Ubuntu and Fedora (with a graphical `pkexec`
authorization agent), official Podman packages on macOS, and Windows x86_64.
macOS/Windows need a Linux VM; a new Vesper machine uses 2 CPUs, 2 GiB RAM and
up to 20 GiB disk. Downloads can exceed 1 GiB. Windows may require OS virtualization
approval and a restart; reopen Vesper and repeat setup afterward. Existing machines
are never reset. A previously verified Vesper machine can start again when needed.

The guided flow is a **preview**. Its real Linux contained-browser check has
passed; clean-machine package installation and macOS/Windows acceptance remain
pending. See the [acceptance record](foundation/dependency-setup-execution.md).
Older builds use **Set up / repair driver** after installing and starting
[Docker](https://docs.docker.com/get-started/get-docker/) or
[Podman](https://podman.io/docs/installation) through the official OS instructions.

Esc requests a stop after the current setup transaction. Installed packages/imports
remain available for Retry; unsaved web toggles stay unsaved. Core coding remains
available when setup is declined or blocked by device policy. Voice, local models,
embeddings and project build tools have separate requirements.

Web access and browser interaction are opt-in. See [web tools](web-tools.md) for permissions, settings, and supported operations. Multi-worker execution additionally needs configured embeddings and an available permitted sandbox backend; see [using workers](using-vesper.md#reasoning-and-workers).

### Voice input

Voice input is optional in the terminal app. Linux uses `arecord`; Windows and macOS use native microphone streams. The operating system must permit microphone access for your terminal. Open **Settings → Voice → Set up local voice dependencies** to verify or repair local recognition and pronunciation tools. Confirmed setup reuses healthy components and shows progress; your OS may ask for administrator approval or developer tools. Text coding remains available if setup is declined.

Starting with v0.23.0, click the red **● Push to talk** control (or press **F5**)
to start recording. Click **■ Stop** or press F5 again to transcribe. Recording
continues until Stop or the managed 120-second/4-MiB per-capture bound, with
elapsed time shown. The store also enforces a 32-MiB cross-instance aggregate
cap and a 1-GiB free-space reserve. Transcription shows progress and keeps the composer editable.
Press F5 while preparing/transcribing to cancel. Failed or cancelled transcription
retains private audio for **Retry voice** or **Discard** (Delete); it is removed
on success, discard or normal app exit. Completed dictation is appended to the
composer for review and is never sent automatically.

Guided setup prepares a private native-layout Python environment with `faster-whisper` and installs missing `espeak-ng` pronunciation tools. Windows setup also checks the C++ runtime required by the optional speech libraries. Downloads have fixed integrity checks; OS approval and restart requirements remain explicit. Recognition may download the selected model on first use. Existing Linux/macOS first-use environment preparation remains available.

If recording is unavailable, check the named error and system microphone permissions, then use guided setup or Retry. Failed audio remains available for Retry/Discard. Voice setup is separate from ordinary text coding.

For conversation, open **Settings → Voice**, turn **Voice conversation** on,
choose recognition/synthesis compute and a speech engine, then leave Settings
and choose **Save changes**. Press F9 once to start capture and again to Stop and
submit the final transcript as one normal turn. Current CPU and FLM recognizers
are final-only: the **Live transcript preview** row says that preview is not
supported while confirming that final text still appears after Stop. F9 during
spoken output performs barge-in; it stops old speech and begins replacement
capture in the same gesture. Ctrl+C / explicit Stop cancels the current voice
work without opening capture.

On compatible AMD Linux systems with FLM and its Whisper model already present,
use **Accelerated recognition (FLM NPU) → Verify**. Verify checks the device,
runtime, model and local recognition path without recording or downloading.
CPU recognition remains available when accelerated recognition is not selected
or supported. Vesper does not provide NPU speech synthesis.

### Optional Natural Voice pack

Voice conversation can speak through the system speech engine. For local neural speech, open Settings → Voice → *Natural Voice pack* → **Install voice pack**. The confirmation shows the exact download, installed size, available storage and peak extra space for your platform; setup refuses unmeasurable capacity. Linux, Windows, Intel macOS and Apple Silicon have separately verified runtime packages. Progress shows actual bytes and supports Esc cancellation. Every file is verified against its pinned source and a silent synthesis check must pass before Ready. Installation does not select the engine, enable conversation or play audio.

After installation, choose **Speech engine → Neural voice**, pick Heart or Michael, and leave Settings with **Save changes**. **Preview voice** plays a fixed phrase without submitting anything to the agent. Windows/macOS use native output streams; Linux retains its audio player. **Repair / Verify** revalidates the pack; **Remove voice pack** deletes only pack-owned files and refuses while another Vesper process is using the pack. Speech runs locally and does not change your coding provider. Removing the selected pack makes that voice unavailable until you choose another engine and save.

Pronunciation uses `espeak-ng`; guided setup offers its installation or repair when missing. Licenses: model/voices/export Apache-2.0, ONNX Runtime MIT, espeak-ng GPL-3.0 (external system component). Optional uv is MIT/Apache-2.0; Windows runtime installation uses Microsoft's official redistributable.

All shipped STT/TTS routes are local or explicitly configured self-hosted
routes. Vesper v1 includes no third-party cloud speech provider. Local speech
never silently falls back to cloud; future optional cloud adapters must use the
existing credential and egress controls, and cloud TTS must pass mandatory text
hygiene/redaction before transmission. Local speech does not make the selected
reasoning provider local: the final transcript and ordinary agent context still
go to that provider. ACP editor sessions have no microphone/audio transport,
speaker playback or Voice Settings surface; use the terminal app for voice.

## Update or choose a version

There is no public update channel. To upgrade, run the installer again with a newer local archive and its digest. The previous generation stays available for local rollback (`sh scripts/install.sh --rollback` or `JARVIS_INSTALL_ACTION=rollback` on Windows). Rollback restores package files only. It does not roll back memory, sessions, or credentials, and it does not contact an update service.

Upgrades keep the memory directory and existing skill edits. Seed skills that were deleted stay deleted.

## Install locations

| Item | Default location |
|---|---|
| Linux/macOS launchers | `~/.local/bin/jarvis-tui` and `~/.local/bin/jarvis-acp` |
| Linux install root | `$XDG_DATA_HOME/jarvis/install`, normally `~/.local/share/jarvis/install` |
| macOS install root | `~/Library/Application Support/Jarvis/Install` |
| Windows install root and launchers | `%LOCALAPPDATA%/Programs/Jarvis` |
| Memory | `~/.jarvis/memory`, or `JARVIS_MEMORY_ROOT` |

Overrides: `JARVIS_INSTALL_ROOT`, `JARVIS_LAUNCHER_DIR`, `JARVIS_MEMORY_ROOT`, `JARVIS_HOME`, and `JARVIS_SHELL_PROFILE`. A custom path cannot point at a Vesper location or a directory the installer does not own. Use the same overrides when uninstalling.

## Troubleshooting

| Symptom | What to check |
|---|---|
| Command not found | Reopen the terminal. Check the launcher directory above is on PATH. A desktop editor may need an absolute executable path. |
| Wrong version starts | Run `command -v jarvis-tui` on Linux/macOS or `Get-Command jarvis-tui` in PowerShell. Check for an older installation earlier on PATH. |
| Checksum verification fails | The archive does not match the supplied digest. Do not bypass the checksum. |
| Windows reports an execution-policy error | Run the installer with `-ExecutionPolicy Bypass` for that process only, or follow organizational policy. Do not set `Unrestricted`. |
| Windows reports unsupported or unknown architecture | Check `[Environment]::Is64BitOperatingSystem`, `$env:PROCESSOR_ARCHITECTURE` and `$env:PROCESSOR_ARCHITEW6432`. Only Windows x86_64 (AMD64) has a published archive; ARM64 is not a release target. |
| Web driver unavailable | Start Docker/Podman, confirm `info` succeeds, then use Settings → Web tools → Set up / repair driver. |
| Provider rejects a request | Check authentication, model entitlement, account limits, and the provider selected in Settings. |
| LM Studio cannot connect | Confirm its server is running and the address in Vesper matches it. |

## Uninstall

```sh
sh scripts/uninstall.sh
```

```powershell
powershell.exe -NoProfile -ExecutionPolicy Bypass -File .\scripts\uninstall.ps1
```

Uninstall removes installer-owned generations and launchers recorded by the ownership file. It does not delete `~/.jarvis/memory`, sessions, credentials, checkpoints, or another product's files. It does not remove a shared bin directory or a PATH entry it did not add. There is no purge-user-data switch.
