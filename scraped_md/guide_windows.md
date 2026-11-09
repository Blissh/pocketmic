# Source: https://micstream.io/guides/use-phone-as-mic-windows

Use Your Phone as a Microphone on Windows — Micstream

[Skip to main content](#main-content) 

[![Micstream logo](/logo/logo_light.svg)![Micstream logo](/logo/logo_dark.svg)Micstream](/) [FEATURES](../#how-it-works) [GUIDES](../guides) [DOWNLOAD](../download?src=header) [FAQ](../#faq)

[Home](../) [Guides](../guides)

Turn your phone into a Windows microphone
=========================================

Updated September 23, 2026 · 4 min read

In short

* Install the Micstream client on your PC
* Install the app on your phone and connect
* In Windows, select the new microphone called "Micstream"

Any app that lets you pick a microphone can use it, like Discord, Zoom or OBS.

Your phone probably has the best microphone you own: better than many USB mics, and far better
than the one in your laptop. The only problem: Windows can't use your phone as a microphone on
its own.

[Micstream](../) fixes this. The Windows client installs a small audio driver
that adds a microphone called "Micstream", and the phone app sends your phone's audio to it. To Windows,
it looks like a real microphone is plugged in, so every app can use it without extra setup. The audio
arrives as 32-bit float at 48 kHz.

You need three things:

* A PC running **Windows 10 or 11**
* An **Android phone** or an **iPhone**
* Both devices on the same Wi-Fi network, or a **USB cable** (Android only)

Setup
-----

1. 1

   ### Install the Windows client

   Download [the installer](https://micstream.io/downloads/MicstreamSetup.exe) and run it. It installs the desktop client and the audio driver, which creates the "Micstream"
   microphone. Your other audio devices stay as they are.
2. 2

   ### Install the phone app

   Get it on [Google Play](https://play.google.com/store/apps/details?id=com.totech.phoneasmic&referrer=utm_source%3Dwebsite%26utm_medium%3Dweb%26utm_campaign%3Dwebsite%26utm_content%3Dguide-windows) or the [App Store](https://apps.apple.com/app/id6791630989?pt=129174575&ct=guide-windows&mt=8). You don't need an account. The audio
   goes straight from your phone to your PC over your local network. It never passes through a
   server.
3. 3

   ### Connect the phone

   Open the app. On Wi-Fi, it finds your PC by itself if both are on the same network. On
   Android, you can also use a USB cable. The first time you connect, allow microphone access.
   On iPhone, also allow local network access, so the app can find your PC.
4. 4

   ### Pick Micstream as the input

   Open the microphone settings of the app you're using and select **Micstream**.
   That's it: Zoom, OBS and every other app treat it like any other microphone.

Discord and OBS ignore the Windows default and use their own setting, so select Micstream there.
There are short guides for [Discord](../guides/phone-mic-for-discord) and [OBS](../guides/phone-mic-for-obs).

Set it as the default mic
-------------------------

To make every app use your phone by default:

* **Windows 11:** open SettingsSystemSound and select
  Micstream under **Input**.
* **Windows 10:** open the same page and pick Micstream under "Choose your input device".

The old sound settings also work on both: press `Win` + `R`, type `mmsys.cpl`, open the **Recording** tab, select Micstream and click **Set Default**.

USB or Wi-Fi?
-------------

USB works with Android phones. It has the lowest delay, often under 10 ms on high-end phones,
and it charges the phone. Use it for recording, streaming, or when you listen to yourself.

Wi-Fi lets you put the phone anywhere, like the middle of a meeting table. Its delay depends on
your network, so if the audio drops out, switch to USB.

Noise cancellation
------------------

The Windows client comes with [DeepFilterNet](https://github.com/Rikorose/DeepFilterNet), an AI noise-cancellation model. It is on by default and filters out keyboard clicks, fans,
and room noise before the audio reaches your apps.

To change it, open Micstream on your PC and go to **Settings**: switch **Noise Reduction** off, or pick a preset from **Quality** (removes
the least) to **Maximum** (removes the most). While streaming, you can also switch it
on or off from the phone app.

Processing runs on the computer, not on the phone, so it does not affect the phone's battery.

Troubleshooting
---------------

### Micstream isn't in the input device list

If Micstream is missing everywhere, the driver install didn't finish. Run the installer
again.

If it's there but an app still uses another mic, select Micstream in that app and restart
the app. Many apps only look for new devices when they start.

### The app can't find your PC over Wi-Fi

* Make sure the phone is on the same Wi-Fi as the PC, not on mobile data.
* Don't use a guest network. Guest networks often stop devices from seeing each other.
* If Windows Firewall asks about Micstream, allow it on private networks.
* On Android, a USB cable avoids all of these problems.

### Audio is choppy over Wi-Fi

This comes from the network, not the microphone. Distance, walls and other traffic on your
Wi-Fi all make it worse. Move closer to the router, or use a USB cable (Android).

Common questions
----------------

### Is it free?

Yes, with ads: each short ad unlocks an hour of free access. The hour counts down whether or not
you're streaming. A Pro subscription removes the ads and the time limit.

### Does it work with an iPhone?

Yes. The iPhone app is [on the App Store](https://apps.apple.com/app/id6791630989?pt=129174575&ct=guide-windows-faq&mt=8) and needs
iOS 18 or newer. It connects over Wi-Fi; USB works with Android phones only. See the [iPhone setup guide](../guides/use-iphone-as-pc-mic).

### macOS or Linux?

Linux works: see the [Linux setup guide](../guides/use-phone-as-mic-linux).
A macOS client is in development.

### What's the actual audio quality?

Windows receives the audio as 32-bit float at 48 kHz, in stereo on phones that support it. It is
never downsampled.

### Can I use the phone while it's streaming?

Yes. The app runs in the background.

Both downloads are free:

[Download Windows Client](/downloads/MicstreamSetup.exe) [Get Android App](https://play.google.com/store/apps/details?id=com.totech.phoneasmic&referrer=utm_source%3Dwebsite%26utm_medium%3Dweb%26utm_campaign%3Dwebsite%26utm_content%3Dguide-windows)

© 2026 Micstream. All rights reserved.

[Guides](../guides) • [Press Kit](/press) • [Privacy Policy](/privacy) • [Terms of Service](/terms)