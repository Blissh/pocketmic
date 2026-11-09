# Source: https://micstream.io/guides/use-iphone-as-pc-mic

Use Your iPhone as a PC Microphone on Windows — Micstream

[Skip to main content](#main-content) 

[![Micstream logo](/logo/logo_light.svg)![Micstream logo](/logo/logo_dark.svg)Micstream](/) [FEATURES](../#how-it-works) [GUIDES](../guides) [DOWNLOAD](../download?src=header) [FAQ](../#faq)

[Home](../) [Guides](../guides)

Use your iPhone as a PC microphone
==================================

Updated September 23, 2026 · 4 min read

In short

* Install the Micstream client on your PC
* Install Micstream on your iPhone from the App Store
* Put both on the same Wi-Fi, open the app, tap your PC
* In Windows, select the input device called "Micstream"

No cable, no Bluetooth pairing, and nothing else to install on the iPhone.

Your iPhone has a better microphone than most cheap USB mics, and a much better one than your
laptop. But Windows can't use an iPhone as a microphone on its own, and iOS has no setting for
it.

[Micstream](../) fixes this. The Windows client installs a small audio driver
that adds a microphone called "Micstream", and the iPhone app sends its audio there over Wi-Fi. To
Windows and apps like Zoom or OBS, it looks like a real microphone. The audio arrives as 32-bit float
at 48 kHz, straight from your iPhone to your PC over your local network. It never passes through a
server.

You need three things:

* An **iPhone** running iOS 18 or newer
* A PC with **Windows 10 or 11**, or Linux (see below)
* Both on the **same Wi-Fi network**

Setup
-----

1. 1

   ### Install the Windows client

   Download [the installer](https://micstream.io/downloads/MicstreamSetup.exe) and run it. It installs the desktop client and the audio driver, which creates the "Micstream"
   microphone. Your other audio devices stay as they are.
2. 2

   ### Install the iPhone app

   Get it [on the App Store](https://apps.apple.com/app/id6791630989?pt=129174575&ct=guide-iphone&mt=8). You don't need an
   account.
3. 3

   ### Allow the two permissions

   When you first open the app, iOS asks for **Local Network** access. Allow it:
   the app needs it to find your PC. When you first connect, iOS asks for **microphone** access. Allow that too. If you said no by mistake, you can
   change both in SettingsAppsMicstream.
4. 4

   ### Connect

   Open the app while the Windows client is running. Your PC appears within a few seconds. Tap
   it to start streaming. You don't need to type an address or a code.
5. 5

   ### Pick Micstream as the input

   Open the microphone settings of the app you're using and select **Micstream**.
   To use it everywhere, open SettingsSystemSound in Windows and select it under **Input**.

Discord and OBS ignore the Windows default and use their own setting, so select Micstream there.
There are short guides for [Discord](../guides/phone-mic-for-discord) and [OBS](../guides/phone-mic-for-obs).

Lock the screen, keep streaming
-------------------------------

Once streaming, you can lock your iPhone or switch to another app. The audio keeps going, and
the orange microphone dot stays at the top of the screen. It uses little battery, so a long
meeting is fine.

A phone call, Siri or an alarm stops the stream, because iOS gives them the microphone.
Micstream doesn't take it back on its own, so the microphone never turns on without you knowing.
To continue, open the app and tap your PC again.

AirPods and other Bluetooth mics
--------------------------------

The app uses the iPhone's built-in microphone by default. That's usually best, because most
Bluetooth headsets only send phone-call quality. To use your headset anyway, turn it on in the
app under SettingsMicrophone. AirPods with the H2 chip on iOS 26 or
newer record in high quality. Other headsets record in phone-call quality.

Tip

For the lowest delay, connect the iPhone and the PC to the same router. A mesh node or range
extender in between adds delay. If you can choose, use 5 GHz Wi-Fi.

Troubleshooting
---------------

### The app can't find your PC

Most often, Local Network access is off. If the app shows an orange banner about it, tap the
banner. Or go to SettingsPrivacy & SecurityLocal Network and turn
on Micstream.

If that's not it:

* Make sure the iPhone is on the same Wi-Fi as the PC, not on mobile data.
* Don't use a guest network. Guest networks often stop devices from seeing each other.
* If Windows Firewall asks about Micstream, allow it on private networks.

### Micstream isn't in the input device list

If Micstream is missing everywhere, the driver install didn't finish. Run the installer
again.

If it's there but an app still uses another mic, select Micstream in that app and restart
the app. Many apps only look for new devices when they start.

### The stream stopped on its own

A call, Siri or an alarm took the microphone (see above), or the Wi-Fi dropped. Open the app
and connect again. If it keeps happening in the same place, the Wi-Fi is weak there. Move
closer to the router.

### Audio is choppy

This comes from the network, not the microphone. Distance, walls and other traffic make it
worse, like a download on the PC or a video on the iPhone. Move closer to the router, or
pause the other traffic.

Common questions
----------------

### Can I connect over USB or Lightning?

No. On iPhone, Micstream works over Wi-Fi only. On a good network, the delay is low enough for
calls, streaming and recording.

### Is it free?

Yes, with ads: each short ad unlocks an hour of free access. The hour counts down whether or not
you're streaming. A Pro subscription removes the ads and the time limit.

### Does it work on iPad?

The app is made for iPhone. It also installs on iPad, where it runs in iPhone mode.

### Does it work with a Mac or Linux PC?

Linux works: see the [Linux setup guide](../guides/use-phone-as-mic-linux).
A macOS client is in development.

### What's the actual audio quality?

Windows receives the audio as 32-bit float at 48 kHz. It is never downsampled.

Both downloads are free:

[Download Windows Client](/downloads/MicstreamSetup.exe) [Get iPhone App](https://apps.apple.com/app/id6791630989?pt=129174575&ct=guide-iphone&mt=8)

© 2026 Micstream. All rights reserved.

[Guides](../guides) • [Press Kit](/press) • [Privacy Policy](/privacy) • [Terms of Service](/terms)