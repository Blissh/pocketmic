# Source: https://micstream.io/guides/phone-mic-for-obs

Use Your Phone as a Mic in OBS Studio — Micstream

[Skip to main content](#main-content) 

[![Micstream logo](/logo/logo_light.svg)![Micstream logo](/logo/logo_dark.svg)Micstream](/) [FEATURES](../#how-it-works) [GUIDES](../guides) [DOWNLOAD](../download?src=header) [FAQ](../#faq)

[Home](../) [Guides](../guides)

Use your phone as a mic in OBS
==============================

Updated September 23, 2026 · 4 min read

In short

Add an **Audio Input Capture** source and choose "Micstream" as the device. After that,
it works like any other mic in OBS: mixer, filters, monitoring and sync offset.

Viewers accept blurry video much sooner than bad audio. A flagship phone's microphone beats most
cheap USB mics, and [Micstream](../) brings it into Windows as a normal microphone,
in 32-bit float at 48 kHz. OBS gives it its own channel, filters and monitoring.

First, set up Micstream: install the Windows client and the phone app, and connect the phone.
The [setup guide](../guides/use-phone-as-mic-windows) covers it. If you stream
from an Android phone, use USB. The delay is very low, and it can't break up mid-stream because someone
started a download on your Wi-Fi.

Add the source
--------------

1. 1

   ### Add an Audio Input Capture source

   In the **Sources** panel, click **+** and choose it from the list.
2. 2

   ### Name it

   Something clear, like "Phone mic".
3. 3

   ### Set the Device to Micstream

   Pick it from the dropdown and click OK. It appears in the Audio Mixer immediately.

To use the mic in every scene, set it globally instead: open SettingsAudioGlobal Audio Devices and choose Micstream for **Mic/Auxiliary Audio**.

Watch out

Use only one of the two. If the mic is both a global device and a scene source, OBS records it
twice and your voice sounds doubled.

Set your levels
---------------

Talk at your normal volume and watch the meter in the Audio Mixer. Aim for peaks around −12 to
−6 dB. That leaves room for loud moments and keeps your voice well above the background noise.
If you're too quiet, raise the volume on the phone (Micstream follows it) instead of boosting
the signal in OBS.

Stereo or mono?
---------------

Micstream sends stereo on supported phones. For a single voice, mono is usually better: it keeps
your voice in the center for every listener. Tick **Mono** for the source in **Advanced Audio Properties**. Keep stereo when you record a room or an instrument.

Filters
-------

Right-click the source and choose **Filters**. OBS has everything you need:

* **Noise Gate:** mutes the mic between sentences, so keyboard and room noise disappear.
  Set the close threshold just above your room's background noise, and the open threshold a few dB
  higher.
* **Compressor:** makes quiet and loud moments more even. For voice, start with a ratio
  of 3:1 or 4:1 and a threshold a few dB below your peaks.
* **Limiter:** goes last and catches sudden shouts.

The phone's audio arrives clean, so don't add filters just in case. Add them for problems you
can actually hear. Micstream already removes background noise on your PC, so you usually don't
need OBS's Noise Suppression filter as well.

Monitor what viewers hear
-------------------------

To hear your mic after the filters, open **Advanced Audio Properties** (the gear
icon in the mixer) and set **Audio Monitoring** to **Monitor and Output**. Use headphones for this: with speakers, the sound goes
straight back into the mic. When you're done, set it back to **Monitor Off**.

Fix audio/video sync
--------------------

If your lips and voice don't match, the video is usually late: webcams add more delay than a
fast mic. Record yourself clapping, measure the gap between sound and picture, and enter it as a
positive **Sync Offset** (in milliseconds) in Advanced Audio Properties. This delays
the mic to match the video. Over USB, the mic's own delay is so small that the offset is basically
your camera's delay. Over Wi-Fi it changes with the network, so check it again before anything important,
or use the cable.

Troubleshooting
---------------

### Micstream isn't in the Device dropdown

OBS only looks for devices when it starts, so restart OBS. If Micstream is also missing from
the Windows sound settings, run the Micstream installer again.

### The meter doesn't move

Check that the phone is connected in the app and that the channel isn't muted. If you added
the mic both globally and as a source, make sure you're watching the right channel.

### Crackles or dropouts mid-stream

Your Wi-Fi is busy, and other traffic gets in the way of your audio. USB (Android) isn't
affected by this.

### Your voice is doubled

OBS records it twice: once as a global device and once as a scene source. Remove one. If it
only happens while monitoring, the monitor sound is also recorded as desktop audio. Set
monitoring to Monitor Off before you go live.

Using Discord while you stream? See the [Discord guide](../guides/phone-mic-for-discord). Still choosing an app? See [how Micstream compares](../guides/micstream-vs-audiorelay-vs-wo-mic) to AudioRelay
and WO Mic.

Not installed yet? Both downloads are free:

[Download Windows Client](/downloads/MicstreamSetup.exe) [Get Android App](https://play.google.com/store/apps/details?id=com.totech.phoneasmic&referrer=utm_source%3Dwebsite%26utm_medium%3Dweb%26utm_campaign%3Dwebsite%26utm_content%3Dguide-obs)

© 2026 Micstream. All rights reserved.

[Guides](../guides) • [Press Kit](/press) • [Privacy Policy](/privacy) • [Terms of Service](/terms)