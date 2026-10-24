# Caught: phone web prototype

A web version of [POC 5](../../plans/poc-05-caught.md) that you can try lying down with your phone on your belly. It's for getting a feel for the practice and checking whether phone breath sensing works, not the iOS and Watch app the plan describes.

## Files

| File | What it is |
| --- | --- |
| `index.html` | The page as published to Claude (page content only; the publisher adds the document wrapper) |
| `standalone.html` | The same page wrapped as a complete document, for hosting anywhere. Rebuild with `python3 build_standalone.py` |
| `build_standalone.py` | Writes `standalone.html` from `index.html` |

## How it works

- **Breath sensing.** It reads the phone's tilt from `devicemotion` (gravity vector), resampled to 25 Hz, with a 5-sample median filter and a 0.1–0.5 Hz biquad bandpass. A rolling principal axis follows whichever way the phone tilts as you breathe. Adaptive hysteresis then marks the end of each breath out.
  - Touches shake the phone, so the signal is held still for 300 ms after each touch.
  - A 30-second calibration checks for breaths, a steady rhythm and a clear signal before the sit can start.
- **Three modes:**
  - **Breath sensing** uses the real phone sensor.
  - **Tap only** guesses missed breaths from gaps in your tapping, so breaths-to-catch is labelled as estimated.
  - **Demo breathing** feeds a synthetic breath so the controls work on a laptop. Demo sits stay out of the trend chart.
- **The sit.** You keep your eyes closed on a near-black screen.
  - Tap once per breath, double-tap on ten, and press and hold when you notice you've wandered.
  - Chimes come at Poisson intervals: a mean of 3 minutes, none in the first minute, 8 seconds to answer.
  - Sits are 5, 10 or 15 minutes, with a hard cap of 20.
- **Scoring follows the transcript.**
  - Count slips come from ten or more taps without a ten, a ten at the wrong count, or two breaths with no tap.
  - A self-catch is a *return*, and its breaths-to-catch is the number of sensor breaths since the first slip.
  - The only setback is answering a chime with "away" when you haven't caught yourself in the previous 10 seconds.
  - There are no timing grades anywhere. Tap-minus-breath offsets go only into the JSON export, labelled as likely.
- **Attention test.** A SART (digits 1–9, don't tap on 3) in a quick 90-trial version or the standard 225. Each digit shows for 250 ms, then a 900 ms mask. It reports taps on 3, missed digits and response time.
- **History.** Sits and tests are stored in the browser's `localStorage`, with two charts on a shared date axis. Example data shows until you have your own. Everything exports as JSON.
- **Check-in.** After each sit it asks how it felt, offers an easy stop, and points to support if it felt bad.

## Using breath sensing

Pages shown inside the Claude viewer don't get motion-sensor access, so the published artifact runs in tap-only or demo mode. To use the phone sensor, serve `standalone.html` from its own HTTPS address. For example:

- **GitHub Pages:** turn on Pages for this repo, then open `/prototypes/caught/standalone.html` on your phone.
- **Any static host:** upload the file and open it on your phone.

iPhone asks for motion access the first time you tap "Use breath sensing". Turn off silent mode, or the chimes won't play. Keep the page in front: if the screen locks, the browser pauses the sit. The page asks for a screen wake lock, and most current browsers grant it.

## Debug mode

Add `?debug=1`, or `#debug` inside the viewer, to get:

- an extra 1-minute length;
- an 8-second calibration;
- chimes every 20 seconds on average, starting after 5 seconds;
- a short SART;
- a live readout during the sit.

## What it leaves out (and the plan keeps)

- The Watch: taps go on the phone screen.
- The Polar H10 reference and the respiration validation study (plan milestones 1–2).
- The Bayesian change-point jitter model (Test A in the plan).
- The 300-person cohort and the transfer test against an independent SART (Test B).

Wellness practice, not a medical device. All processing stays on the device.
