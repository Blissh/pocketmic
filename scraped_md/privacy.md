# Source: https://micstream.io/privacy

Privacy Policy - Micstream

[Skip to main content](#main-content) 

[![Micstream logo](/logo/logo_light.svg)![Micstream logo](/logo/logo_dark.svg)Micstream](/) [FEATURES](./#how-it-works) [GUIDES](./guides) [DOWNLOAD](./download?src=header) [FAQ](./#faq)

Privacy Policy
--------------

**Effective Date:** July 6th 2026  
**Last Updated:** November 1st 2026

### 1. Introduction

This Privacy Policy explains how StreamDev Technologies OÜ (“we,” “us,” or “our”) collects, uses, and protects your information when you use our audio streaming application and Windows and Linux desktop clients (together, the “Software”).

### 2. Data Controller Information

**StreamDev Technologies OÜ**  
 Vana-Kalamaja tn 20a-2  
 10414 Tallinn, Estonia  
 European Union  
 Email: [contact@streamdevtech.com](mailto:contact@streamdevtech.com)

### 3. Our Core Promise

**No audio content is ever transmitted to our servers or stored.** All audio streaming occurs directly between your devices on your local network. Audio data never leaves your devices and is never sent to, recorded by, or accessible to our infrastructure.

### 4. Information We Collect

Our Software does not require user accounts or registration. We collect limited, pseudonymous technical information for diagnostic and reliability purposes, and derive aggregated, de-identified usage statistics from our own server logs (see Section 4.5). These first-party diagnostics go only to our own servers and are never sent to third-party analytics platforms. The Android app additionally uses Google Firebase services (see Section 8.3); the iOS app does not, and the desktop clients use no third-party services at all.

#### 4.1 Device Information

* Device model and operating system details
* App version information
* Pseudonymous device identifiers (salted hash — we cannot identify you from this)

#### 4.2 Connection Data

* Network connection type (USB or Wi-Fi) and the phone’s network interface name (e.g. `wlan0`)
* Local network configuration needed to diagnose connection failures: local IP addresses of the phone and desktop, gateway, broadcast, DHCP server, and subnet mask (these are private LAN addresses, not your public internet address)
* Connection quality metrics (latency, packet loss)
* Connection timestamps and session duration
* Desktop operating system and protocol version

#### 4.3 Always-On Reliability Diagnostics

The Software sends pseudonymous, always-on diagnostics to our own servers. These are essential for detecting and fixing faults — including kernel-driver issues that can crash the user’s machine — and are used solely to diagnose problems and improve the software.

**Windows desktop diagnostics** include:

* A persistent, random `windows_client_id` (a random GUID stored locally, used only to track whether a specific Windows installation has recurring issues over time — it is not linked to your phone, your identity, or any payment information)
* Installer lifecycle events (install started, driver result — these carry **no** client identifier)
* Client startup, driver readiness status, update outcomes
* Stream quality summaries (packet counts, timing metrics — never audio content)
* Driver capture health counters (audio dropout counts, buffer resets, how often the microphone was opened and closed — never audio content)
* Local network and audio-endpoint context measured once when a stream starts: adapter type (Ethernet/Wi-Fi), link speed, Wi-Fi signal quality and power-saving mode, Windows network profile (public/private/domain), whether our firewall rule and a VPN adapter are present, and whether the Micstream microphone is the default device including its volume and mute state (never Wi-Fi network names, MAC addresses, or public IP addresses)
* Sanitized error codes (never raw error messages, file paths, or usernames)

**Linux desktop diagnostics** include:

* A persistent, random client ID (stored locally in `~/.local/state/micstream/client_id`, with the same properties and sole purpose as the Windows ID)
* Client startup events and stream quality summaries (packet counts, timing metrics — never audio content)

**Mobile diagnostics** include:

* Permission grant/deny results, paywall and purchase flow outcomes
* Ad consent (on iOS including the App Tracking Transparency choice) and load/show failures
* Recording-start failures
* Stream session summaries (duration, why the stream ended, and whether it ran over Wi-Fi or USB)
* Audio health counters for each stream: counts of audio glitches, the audio settings your device used (such as sample rate and channel count), failed network sends, and error codes. These are plain numbers, never audio.
* An optional post-stream rating (happy / neutral / unhappy) together with that session’s duration — sent only if you tap a smiley; dismissing the prompt sends nothing

**iOS crash and exit reports** come from MetricKit, Apple’s built-in diagnostics, carry the same pseudonymous device identifier as the other mobile diagnostics, and include:

* On the launch after a crash: the crash type as iOS reports it (signal, exception type and code, and the termination reason as its code only), the app build and the identifier of our app binary, the operating-system version and device model, and, when iOS names the crashed thread, its innermost ten frames as binary names and offsets, which we resolve against our own build symbols — never exception messages or user content
* At most once a day: counts of how the app exited during the previous day, split by whether it was in the foreground or the background and by reason (such as a normal exit, the memory limit, or a watchdog timeout), together with the start and end of the period they cover and the app build they measured

All diagnostics are:

* **Pseudonymous**: random identifiers scoped to a single domain (each desktop client’s id stays in that platform’s events; the mobile device id stays in mobile events) — we never build a cross-device identity graph
* **Minimal**: a closed, server-enforced set of known event types and property names, with a hard size cap on every event; no free-form text, no file paths, no usernames, no machine names
* **Self-hosted**: stored only on our own EU infrastructure, never sent to third-party analytics or data platforms

#### 4.4 What We Do NOT Collect

Our own diagnostics and server logs never include:

* Audio content (our core promise)
* Raw usernames, machine names, or hardware serial numbers
* MAC addresses or file paths
* Raw exception messages, or stack traces beyond the iOS crash frames described in Section 4.3
* Any data that could directly identify you as an individual

(Crash reports collected by Firebase Crashlytics in the Android app are separate and are described in Section 8.3.)

#### 4.5 Server Logs and Aggregated Statistics

Like virtually every web service, our servers keep operational access logs (IP address, requested URL, timestamp, user agent, and coarse GeoIP-derived location such as country). These logs are used for security, fault diagnosis, and operations, and are deleted after 30 days.

Before deletion, we derive **aggregated, de-identified daily statistics** from these logs — for example, how many distinct visitors downloaded a desktop client per day, per country, or how many installations checked for updates. These aggregates contain only dates, counts, country codes, and software versions; they contain no IP addresses, no user agents, no per-event timestamps, and small counts are suppressed so individuals cannot be singled out. The aggregates are retained indefinitely.

### 5. Legal Basis for Processing

We process your data based on the following legal grounds under GDPR:

* **Legitimate Interests** (Article 6(1)(f)): We collect always-on reliability diagnostics to maintain and improve our software, detect and fix kernel-driver faults, and ensure stability; we keep short-lived server access logs for security and operations; we derive aggregated, de-identified usage statistics from those logs to understand adoption of our software; and, in the Android app, we measure app usage (Firebase Analytics, auto-collected events only), collect crash and performance reports, and deliver push notifications via the Firebase services described in Section 8.3. A documented Legitimate Interest Assessment (LIA) covering the always-on first-party diagnostics is available upon request.
* **Contract** (Article 6(1)(b)): Processing needed to provide purchased subscriptions and validate in-app purchases (RevenueCat, Section 8.1).
* **Consent** (Article 6(1)(a)): For ad personalization settings managed by the mobile app’s consent framework (UMP/TCF), which governs ad-related data processing only and is separate from first-party diagnostics.

### 6. Purpose of Data Collection

We use the collected information exclusively for:

* Detecting and fixing kernel-driver faults and application crashes
* Monitoring installer success rates and diagnosing installation failures
* Understanding stream quality and connection reliability across software versions
* Fixing bugs and improving software stability
* Providing technical support when requested
* Measuring adoption of our software (downloads, active installations, website visits) through aggregated, de-identified daily statistics

### 7. Data Storage and Security

#### 7.1 Location

All data we collect ourselves (Section 4) is stored on secure servers operated by Hetzner within the European Union and never leaves the EU. Data processed by the third-party services in Section 8 is handled by those providers under their own privacy policies and may be processed outside the EU.

#### 7.2 Security Measures

We implement appropriate technical and organizational measures to protect your data, including:

* Encrypted data transmission
* Secure server infrastructure
* Limited access controls
* Regular security assessments

#### 7.3 Data Retention

* **Reliability diagnostics**: retained for as long as needed to diagnose faults and track reliability across software versions
* **Server access logs**: deleted after 30 days
* **Aggregated, de-identified statistics** (no IP addresses or other identifiers): retained indefinitely
* **User support bundles**: retained per explicit policy, deleted upon resolution

You may request deletion of your data at any time (see Section 9).

### 8. Third-Party Services

#### 8.1 RevenueCat

Used for subscription management and in-app purchase processing. RevenueCat may collect:

* Purchase transaction data
* Subscription status and history
* Device identifiers for purchase validation

RevenueCat is provided by RevenueCat, Inc. For more information, see [RevenueCat’s privacy policy](https://www.revenuecat.com/privacy/).

#### 8.2 Google AdMob

Used for displaying ads in the free tier of the mobile app. Ad personalization is governed by the mobile app’s consent framework (UMP/TCF) and is separate from our first-party diagnostics. On iOS, Google may use your device’s advertising identifier to personalize ads only if you allow it in Apple’s App Tracking Transparency prompt, which the app shows before an ad; without that permission, ads stay non-personalized. You can change this choice at any time in iOS Settings → Privacy & Security → Tracking. In the EEA, the UK and Switzerland, ads additionally follow the choices you made in the app’s consent form, and the tracking prompt is shown only if you consented to personalized ads there. For more information, see [Google’s privacy policy](https://policies.google.com/privacy).

#### 8.3 Google Firebase (Android app only)

The Android app uses Google Firebase services: Firebase Analytics (automatically collected app-usage events only — we define no custom events), Firebase Crashlytics (crash reports, which include device information and stack traces), Firebase Performance Monitoring, and Firebase Cloud Messaging (push notifications). To deliver notifications, the app registers a random push token issued by FCM and shares it, together with the notification-permission state, with our servers, where it is stored alongside the pseudonymous device record until it is replaced by a newer token or you request deletion. The Firebase services are provided by Google and process data as described in [Google’s privacy policy](https://policies.google.com/privacy). Our first-party reliability diagnostics (Section 4.3) are separate from Firebase and are never sent to it. The iOS app does not use Firebase, and the desktop clients do not use Firebase or any other third-party service.

### 9. Your Rights

Under GDPR, you have the following rights:

* **Access**: Request a copy of your personal data
* **Rectification**: Request correction of inaccurate data
* **Erasure**: Request deletion of your data (“right to be forgotten”)
* **Restriction**: Request limited processing of your data
* **Portability**: Receive your data in a portable format
* **Objection**: Object to certain types of processing
* **Automated Decision-Making**: We do not use automated decision-making

#### Important Note About Exercising Your Rights

Our Software does not use user accounts. To exercise your rights, please contact us at [contact@streamdevtech.com](mailto:contact@streamdevtech.com) and provide:

* Your Windows client ID (found in `%LocalAppData%\Micstream\client_id`) or your Linux client ID (found in `~/.local/state/micstream/client_id`). The mobile app does not display its device ID; for the mobile app, tell us the platform (Android or iOS), the device model and the approximate install date instead
* Approximate dates when you used the Software

Because our identifiers are pseudonymous and domain-scoped, we may not always be able to link data across platforms. In such cases, your data is effectively anonymous and not personally identifiable.

### 10. Children’s Privacy

Our Software is not intended for children under 16. We do not knowingly collect personal information from children under 16. If you believe we have collected such information, please contact us immediately.

### 11. Data Breach Notification

In the event of a data breach that is likely to result in risk to your rights and freedoms, we will notify you and the relevant supervisory authority within 72 hours of becoming aware of the breach.

### 12. Changes to This Policy

We may update this Privacy Policy from time to time. We will notify you of any material changes by:

* Posting the new Privacy Policy on our website
* Updating the “Last Updated” date

### 13. Contact Us

If you have any questions about this Privacy Policy or our data practices, please contact us at:

Email: [contact@streamdevtech.com](mailto:contact@streamdevtech.com) Address: StreamDev Technologies OÜ, Vana-Kalamaja tn 20a-2, 10414 Tallinn, Estonia

### 14. Supervisory Authority

You have the right to lodge a complaint with a supervisory authority. As we are based in Estonia, the relevant authority is:

**Estonian Data Protection Inspectorate (Andmekaitse Inspektsioon)**  
 Website: <https://www.aki.ee/en>  
 Email: [info@aki.ee](mailto:info@aki.ee)

© 2026 Micstream. All rights reserved.

[Guides](./guides) • [Press Kit](/press) • [Privacy Policy](/privacy) • [Terms of Service](/terms)